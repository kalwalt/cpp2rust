#!/usr/bin/env bash

cargo build --target wasm32-unknown-emscripten
cp target/wasm32-unknown-emscripten/debug/cpp2rust.js build
cp target/wasm32-unknown-emscripten/debug/cpp2rust.wasm build