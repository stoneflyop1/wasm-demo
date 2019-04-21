// https://rustwasm.github.io/wasm-bindgen/reference/arbitrary-data-with-serde.html
use wasm_bindgen::prelude::*;

use std::collections::HashMap;

use crate::imports::log;

/// struct used to convert to and from JsValue
#[derive(Serialize, Deserialize)]
pub struct StructExample {
    pub name: String,
    pub props: HashMap<String, u32>,
}

/// send a rust struct type as js obj to js
#[wasm_bindgen(js_name = sendRustStruct)]
pub fn send_rust_struct_to_js() -> JsValue {
    let mut map = HashMap::new();
    map.insert("field".to_owned(), 1);
    let exam = StructExample {
        name: "Jeff".to_owned(),
        props: map,
    };
    JsValue::from_serde(&exam).unwrap()
}
/// receiving from js obj and deserialize as a rust struct
#[wasm_bindgen(js_name = receiveJs)]
pub fn receive_from_js(val: &JsValue) {
    let exam: StructExample = val.into_serde().unwrap();
    log(&format!("FromJS: {}, {:?}", exam.name, exam.props))
}