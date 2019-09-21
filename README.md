# gudot 
Linear regression using FHE [gMorph] library

[gMorph]: https://github.com/golemfactory/gmorph

## Requirements
* emsdk - installation instructions can be found
  [here](https://emscripten.org/docs/getting_started/downloads.html)
* gwasm-runner - you can either build it from source as described
  [here](https://github.com/golemfactory/gwasm-runner), or download
  a precompiled binary from [here](https://github.com/golemfactory/gwasm-runner/releases)

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
cargo run -- generate
cargo run -- encrypt
gwasm-runner target/release/gudot.wasm -- --subtasks=N
```

## Components
* `generate` --- creates `input.json` containing sample data in the format `[[x1,...,xn][y1,...,yn]]`
  (this step is optional - you can provide your own `input.json`)
* `encrypt` --- generates a secret encryption key (`keys.json`), and encrypts `input.json` yielding `data.json`
* `gudot.wasm` --- the main gWasm binary, meant to be run with `gwasm-runner`; it splits the encrypted data,
  sends parts to providers, decrypts and combines partial results returned by the providers
  (reads `keys.json` and `data.json`).
