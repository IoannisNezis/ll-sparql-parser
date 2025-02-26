mod parser;
mod syntax_kind;
mod syntax_node;

use js_sys::{Array, Object, Reflect};
use syntax_node::SyntaxNode;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use syntax_kind::SyntaxKind;

#[wasm_bindgen]
pub fn get_parse_tree(input: &str) -> JsValue {
    let root = SyntaxNode::new_root(parser::parse_text(input));
    build_js_tree(&root)
}

fn build_js_tree(node: &SyntaxNode) -> JsValue {
    let obj = Object::new();
    Reflect::set(
        &obj,
        &JsValue::from_str("kind"),
        &JsValue::from_str(&format!("{:?}", node.kind())),
    )
    .unwrap();
    Reflect::set(&obj, &JsValue::from_str("type"), &JsValue::from_str("node")).unwrap();
    let children = Array::from_iter(node.children_with_tokens().filter_map(|child| match child {
        rowan::NodeOrToken::Node(node) => Some(build_js_tree(&node)),
        rowan::NodeOrToken::Token(token) if token.kind() != SyntaxKind::WHITESPACE => {
            let token_obj = Object::new();
            Reflect::set(
                &token_obj,
                &JsValue::from_str("kind"),
                &JsValue::from_str(&format!("{:?}", token.kind())),
            )
            .unwrap();
            Reflect::set(
                &token_obj,
                &JsValue::from_str("type"),
                &JsValue::from_str("token"),
            )
            .unwrap();
            Reflect::set(
                &token_obj,
                &JsValue::from_str("text"),
                &JsValue::from_str(token.text()),
            )
            .unwrap();
            Some(token_obj.into())
        }
        _ => None,
    }));
    Reflect::set(&obj, &JsValue::from_str("children"), &children.into()).unwrap();
    obj.into()
}
