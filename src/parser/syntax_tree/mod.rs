mod kinds;

use std::fmt::Display;

pub use kinds::{TokenKind, TreeKind};
use logos::Span;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Tree {
    pub kind: TreeKind,
    pub children: Vec<Child>,
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        let mut stack = Vec::new();
        stack.extend(self.children.iter().map(|child| (1, child)).rev());
        s += &format!("{:?}\n", self.kind);

        while let Some((indentation, child)) = stack.pop() {
            match child {
                Child::Token(token) => {
                    s += &format!(
                        "{}\"{}\" [{:?}]\n",
                        "  ".repeat(indentation),
                        token.text,
                        token.span
                    );
                }
                Child::Tree(tree) => {
                    s += &format!("{}{:?}\n", "  ".repeat(indentation), tree.kind);
                    stack.extend(
                        tree.children
                            .iter()
                            .map(|child| (indentation + 1, child))
                            .rev(),
                    );
                }
            }
        }

        write!(f, "{}", s)
    }
}

#[derive(Debug, Serialize)]
pub enum Child {
    Token(Token),
    Tree(Tree),
}

#[derive(Debug, Serialize)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
    pub span: Span,
}
