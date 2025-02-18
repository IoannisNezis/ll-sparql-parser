mod parser;
use parser::Parser;

fn main() {
    let _tree = Parser::parse("SELECT * where { ?  ");
    println!("{}", _tree);
}
