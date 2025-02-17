mod parser;
use parser::Parser;

fn main() {
    let tree =
        Parser::parse("PREFIX namespace: <> SElECt * where { ?a rdfs:label ?c FILTER  (?a) }");
    println!("{}", tree);
}
