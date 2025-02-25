mod utils;
use std::{
    fs::File,
    io::{Read, Write},
    str::FromStr,
};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use ungrammar::{Grammar, Rule};
use utils::{compute_first, is_nullable, FirstSet};

pub fn generate() {
    let mut file = File::open("grammar.ungram").expect("File should exist");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("could not read file");
    let grammar = ungrammar::Grammar::from_str(&contents).unwrap();
    let first = compute_first(&grammar);

    generate_types(&grammar);
    generate_parser(&grammar, &first);
}

fn generate_rule(grammar: &Grammar, rule: &Rule, first: &FirstSet) -> TokenStream {
    match rule {
        Rule::Labeled {
            label: _,
            rule: other,
        } => generate_rule(grammar, other, first),
        Rule::Node(node) => {
            let ident = format_ident!("parse_{}", grammar[*node].name);
            quote! {#ident (p);}
        }
        Rule::Token(token) => {
            let ident = generate_token_kind(&grammar[*token].name);
            quote! {p.expect(SyntaxKind::#ident);}
        }
        Rule::Seq(rules) => rules
            .iter()
            .map(|other| generate_rule(grammar, other, first))
            .collect(),
        Rule::Alt(rules) => {
            let match_arms: Vec<TokenStream> = rules
                .iter()
                .map(|other_rule| {
                    let tokens: Vec<TokenStream> = first
                        .get_first_of(other_rule, grammar)
                        .iter()
                        .map(|token| {
                            let kind = generate_token_kind(&grammar[*token].name);
                            quote! { SyntaxKind::#kind }
                        })
                        .collect();
                    let parse_rule = generate_rule(grammar, other_rule, first);
                    quote! {
                        #(#tokens )|* => {
                            #parse_rule
                        }
                    }
                })
                .collect();
            let catch_arm = match is_nullable(rule, grammar) {
                true => quote! {
                    _ => {}
                },
                false => quote! {
                    _ =>{
                        p.advance_with_error("Expected ....");
                    }
                },
            };
            quote! {
                match p.nth(0){
                  #(#match_arms)*,
                  SyntaxKind::Eof => {
                        eprintln!("Unexpected Eof");
                        p.close(marker, SyntaxKind::Error);
                        return
                  },
                  #catch_arm
                };
            }
        }
        Rule::Opt(other_rule) => {
            let first_set: Vec<TokenStream> = first
                .get_first_of(other_rule, grammar)
                .iter()
                .map(|token| generate_token_kind(&grammar[*token].name))
                .map(|ident| quote! {SyntaxKind::#ident})
                .collect();

            let parse_rule = generate_rule(grammar, other_rule, first);
            quote! {
                if p.at_any(&[#(#first_set),*]){
                #parse_rule
                }
            }
        }
        Rule::Rep(other_rule) => {
            let first_set: Vec<TokenStream> = first
                .get_first_of(other_rule, grammar)
                .iter()
                .map(|token| generate_token_kind(&grammar[*token].name))
                .map(|ident| quote! {SyntaxKind::#ident})
                .collect();

            let parse_rule = generate_rule(grammar, other_rule, first);
            quote! {
                while [#(#first_set),*].contains(&p.nth(0)) {
                    #parse_rule
                }
            }
        }
    }
}

pub fn generate_token_kind(token: &str) -> proc_macro2::Ident {
    format_ident!(
        "{}",
        match token {
            "*" => "Star",
            "+" => "Plus",
            "-" => "Minus",
            "(" => "LParen",
            ")" => "RParen",
            "{" => "LCurly",
            "}" => "RCurly",
            "[" => "LBrack",
            "]" => "RBrack",
            "." => "Dot",
            "," => "Colon",
            ";" => "Semicolon",
            "|" => "Pipe",
            "||" => "DoublePipe",
            "/" => "Slash",
            "^" => "Zirkumflex",
            "^^" => "DoubleZirkumflex",
            "?" => "QuestionMark",
            "$" => "Dollar",
            "!" => "ExclamationMark",
            "&&" => "DoubleAnd",
            "=" => "Equals",
            "!=" => "ExclamationMarkEquals",
            "<" => "Less",
            "<=" => "LessEquals",
            ">" => "More",
            ">=" => "MoreEquals",
            "true" => "True",
            "false" => "False",
            other => other,
        }
    )
}

fn generate_types(grammar: &Grammar) {
    let token_kinds = grammar
        .tokens()
        .map(|token| generate_token_kind(grammar[token].name.as_str()))
        .map(|token| {
            let x = format!("{}", token);
            quote! {

                    #[token(#x, ignore(case))]
                    #token
            }
        });
    let parser_trees = grammar
        .iter()
        .map(|node| format_ident!("{}", grammar[node].name));

    let tokens = quote! {
        use logos::{Logos, Span};

        #[allow(non_camel_case_types)]
        #[derive(Logos, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Serialize)]
        #[repr(u16)]
        pub enum SyntaxKind  {
            Eof = 0,
            Error,
            #[regex(r#"[ \t\n\f]+"#)]
            WHITESPACE,
            #(#token_kinds),*
        }

        #[allow(non_camel_case_types)]
        pub enum SyntaxKind {
            Error,
            #(#parser_trees),*
        }
    };

    let syntax_tree = syn::parse2(tokens).unwrap();
    let formatted_code = prettyplease::unparse(&syntax_tree);

    let mut file = File::create("gen_kinds.rs").unwrap();
    file.write_all(formatted_code.as_bytes()).unwrap();
}

fn format_rule(grammar: &Grammar, rule: &Rule) -> String {
    match rule {
        ungrammar::Rule::Labeled {
            label: _,
            rule: other,
        } => format_rule(grammar, other),
        ungrammar::Rule::Node(node) => grammar[*node].name.clone(),
        ungrammar::Rule::Token(token) => format!("'{}'", grammar[*token].name.clone()),
        ungrammar::Rule::Seq(rules) => rules
            .iter()
            .map(|other| {
                let formatted = format_rule(grammar, other);
                match other {
                    Rule::Alt(_) => format!("({})", formatted),
                    _ => formatted,
                }
            })
            .collect::<Vec<String>>()
            .join(" "),
        ungrammar::Rule::Alt(rules) => rules
            .iter()
            .map(|other| format_rule(grammar, other))
            .collect::<Vec<String>>()
            .join(" | "),
        ungrammar::Rule::Opt(other) => match **other {
            Rule::Seq(_) | Rule::Alt(_) => format!("({})?", format_rule(grammar, other)),
            _ => format!("{}?", format_rule(grammar, other)),
        },

        ungrammar::Rule::Rep(other) => match **other {
            Rule::Seq(_) | Rule::Alt(_) => format!("({})*", format_rule(grammar, other)),
            _ => format!("{}*", format_rule(grammar, other)),
        },
    }
}

fn generate_parser(grammar: &Grammar, first: &FirstSet) {
    let functions = grammar.iter().enumerate().map(|(idx, node)| {
        let comment = format!(
            " [{}] {} -> {}",
            idx,
            grammar[node].name,
            format_rule(grammar, &grammar[node].rule)
        );
        let tree_kind = format_ident!("{}", grammar[node].name);
        let function_name = format_ident!("parse_{}", grammar[node].name);
        let rules = generate_rule(grammar, &grammar[node].rule, first);
        quote! {
            #[doc = #comment]


            pub (super) fn #function_name (p: &mut Parser){
                let marker = p.open();
                #rules

                p.close(marker, SyntaxKind::#tree_kind);
            }
        }
    });
    let parser = quote! {
        #![allow(non_snake_case)]
        use crate::SyntaxKind;
        use super::Parser;
         #(#functions)*
    };

    let syntax_tree = syn::parse2(parser).unwrap();
    let formatted_code = prettyplease::unparse(&syntax_tree);

    let mut file = File::create("src/parser/grammar.rs").unwrap();
    file.write_all(formatted_code.as_bytes()).unwrap();
}
