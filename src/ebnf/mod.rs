mod parser;

use std::{fmt, fs::File, io::Read};

use parser::parse_grammar;

#[derive(Debug)]
pub struct Grammar {
    pub start: String,
    pub rules: Vec<Rule>,
}

impl Grammar {
    pub fn from_file(file_path: &str) -> Result<Self, String> {
        let mut file = File::open(file_path).map_err(|_| "Could not open grammar file")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|_| "Could not read grammar file")?;

        match parse_grammar(&contents) {
            Ok(("", grammar)) => Ok(grammar),
            Ok((rest, _grammar)) => Err(format!("could not parse grammar file\n{}", rest)),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl fmt::Display for Grammar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.rules
                .iter()
                .map(|term| format!("{}", term))
                .collect::<Vec<String>>()
                .as_slice()
                .join("\n")
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Rule(pub String, pub Term);

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ::= {}", self.0, self.1)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Term {
    Terminal(String),
    NonTerminal(String),
    Alternation(Vec<Term>),
    Concatination(Vec<Term>),
    Optional(Box<Term>),
    Kleene(Box<Term>),
    Plus(Box<Term>),
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Terminal(s) | Term::NonTerminal(s) => write!(f, "{}", s),
            Term::Alternation(terms) => {
                write!(
                    f,
                    "{}",
                    terms
                        .iter()
                        .map(|term| format!("{}", term))
                        .collect::<Vec<String>>()
                        .as_slice()
                        .join(" | ")
                )
            }
            Term::Concatination(terms) => {
                write!(
                    f,
                    "{}",
                    terms
                        .iter()
                        .map(|term| match term {
                            Term::Concatination(_) | Term::Alternation(_) =>
                                format!("( {} )", term),
                            Term::NonTerminal(_)
                            | Term::Terminal(_)
                            | Term::Optional(_)
                            | Term::Kleene(_)
                            | Term::Plus(_) => format!("{}", term),
                        })
                        .collect::<Vec<String>>()
                        .as_slice()
                        .join(" ")
                )
            }
            Term::Plus(term) | Term::Kleene(term) | Term::Optional(term) => {
                let suffix = match self {
                    Term::Optional(_) => "?",
                    Term::Kleene(_) => "*",
                    Term::Plus(_) => "+",
                    _ => "",
                };
                match **term {
                    Term::Concatination(_) | Term::Alternation(_) => {
                        write!(f, "( {} ){}", term, suffix)
                    }
                    Term::NonTerminal(_)
                    | Term::Terminal(_)
                    | Term::Optional(_)
                    | Term::Kleene(_)
                    | Term::Plus(_) => write!(f, "{}{}", term, suffix),
                }
            }
        }
    }
}

impl Term {
    fn flatten(&mut self) {
        match self {
            Term::Alternation(terms) | Term::Concatination(terms) => {
                terms.iter_mut().for_each(|term| term.flatten());
                if terms.len() == 1 {
                    *self = terms[0].clone();
                }
            }
            Term::Optional(term) | Term::Kleene(term) | Term::Plus(term) => term.flatten(),
            _other => {}
        };
    }
}
