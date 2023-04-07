# Web Assembly plugins

This is a little proof of concept of using WebAssembly to distribute plugins.
Currently it doesn't really provide true language independence because only plugins generated from Rust code
via [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/) will work, but it's a start.

The plugins allow to dynamically add greeting messages in several languages.

Plugins provide two functions:
- `language` returns the language as a string.
- `greet` takes a name and returns a greeting in the language.

Using strings forces us to deal with more involved operations than just using the WebAssembly basic numeric types.

[english-rs](english-rs/src/lib.rs) is a Rust plugin returning a greeting in English.

[i18n-greeter](i18n-greeter/src/main.rs) is the Rust code that actually loads the WASM files and run the greeting using a name given
as argument. It currently expects that the function are called using the wasm-bindgen conventions.
