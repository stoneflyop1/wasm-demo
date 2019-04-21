// https://rustwasm.github.io/wasm-bindgen/reference/types/number-slices.html

use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name=numberSlice)]
pub fn take_number_slice(x: &[f32]) -> usize {
    x.len()
}

#[wasm_bindgen(js_name=updateNumSlice)]
pub fn update_number_slice(x: &mut[f32], index: usize, val: f32) -> Result<f32, JsValue> {
    if x.len() <= index || x.len() == 0 {
        Err(JsValue::FALSE)
    } else {
        x[index] = val;
        Ok(val)
    }
}