// https://rustwasm.github.io/wasm-bindgen/reference/types/number-slices.html

use wasm_bindgen::prelude::*;

/// receive a typed array from js
/// 
/// # Examples
/// ```js
/// const a = new Float32Array([1,2,3]);
/// const count = numberSlice(a); // count = 3
/// ```
#[wasm_bindgen(js_name=numberSlice)]
pub fn take_number_slice(x: &[f32]) -> usize {
    x.len()
}
/// receive a typed array from js, and change the value of index to val
/// 
/// # Examples
/// ```js
/// const a = new Float32Array([1,2,3]);
/// const val = updateNumSlice(a, 0, 4); // val = 4
/// ```
#[wasm_bindgen(js_name=updateNumSlice)]
pub fn update_number_slice(x: &mut[f32], index: usize, val: f32) -> Result<f32, JsValue> {
    if x.len() <= index || x.len() == 0 {
        Err(JsValue::FALSE)
    } else {
        x[index] = val;
        Ok(val)
    }
}