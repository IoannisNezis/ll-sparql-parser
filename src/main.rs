mod parser;
mod syntax_kind;
mod syntax_node;

use syntax_kind::SyntaxKind;
use syntax_node::SyntaxNode;

fn main() {
    let green = parser::Parser::parse("SELECT*where{?a}");
    let red: SyntaxNode = SyntaxNode::new_root(green);
    println!("{:?}", red.token_at_offset(15.into()));
}
