use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
#[logos(skip r"[ \t\n\f]+")]
pub enum TokenKind {
    Eof,
    ErrorToken,
    #[token("SELECT")]
    SELECT,
    #[token("DESTINCT")]
    DESTINCT,
    #[token("REDUCED")]
    REDUCED,
    #[regex(r"[\?$][A-Za-z]+")]
    VAR,
    #[token("*")]
    STAR,
    #[token("WHERE")]
    WHERE,
    #[token("{")]
    LCurly,
    #[token("}")]
    RCurly,
}

#[derive(Debug)]
pub enum TreeKind {
    ErrorTree,
    SelectQuery,
    SelectClause,
    WhereClause,
    GroupGraphPattern,
}
