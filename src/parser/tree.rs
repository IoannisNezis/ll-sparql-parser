use logos::Logos;
use strum_macros::EnumString;

#[derive(Logos, Debug, PartialEq, Eq, Hash, Clone, EnumString)]
#[logos(skip r"[ \t\n\f]+")]
pub enum TokenKind {
    Eof,
    #[token("+")]
    PLUS,
    #[token("*")]
    STAR,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("ID")]
    ID,
}

pub enum TreeKind {
    Expr,
    Term,
    ExprAdd,
    Final,
    ExprMult,
}
