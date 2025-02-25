use crate::SyntaxKind::{self, *};

pub type SyntaxNode = rowan::SyntaxNode<Sparql>;
pub type SyntaxToken = rowan::SyntaxToken<Sparql>;
pub type SyntaxElement = rowan::SyntaxElement<Sparql>;
pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<Sparql>;
pub type SyntaxElementChildren = rowan::SyntaxElementChildren<Sparql>;
pub type PreorderWithTokens = rowan::api::PreorderWithTokens<Sparql>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Sparql {}
impl rowan::Language for Sparql {
    type Kind = SyntaxKind;
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= QueryUnit as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

pub fn print_full_tree(syntax_node: &SyntaxNode, indent: usize) -> std::string::String {
    let mut s = std::string::String::new();
    s += &format!("{}{:?}\n", "    ".repeat(indent), syntax_node,);
    syntax_node
        .children_with_tokens()
        .for_each(|child| match child {
            rowan::NodeOrToken::Node(node) => s += &print_full_tree(&node, indent + 1),
            rowan::NodeOrToken::Token(token) => {
                s += &format!("{}{:?}\n", "    ".repeat(indent), token);
            }
        });
    return s;
}
