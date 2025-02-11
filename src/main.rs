mod parser;
mod utils;

use parser::Parser;
use std::{fs::File, io::Read, str::FromStr};
use ungrammar::Grammar;
use utils::construct_parse_table;

fn main() {
    let mut file = File::open("grammar.ungram").expect("File should exist");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("could not read file");
    let grammar = Grammar::from_str(&contents);

    let tree = Parser::parse("SElECt * where { }");
    println!("{}", tree);
}
