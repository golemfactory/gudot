# gudot
Linear regression using FHE [gMorph] library

[gMorph]: https://github.com/golemfactory/gmorph

## Requirements
* emsdk - installation instructions can be found
  [here](https://emscripten.org/docs/getting_started/downloads.html)
* gwasm-runner - you can either build it from source as described
  [here](https://github.com/golemfactory/gwasm-runner), or download
  a precompiled binary from [here](https://github.com/golemfactory/gwasm-runner/releases)
* freetype library - on Ubuntu install package `libfreetype6-dev`
## Building
Assuming you've got the prerequisites satisfied, in order to build the project, simply run

```
cargo build --release
```

## Running
The project involves executing three steps in sequence: 2 steps are done on the host OS (so your machine),
whereas the final step is performed using gWasm (run on `gwasm-runner`). The sequence can be
summarised as follows

```
cargo run --release -- generate
cargo run --release -- encrypt
gwasm-runner target/release/gudot.wasm -- --subtasks=N
cargo run --release -- decrypt
cargo run --release -- plot
```

## Components
* `generate` --- creates `input.json` containing sample data in the format `[[x1,...,xn][y1,...,yn]]`
  (this step is optional - you can provide your own `input.json`);
* `encrypt` --- generates a secret encryption key (`keys.json`), and encrypts `input.json` yielding `enc_input.json`;
* `gudot.wasm` --- the main gWasm binary, meant to be run with `gwasm-runner`; it splits the encrypted data;
  sends parts to providers and combines partial results returned by the providers
* `decrypt` --- decrypts the received results, producing `output.json`   (reads `keys.json` and `enc_output.json`);
* `plot` --- plots the regression line along with the input points, producing `plot.png`.
