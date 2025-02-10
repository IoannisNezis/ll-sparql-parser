mod parser;
use std::{fs::File, io::Read, str::FromStr};

use parser::Parser;

fn main() {
    // while let Some(Ok(token)) = lex.next() {
    //     println!("{} {:?} [{:?}]", lex.slice(), token, lex.span());
    // }
    let mut file = File::open("grammar_mini.ungram").expect("Could not open grammar file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read grammar file");
    let grammar = ungrammar::Grammar::from_str(&contents).unwrap();
    let parser = Parser::from_grammar(grammar);
    // parser.parse("SELECT    <http://dings.de/dings#asd> ?abc * WHERE { ?dasd0123a $a ?a ?a [ ?a ?b ]] ] ] ? ] }");
    parser.parse("id * id");
}
