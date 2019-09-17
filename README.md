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
cargo run --bin=client1
gwasm-runner target/wasm32-unknown-emscripten/release/gudot.wasm -- --subtasks=N 
```
