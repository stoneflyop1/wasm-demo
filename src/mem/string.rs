use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name=passString)]
pub fn pass_string(a: &str) -> String {
    format!("Hello, passed from Rust: {}", a)
}