use crate::SyntaxKind::{self, *};

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

pub type SyntaxNode = rowan::SyntaxNode<Sparql>;
pub type SyntaxToken = rowan::SyntaxToken<Sparql>;
pub type SyntaxElement = rowan::SyntaxElement<Sparql>;
pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<Sparql>;
pub type SyntaxElementChildren = rowan::SyntaxElementChildren<Sparql>;
pub type PreorderWithTokens = rowan::api::PreorderWithTokens<Sparql>;
