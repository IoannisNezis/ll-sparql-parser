mod build;
mod tree;

use std::collections::HashMap;

use tree::TokenKind;
use ungrammar::{Grammar, Node, Rule};

struct ParsingTable(HashMap<Node, HashMap<TokenKind, Rule>>);

impl ParsingTable {
    fn lookup(&self, node: &Node, token: &TokenKind) -> Option<&Rule> {
        self.0.get(node).map(|inner| inner.get(token)).flatten()
    }

    fn new() -> Self {
        Self(HashMap::new())
    }

    fn insert(&mut self, node: Node, token: TokenKind, rule: &Rule) {
        self.0
            .entry(node)
            .and_modify(|inner| {
                inner.insert(token.clone(), rule.clone());
            })
            .or_insert(HashMap::from([(token, rule.clone())]));
    }
}

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
        // let mut lex = TokenKind::lexer(input);
        // let mut stack: Vec<&Term> = Vec::new();
        // stack.push(&Term::EOF);
        // stack.push(&self.grammar.start);
        // let mut token = lex.next().unwrap().unwrap_or(Token::EOF);
        // loop {
        //     println!("Stack: {:?}", stack);
        //     println!("token: {:?}", &token);
        //     let stack_top = stack.pop().expect("The Stack should never be empty");
        //     match stack_top {
        //         Term::Terminal(terminal) if terminal == "''" => {}
        //         Term::Terminal(terminal) => {
        //             // TODO: match terminal and move lexer
        //             if token == Token::from_str(&terminal).unwrap() {
        //                 token = lex.next().unwrap_or(Ok(Token::EOF)).unwrap();
        //             } else {
        //                 panic!("Token did not match {:?} != {}", token, terminal);
        //             }
        //         }
        //         Term::NonTerminal(non_terminal) => {
        //             // NOTE: lookup rule and push to stack
        //             match self.parsing_table.lookup(non_terminal, &token) {
        //                 Some(Term::Concatination(terms)) => stack.extend(terms.iter().rev()),
        //                 Some(term) => stack.push(term),
        //                 None => {
        //                     panic!("No entry in parsing table ({}, {:?})", non_terminal, &token)
        //                 }
        //             };
        //         }
        //         Term::EOF => {
        //             println!("SUCCESS parsed everything!");
        //             break;
        //         }
        //         _ => panic!("Stack should only contain Terminals or non-terminals"),
        //     }
        // }
    }
}
