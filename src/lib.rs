mod parser;
mod syntax_kind;
mod syntax_node;

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use syntax_kind::SyntaxKind;

#[wasm_bindgen]
pub fn parse(input: &str) -> JsValue {
    let tree = parser::Parser::parse(input);
    todo!()
}
