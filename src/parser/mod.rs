mod grammar;

use std::cell::Cell;

use crate::SyntaxKind;
use logos::Logos;
use rowan::{GreenNode, GreenNodeBuilder};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    fuel: Cell<u32>,
    events: Vec<Event>,
}

#[derive(Debug)]
struct Token {
    kind: SyntaxKind,
    text: std::string::String,
    start: usize,
}

impl Parser {
    pub fn parse(text: &str) -> GreenNode {
        let mut parser = Parser {
            tokens: lex(text),
            pos: 0,
            fuel: 256.into(),
            events: Vec::new(),
        };
        if true {
            grammar::parse_QueryUnit(&mut parser);
        } else {
            grammar::parse_UpdateUnit(&mut parser);
        }
        parser.build_tree()
    }
}

enum Event {
    Open { kind: SyntaxKind },
    Close,
    Advance,
}

struct MarkOpened {
    index: usize,
}

impl Parser {
    fn open(&mut self) -> MarkOpened {
        let mark = MarkOpened {
            index: self.events.len(),
        };
        self.events.push(Event::Open {
            kind: SyntaxKind::Error,
        });
        mark
    }

    fn close(&mut self, m: MarkOpened, kind: SyntaxKind) {
        self.events[m.index] = Event::Open { kind };
        self.events.push(Event::Close);
    }

    fn advance(&mut self) {
        assert!(!self.eof());
        self.fuel.set(256);
        self.events.push(Event::Advance);
        self.pos += 1;
    }

    fn eof(&self) -> bool {
        self.pos == self.tokens.len()
    }

    fn nth(&self, lookahead: usize) -> SyntaxKind {
        if self.fuel.get() == 0 {
            panic!("parser is stuck")
        }
        self.fuel.set(self.fuel.get() - 1);
        self.tokens
            .get(self.pos + lookahead)
            .map_or(SyntaxKind::Eof, |it| it.kind)
    }

    fn at(&self, kind: SyntaxKind) -> bool {
        self.nth(0) == kind
    }

    fn at_any(&self, kinds: &[SyntaxKind]) -> bool {
        kinds.iter().any(|kind| self.at(*kind))
    }

    fn eat(&mut self, kind: SyntaxKind) -> bool {
        if self.at(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, kind: SyntaxKind) {
        if self.eat(kind) {
            return;
        }
        // TODO: Error reporting.
        eprintln!("expected {kind:?}");
    }

    fn advance_with_error(&mut self, error: &str) {
        let m = self.open();
        // TODO: Error reporting.
        eprintln!("{error}");
        self.advance();
        self.close(m, SyntaxKind::Error);
    }

    fn build_tree(self) -> GreenNode {
        let mut tokens = self.tokens.into_iter();
        let mut events = self.events;
        let mut builder = GreenNodeBuilder::new();

        // Special case: pop the last `Close` event to ensure
        // that the stack is non-empty inside the loop.
        assert!(matches!(events.pop(), Some(Event::Close)));

        for event in events {
            match event {
                Event::Open { kind } => builder.start_node(kind.into()),
                Event::Close => builder.finish_node(),
                Event::Advance => {
                    let token = tokens.next().unwrap();
                    builder.token(token.kind.into(), &token.text);
                }
            }
        }
        builder.finish()
    }
}

fn lex(text: &str) -> Vec<Token> {
    let mut lexer = SyntaxKind::lexer(text);
    let mut tokens = Vec::new();

    while let Some(result) = lexer.next() {
        tokens.push(Token {
            kind: result.unwrap_or(SyntaxKind::Error),
            text: lexer.slice().to_string(),
            start: lexer.span().start,
        });
    }
    return tokens;
}
