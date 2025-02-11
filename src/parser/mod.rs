mod syntax_tree;

use std::cell::Cell;

use logos::Logos;
use syntax_tree::{Child, Token, TokenKind, Tree, TreeKind};

// [1] QueryUnit = Query
fn base(p: &mut Parser) {
    let m = p.open();
    p.close(m, TreeKind::QueryUnit);
}

// [1] QueryUnit = Query
fn query_unit(p: &mut Parser) {
    let m = p.open();
    query(p);
    p.close(m, TreeKind::QueryUnit);
}

// [2] Query = Prologue ( SelectQuery | ConstructQuery | DescribeQuery | AskQuery ) ValuesClause
fn query(p: &mut Parser) {
    let m = p.open();

    if p.at_any(&[TokenKind::BASE, TokenKind::PREFIX]) {
        prologue(p);
    }
    select_query(p);

    p.close(m, TreeKind::Query);
}

// [4] Prologue = ( BaseDecl | PrefixDecl )*
fn prologue(p: &mut Parser) {
    let m = p.open();

    loop {
        match p.nth(0) {
            TokenKind::BASE => base_decl(p),
            TokenKind::PREFIX => prefix_decl(p),
            _ => break,
        }
    }
    p.close(m, TreeKind::Prologue);
}

// [5] BaseDecl = 'BASE' 'IRIREF'
fn base_decl(p: &mut Parser) {
    let m = p.open();
    p.expect(TokenKind::BASE);
    p.expect(TokenKind::IRIREF);
    p.close(m, TreeKind::BaseDecl);
}

// [6] PrefixDecl = 'PREFIX' 'PNAME_NS' 'IRIREF'
fn prefix_decl(p: &mut Parser) {
    let m = p.open();
    p.expect(TokenKind::PREFIX);
    p.expect(TokenKind::PNAME_NS);
    p.expect(TokenKind::IRIREF);
    p.close(m, TreeKind::PrefixDecl);
}

// [7] SelectQuery = SelectClause DatasetClause* WhereClause SolutionModifier
fn select_query(p: &mut Parser) {
    let m = p.open();
    // SelectClause
    select_clause(p);
    // WhereClause
    where_clause(p);

    p.close(m, TreeKind::SelectQuery);
}

// [9] SelectClause = 'SELECT' ( 'DISTINCT' | 'REDUCED' )? ( ( Var | ( '(' Expression 'AS' Var ')' ) ) ( Var | ( '(' Expression 'AS' Var ')' ) )* | '*' )
fn select_clause(p: &mut Parser) {
    let m = p.open();

    // 'SELECT'
    p.expect(TokenKind::SELECT);

    // ('DESTINCT' | 'REDUCED')?
    p.eat(TokenKind::DISTINCT);
    p.eat(TokenKind::REDUCED);

    // ( Var | ( '(' Expression 'AS' Var ')' ) )
    match p.nth(0) {
        TokenKind::Star => p.advance(),
        TokenKind::VAR1 | TokenKind::VAR2 | TokenKind::LParen => {
            match p.nth(0) {
                TokenKind::VAR1 | TokenKind::VAR2 => p.advance(),
                TokenKind::LParen => {
                    p.advance();
                    // TODO: Expression
                    p.expect(TokenKind::AS);
                    p.eat(TokenKind::VAR1);
                    p.eat(TokenKind::VAR2);
                    p.expect(TokenKind::RParen);
                }
                _ => p.advance_with_error("Expected Var or assignment"),
            };

            loop {
                match p.nth(0) {
                    TokenKind::VAR1 | TokenKind::VAR2 => p.advance(),
                    TokenKind::LParen => {
                        p.advance();
                        // TODO: Expression
                        p.expect(TokenKind::AS);
                        p.eat(TokenKind::VAR1);
                        p.eat(TokenKind::VAR2);
                        p.expect(TokenKind::RParen);
                    }
                    _ => break,
                };
            }
        }
        _ => p.advance_with_error("Expected Star, Var or assignment"),
    }

    p.close(m, TreeKind::SelectClause);
}

// WhereClause = 'WHERE'? GroupGraphPattern
fn where_clause(p: &mut Parser) {
    let m = p.open();
    p.eat(TokenKind::WHERE);
    group_graph_pattern(p);
    p.close(m, TreeKind::WhereClause);
}

// GroupGraphPattern = '{' '}'
fn group_graph_pattern(p: &mut Parser) {
    let m = p.open();
    p.expect(TokenKind::LCurly);
    p.expect(TokenKind::RCurly);
    p.close(m, TreeKind::GroupGraphPattern);
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    fuel: Cell<u32>,
    events: Vec<Event>,
}

impl Parser {
    pub fn parse(text: &str) -> Tree {
        let mut parser = Parser {
            tokens: lex(text),
            pos: 0,
            fuel: 256.into(),
            events: Vec::new(),
        };
        query_unit(&mut parser);
        parser.build_tree()
    }
}

enum Event {
    Open { kind: TreeKind },
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
            kind: TreeKind::ErrorTree,
        });
        mark
    }

    fn close(&mut self, m: MarkOpened, kind: TreeKind) {
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

    fn nth(&self, lookahead: usize) -> TokenKind {
        if self.fuel.get() == 0 {
            panic!("parser is stuck")
        }
        self.fuel.set(self.fuel.get() - 1);
        self.tokens
            .get(self.pos + lookahead)
            .map_or(TokenKind::Eof, |it| it.kind)
    }

    fn at(&self, kind: TokenKind) -> bool {
        self.nth(0) == kind
    }

    fn at_any(&self, kinds: &[TokenKind]) -> bool {
        kinds.iter().any(|kind| self.at(*kind))
    }

    fn eat(&mut self, kind: TokenKind) -> bool {
        if self.at(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, kind: TokenKind) {
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
        self.close(m, TreeKind::ErrorTree);
    }

    fn build_tree(self) -> Tree {
        let mut tokens = self.tokens.into_iter();
        let mut events = self.events;
        let mut stack = Vec::new();

        // Special case: pop the last `Close` event to ensure
        // that the stack is non-empty inside the loop.
        assert!(matches!(events.pop(), Some(Event::Close)));

        for event in events {
            match event {
                // Starting a new node; just push an empty tree to the stack.
                Event::Open { kind } => stack.push(Tree {
                    kind,
                    children: Vec::new(),
                }),

                // A tree is done.
                // Pop it off the stack and append to a new current tree.
                Event::Close => {
                    let tree = stack.pop().unwrap();
                    stack
                        .last_mut()
                        // If we don't pop the last `Close` before this loop,
                        // this unwrap would trigger for it.
                        .unwrap()
                        .children
                        .push(Child::Tree(tree));
                }

                // Consume a token and append it to the current tree
                Event::Advance => {
                    let token = tokens.next().unwrap();
                    stack.last_mut().unwrap().children.push(Child::Token(token));
                }
            }
        }

        // Our parser will guarantee that all the trees are closed
        // and cover the entirety of tokens.
        assert!(stack.len() == 1);
        assert!(tokens.next().is_none());

        stack.pop().unwrap()
    }
}

fn lex(text: &str) -> Vec<Token> {
    let mut lexer = TokenKind::lexer(text);
    let mut tokens = Vec::new();

    while let Some(result) = lexer.next() {
        tokens.push(Token {
            kind: result.unwrap_or(TokenKind::ErrorToken),
            text: lexer.slice().to_string(),
            span: lexer.span(),
        });
    }
    return tokens;
}
