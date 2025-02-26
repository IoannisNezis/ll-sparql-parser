mod parser;
mod syntax_kind;
mod syntax_node;

use syntax_kind::SyntaxKind;
use syntax_node::{print_full_tree, SyntaxNode};

fn main() {
    let green = parser::parse_text("SELECT * {}");
    let red: SyntaxNode = SyntaxNode::new_root(green);

    let childs: Vec<SyntaxNode> = red.children().collect();
    println!("{}", print_full_tree(&red, 0));
}
