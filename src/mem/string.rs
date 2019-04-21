use wasm_bindgen::prelude::*;

/// pass string demo
/// 
/// # Examples
/// ```js
/// const a = 'Jeff';
/// const res = passString(a); // res = 'Hello, passed from Rust: Jeff'
/// ```
#[wasm_bindgen(js_name=passString)]
pub fn pass_string(a: &str) -> String {
    format!("Hello, passed from Rust: {}", a)
}