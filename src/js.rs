// https://rustwasm.github.io/wasm-bindgen/examples/import-js.html

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/wasmjs.js")]
extern "C" {
    fn hello() -> String;

    type DemoJs;

    #[wasm_bindgen(constructor)]
    fn new() -> DemoJs;

    #[wasm_bindgen(method, getter)]
    fn id(this: &DemoJs) -> u32;

    #[wasm_bindgen(method, setter)]
    fn set_id(this: &DemoJs, n: u32) -> DemoJs;

    #[wasm_bindgen(method)]
    fn render(this: &DemoJs) -> String;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
/// log from rust, use console.log from js, output will be prefixed with 'Test Import From JS'
fn rust_log(mut s: String) {
    s.insert_str(0, "Test Import From JS");
    log(&s);
}


#[wasm_bindgen(js_name = run)]
pub fn start() {
    let hello = (format!("Hello, {}!", hello())).to_owned();
    rust_log(hello);

    let x = DemoJs::new();
    assert_eq!(x.id(), 42);
    x.set_id(10);
    rust_log(x.render());
}