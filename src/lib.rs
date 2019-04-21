#[macro_use]
extern crate serde_derive;

pub mod mem;
pub mod imports;

//// if expose mod js, wasm-pack test will fail.
//pub mod js;

#[cfg(test)]
pub mod tests {
    use wasm_bindgen_test::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[wasm_bindgen_test]
    fn pass() {
        assert_eq!(1,1);
    }
}
