mod build;
mod lexer;

use std::collections::{HashMap, HashSet};

use lexer::Token;
use logos::Logos;

use crate::ebnf::{Grammar, Term};

type FirstSet = HashMap<String, HashSet<String>>;
type FollowSet = HashMap<String, HashSet<String>>;
type ParsingTable = HashMap<(String, String), Term>;

pub struct Parser {
    grammar: Grammar,
    parsing_table: ParsingTable,
}

impl Parser {
    pub fn from_grammar(grammar: Grammar) -> Self {
        Self {
            parsing_table: build::construct_parse_table(&grammar),
            grammar,
        }
    }

    pub(crate) fn parse(&self, input: &str) {
        let mut lex = Token::lexer(input);
        let mut stack: Vec<Term> = Vec::new();
        stack.push(Term::EOF);
        stack.push(Term::NonTerminal(self.grammar.start.clone()));

        while let Some(Ok(token)) = lex.next() {
            println!("{} {:?} [{:?}]", lex.slice(), token, lex.span());
        }
    }
}
