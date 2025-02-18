mod parser;
use parser::Parser;

fn main() {
    let _tree = Parser::parse(
        "PREFIX namespace: <> SElECt * where { {{ Filter (?a) }} ?a rdfs:label ?c FILTER  (?a) } group By ?a  ",
    );
    // println!("{}", _tree);
}
