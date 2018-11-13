#!/bin/bash
set -x
mkdir -p build
#cargo build --target wasm32-unknown-unknown
#wasm-bindgen target/wasm32-unknown-unknown/debug/done.wasm --out-dir .
cargo watch \
    --exec "build --target wasm32-unknown-unknown" \
    --shell "wasm-bindgen ../../target/wasm32-unknown-unknown/debug/dommer_example_simple.wasm --out-dir ./build" \
    --watch src \
    --watch ../../src \
    --watch Cargo.toml \
    --watch ../../Cargo.toml
