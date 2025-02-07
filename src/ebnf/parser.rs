use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_while1},
    character::complete::{line_ending, multispace0, none_of, space1},
    error::ParseError,
    multi::{many0, separated_list1},
    sequence::{delimited, separated_pair, terminated},
    AsChar, IResult, Parser,
};

use super::{Grammar, Rule, Term};

pub fn ws<'a, O, E: ParseError<&'a str>, F>(inner: F) -> impl Parser<&'a str, Output = O, Error = E>
where
    F: Parser<&'a str, Output = O, Error = E>,
{
    delimited(multispace0, inner, multispace0)
}

fn word(input: &str) -> IResult<&str, &str> {
    take_while1(|c| !"\n ()'|+*?".contains(c))(input)
}

fn term_or_nonterm(input: &str) -> IResult<&str, Term> {
    word(input).map(|(rest, string)| {
        (
            rest,
            match string
                .chars()
                .all(|c| !c.is_alpha() || c.is_ascii_uppercase())
            {
                true => Term::Terminal(string.to_string()),
                false => Term::NonTerminal(string.to_string()),
            },
        )
    })
}

fn string(input: &str) -> IResult<&str, Term> {
    delimited(
        tag("'"),
        escaped(many0(none_of("\\\'")), '\\', tag("'")),
        tag("'"),
    )
    .parse(input)
    .map(|(rest, word)| (rest, Term::Terminal(format!("'{}'", word.to_string()))))
}

fn term1(input: &str) -> IResult<&str, Term> {
    alt((
        term_or_nonterm,
        string,
        delimited(tag("( "), term4, tag(" )")),
    ))
    .parse(input)
}

fn term2(input: &str) -> IResult<&str, Term> {
    alt((kleene, plus, optional, term1)).parse(input)
}

fn term3(input: &str) -> IResult<&str, Term> {
    alt((concatination, term2)).parse(input)
}

fn term4(input: &str) -> IResult<&str, Term> {
    alt((alternation, term3)).parse(input)
}

fn optional(input: &str) -> IResult<&str, Term> {
    terminated(term1, tag("?"))
        .parse(input)
        .map(|(rest, term)| (rest, Term::Optional(Box::new(term))))
}

fn kleene(input: &str) -> IResult<&str, Term> {
    terminated(term1, tag("*"))
        .parse(input)
        .map(|(rest, term)| (rest, Term::Kleene(Box::new(term))))
}

fn plus(input: &str) -> IResult<&str, Term> {
    terminated(term1, tag("+"))
        .parse(input)
        .map(|(rest, term)| (rest, Term::Plus(Box::new(term))))
}

fn alternation(input: &str) -> IResult<&str, Term> {
    separated_list1(ws(tag("|")), term3)
        .parse(input)
        .map(|(rest, terms)| (rest, Term::Alternation(terms)))
}

fn concatination(input: &str) -> IResult<&str, Term> {
    separated_list1(space1, term2)
        .parse(input)
        .map(|(rest, terms)| (rest, Term::Concatination(terms)))
}

fn term(input: &str) -> IResult<&str, Term> {
    term4(input).map(|(rest, mut term)| {
        term.flatten();
        (rest, term)
    })
}

fn rule(input: &str) -> IResult<&str, Rule> {
    separated_pair(word, ws(tag("::=")), term)
        .parse(input)
        .map(|(rest, (word, term))| (rest, Rule(word.to_string(), term)))
}

pub(super) fn parse_grammar(input: &str) -> IResult<&str, Grammar> {
    terminated(separated_list1(line_ending, rule), line_ending)
        .parse(input)
        .map(|(rest, rules)| {
            (
                rest,
                Grammar {
                    start: rules[0].0.clone(),
                    rules,
                },
            )
        })
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::ebnf::{
        parser::{concatination, optional, term, term_or_nonterm},
        Rule, Term,
    };

    use super::{parse_grammar, rule, string};

    #[test]
    fn test_parse_grammar() {
        let grammar_str = indoc!(
            "SelectClause  ::=  'SELECT' ( Var+ | '*' ) ( 'DISTINCT' | 'REDUCED' )?
             QueryUnit  ::=  Query
             QueryUnit  ::=  Query
            "
        );
        fully_parsed(parse_grammar(grammar_str).unwrap());
    }

    #[test]
    fn test_parse_string() {
        string("'s'").unwrap();
    }

    fn fully_parsed<T: std::fmt::Debug>(res: (&str, T)) {
        let (rest, result) = res;
        if rest != "" && rest != "\n" {
            println!("Not fully parsed!");
            println!("Remaining input: \"{}\"", rest);
            println!("intermediat result: {:?}", result);
            panic!("Not fully parsed")
        }
    }

    #[test]
    fn parse_rule2() {
        rule("DatasetClause ::= 'FROM' ( DefaultGraphClause | NamedGraphClause )").unwrap();
    }

    #[test]
    fn parse_rule1() {
        let (rest, res) =
            rule("SelectClause  ::=  'SELECT' ( 'DISTINCT' | 'REDUCED' )? ( Var+ | '*' )").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            res,
            Rule(
                "SelectClause".to_string(),
                Term::Concatination(vec![
                    Term::Terminal("'SELECT'".to_string()),
                    Term::Optional(Box::new(Term::Alternation(vec![
                        Term::Terminal("'DISTINCT'".to_string()),
                        Term::Terminal("'REDUCED'".to_string()),
                    ]))),
                    Term::Alternation(vec![
                        Term::Plus(Box::new(Term::NonTerminal("Var".to_string()))),
                        Term::Terminal("'*'".to_string()),
                    ])
                ])
            )
        );
        rule("SelectClause  ::=  'SELECT' ( 'DISTINCT' | 'REDUCED' )? (  Var+ | '*' )").unwrap();
    }

    #[test]
    fn parse_non_terminal() {
        let (rest, res) = term_or_nonterm("Query ").unwrap();
        assert_eq!(rest, " ");
        assert_eq!(res, Term::NonTerminal("Query".to_string()));
    }

    #[test]
    fn parse_terminal_var() {
        let (rest, res) = term_or_nonterm("Abc123 ").unwrap();
        assert_eq!(rest, " ");
        assert_eq!(res, Term::NonTerminal("Abc123".to_string()));
    }

    #[test]
    fn parse_terminal() {
        let (rest, res) = string("']'").unwrap();
        assert_eq!(rest, "");
        assert_eq!(res, Term::Terminal("']'".to_string()));
    }

    #[test]
    fn parse_optional() {
        let (rest, res) = optional("Triples?").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            res,
            Term::Optional(Box::new(Term::NonTerminal("Triples".to_string())))
        );
    }

    #[test]
    fn parse_kleene() {
        let (rest, res) = term("( Ab | Bb Cc )*").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            res,
            Term::Kleene(Box::new(Term::Alternation(vec![
                Term::NonTerminal("Ab".to_string()),
                Term::Concatination(vec![
                    Term::NonTerminal("Bb".to_string()),
                    Term::NonTerminal("Cc".to_string()),
                ])
            ])))
        );
    }

    #[test]
    fn parse_alternation() {
        let (rest, res) = term("Query | Bb* | Abc1234? | Rule1 Rule2").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            res,
            Term::Alternation(vec![
                Term::NonTerminal("Query".to_string()),
                Term::Kleene(Box::new(Term::NonTerminal("Bb".to_string()))),
                Term::Optional(Box::new(Term::NonTerminal("Abc1234".to_string()))),
                Term::Concatination(vec![
                    Term::NonTerminal("Rule1".to_string()),
                    Term::NonTerminal("Rule2".to_string())
                ])
            ])
        );
    }

    #[test]
    fn parse_concat() {
        let (rest, res) = concatination("Query Bb   Abc1234").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            res,
            Term::Concatination(vec![
                Term::NonTerminal("Query".to_string()),
                Term::NonTerminal("Bb".to_string()),
                Term::NonTerminal("Abc1234".to_string())
            ])
        );
    }

    #[test]
    fn parse_term() {
        let (rest, res) = concatination("Ab Bb PNAME_NS  'Abc1234' De '}'").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            res,
            Term::Concatination(vec![
                Term::NonTerminal("Ab".to_string()),
                Term::NonTerminal("Bb".to_string()),
                Term::Terminal("PNAME_NS".to_string()),
                Term::Terminal("'Abc1234'".to_string()),
                Term::NonTerminal("De".to_string()),
                Term::Terminal("'}'".to_string())
            ])
        );
    }
}
