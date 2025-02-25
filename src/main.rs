mod parser;
mod syntax_kind;
mod syntax_node;

use syntax_kind::SyntaxKind;
use syntax_node::{print_full_tree, SyntaxNode};

fn main() {
    let green = parser::parse_text("SELECT * WHERE  {?sub ?a  }");
    let red: SyntaxNode = SyntaxNode::new_root(green);

    let childs: Vec<SyntaxNode> = red.children().collect();
    println!("{:?}", childs[1].children().collect::<Vec<SyntaxNode>>());
    println!("{}", print_full_tree(&red, 0));
    println!("{:?}", red.token_at_offset(25.into()));
}
