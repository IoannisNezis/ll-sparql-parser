mod generator;
mod parser;
use parser::Parser;

fn main() {
    generator::generate();
    let tree =
        Parser::parse("PREFIX namespace: <> SElECt * where { ?a rdfs:label ?c FILTER  (?a) }");
    println!("{}", tree);
}
