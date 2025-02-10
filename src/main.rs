use parser::Parser;
mod parser;

fn main() {
    let tree = Parser::parse("SELECT  ?a ?b  WHERE { }");
    println!("{}", tree);
}
