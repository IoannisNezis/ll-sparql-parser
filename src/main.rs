mod ebnf;
mod parser;
use ebnf::Grammar;
use parser::Parser;

fn main() {
    // while let Some(Ok(token)) = lex.next() {
    //     println!("{} {:?} [{:?}]", lex.slice(), token, lex.span());
    // }
    let grammar = Grammar::from_file("grammar_mini.ebnf").expect("Grammar file should be readable");
    let parser = Parser::from_grammar(grammar);
    // parser.parse("SELECT    <http://dings.de/dings#asd> ?abc * WHERE { ?dasd0123a $a ?a ?a [ ?a ?b ]] ] ] ? ] }");
    parser.parse("(id + (id * id)");
}
