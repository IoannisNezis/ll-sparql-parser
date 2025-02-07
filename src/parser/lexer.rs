use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
pub enum Token {
    // [139]
    #[regex(r#"<([^<>"{}|^`\\\u{0}-\u{20}])*>"#)]
    IRIREF,

    #[token("SELECT")]
    SELECT,

    #[token("WHERE")]
    WHERE,

    #[regex(r"\?[A-Za-z0-9]+")]
    VAR1,

    #[regex(r"\$[A-Za-z0-9]+")]
    VAR2,

    #[token("*")]
    STAR,

    #[token("(")]
    LeftParenthesis,

    #[token("{")]
    LeftCurlyBreaked,

    #[token("[")]
    LeftSquareBreaked,

    #[token(")")]
    RightParenthesis,

    #[token("}")]
    RightCurlyBreaked,

    #[token("]")]
    RightSquareBreaked,

    #[token(".")]
    Dot,

    #[token(";")]
    Semicolon,
}
