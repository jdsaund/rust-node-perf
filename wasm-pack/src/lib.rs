use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: f64, b: f64) -> f64 {
    a + b
}
