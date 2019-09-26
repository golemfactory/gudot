# gudot
It computes linear regression using FHE [gMorph] library. 
This is a showcase for feasibility of gMorph/FHE on Golem.
The whole software is experimental and should be treated as demonstration.
It is included in Golem's workshop on deVcon 5.

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
The project involves executing several steps in sequence: some steps are done on the host OS (so your machine),
whereas the main step is performed using gWasm (run on `gwasm-runner`). The sequence can be
summarised as follows

```
$ target/release/gudot generate
Writing input.json

$ target/release/gudot encrypt
Reading input.json
Writing enc_input.json
Writing keys.json

$ gwasm-runner target/release/gudot.wasm -- --subtasks=N
work dir: .../split
Reading enc_input.json
Writing enc_output.json

$ target/release/gudot decrypt
Reading keys.json
Reading enc_output.json
Writing output.json

$ target/release/gudot regress
Reading output.json
Reading input.json
Fitted model: y = -2.6898656093724984x + 93773.13902413758
Writing regress.json

$ target/release/gudot plot
Reading input.json
Reading regress.json
Writing plot.png
```

## Components
* `generate` --- creates `input.json` containing sample data in the format `[[x1,...,xn][y1,...,yn]]`
  (this step is optional - you can provide your own `input.json`);
* `encrypt` --- generates a secret encryption key (`keys.json`), and encrypts `input.json` yielding `enc_input.json`;
* `gudot.wasm` --- the main gWasm binary, meant to be run with `gwasm-runner`; it splits the encrypted data;
  sends parts to providers and combines partial results returned by the providers
* `decrypt` --- decrypts the received results, producing `output.json`   (reads `keys.json` and `enc_output.json`);
* `regress` --- calculates parameters `(slope, intercept)` of the fitted linear model, producing `regress.json`
* `plot` --- plots the regression line along with the input points, producing `plot.png`.
