// src/build.rs
fn main() {
    //cargo build --target wasm32-unknown-emscripten
    //cp target/wasm32-unknown-emscripten/debug/cpp2rust.js build

    //rustc -L . -l C:\Users\perda\RustroverProjects\c++2rust\src/main.a --target wasm32-unknown-emscripten  src/main.rs -o out.wasm

    cc::Build::new()
        .include("src")
        .file("src/main.cpp")
        .compile("main")
}
