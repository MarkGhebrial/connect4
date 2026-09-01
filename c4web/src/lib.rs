use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// The `alert` browser API
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}", name.to_uppercase()));
}