use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

#[wasm_bindgen]
pub fn sumBuffer(raw: &[u32]) -> f64 {
    let mut sum: u32 = 0;

    for val in raw {
        sum += *val;
    }

    sum as f64
}
