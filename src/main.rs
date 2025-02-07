mod ebnf;
mod lexer;
mod parser;
use ebnf::Grammar;

fn main() {
    // let mut lex = Token::lexer(
    //     "SELECT    <http://dings.de/dings#asd> ?abc * WHERE { ?dasd0123a $a ?a ?a [ ?a ?b ]] ] ] ? ] }",
    // );
    // while let Some(Ok(token)) = lex.next() {
    //     println!("{} {:?} [{:?}]", lex.slice(), token, lex.span());
    // }
    let grammar = Grammar::from_file("grammar_mini.ebnf").expect("Grammar file should be readable");
    parser::construct_parse_table(&grammar);
}
