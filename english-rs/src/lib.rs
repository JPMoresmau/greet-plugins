mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// The language we greet in.
#[wasm_bindgen]
pub fn language() -> String {
    String::from("English")
}

/// Greet the given name.
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}
