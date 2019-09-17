# gudot
Linear regression using FHE gMorph library

## Requirements
* emsdk
* Rust stable
* gwasm-runner

## Building
```
cargo build --bin=generate
cargo rustc --release --target wasm32-unknown-emscripten --bin gudot -- -C link-args="-s ALLOW_MEMORY_GROWTH=1"
```

## Running
```
cargo run --bin=generate
cargo run --bin=encrypt
gwasm-runner target/wasm32-unknown-emscripten/release/gudot.wasm -- --subtasks=N
```

## Components

* `generate` --- creates `input.json` containing sample data in the format `[[x1,...,xn][y1,...,yn]]`
(this step is optional - you can provide your own `input.json`)
* `encrypt` --- generates a secret encryption key (`keys.json`) and encrypts `input.json` yielding `data.json`
* `gudot` --- the main binary, meant to be run with `gwasm-runner` - splits the encrypted data, sends parts to providers, decrypts and combines partial results returned by the providers (reads `keys.json` and `data.json`).
