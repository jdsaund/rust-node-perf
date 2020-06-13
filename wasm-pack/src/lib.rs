use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

#[wasm_bindgen]
pub fn sumBuffer(raw: &[u8]) -> f64 {
    let mut sum: f64 = 0.0;

    for val in raw {
        sum += *val as f64;
    }

    sum
}
