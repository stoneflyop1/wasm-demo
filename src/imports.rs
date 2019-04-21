use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// log from js console
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}