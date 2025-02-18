mod parser;

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub fn parse(input: &str) -> JsValue {
    let tree = parser::Parser::parse(input);
    serde_wasm_bindgen::to_value(&tree).unwrap()
}
