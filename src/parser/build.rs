use std::collections::{HashMap, HashSet};

use crate::ebnf::{Grammar, Rule, Term};

use super::ParsingTable;

type FirstSet = HashMap<String, HashSet<String>>;
type FollowSet = HashMap<String, HashSet<String>>;

pub(super) fn construct_parse_table(grammar: &Grammar) -> ParsingTable {
    // NOTE: compute FIRST set
    let mut first = FirstSet::new();
    for rule in &grammar.rules {
        compute_first_set(&rule.0, &grammar, &mut first);
    }

    // NOTE: compute FOLLOW set
    let mut follow = FollowSet::new();
    for rule in &grammar.rules {
        compute_follow_set(&rule.0, &grammar, &first, &mut follow);
    }

    // NOTE: compute parsing table
    let mut table: ParsingTable = ParsingTable::new();
    for rule in &grammar.rules {
        let first_set = get_first(&rule.1, grammar, &mut first);
        for terminal in &first_set {
            if terminal != "''" {
                assert_eq!(
                    table.insert((rule.0.clone(), terminal.clone()), rule.1.clone()),
                    None
                );
            }
        }
        if first_set.contains("''") {
            for terminal in follow
                .get(&rule.0)
                .expect(&format!("Terminal \"{}\" should have a FOLLOW set", rule.0))
            {
                assert_eq!(
                    table.insert((rule.0.clone(), terminal.clone()), rule.1.clone()),
                    None
                );
            }
        }
    }
    return table;
}

fn compute_follow_set(
    non_term: &String,
    grammar: &Grammar,
    first_set: &FirstSet,
    follow_set: &mut FollowSet,
) {
    if follow_set.contains_key(non_term) {
        return;
    }
    follow_set.insert(non_term.clone(), HashSet::new());
    if non_term == &grammar.start {
        follow_set.entry(non_term.to_string()).and_modify(|e| {
            e.insert("$".to_string());
        });
    }
    grammar
        .rules
        .iter()
        .filter_map(|rule| match rule {
            Rule(_lhs, Term::Concatination(terms)) => {
                if terms.iter().any(|term| match term {
                    Term::NonTerminal(nt) if nt == non_term => true,
                    _ => false,
                }) {
                    return Some(rule);
                }
                return None;
            }
            _ => None,
        })
        .for_each(|rule| {
            match &rule.1 {
                Term::Concatination(terms) => {
                    terms
                        .iter()
                        .zip(terms.iter().skip(1))
                        .filter_map(|(elem, next)| match elem {
                            Term::NonTerminal(nt) if nt == non_term => Some(next),
                            _ => None,
                        })
                        .for_each(|elem| match elem {
                            Term::Terminal(t) => {
                                follow_set
                                    .entry(non_term.clone())
                                    .and_modify(|e| {
                                        e.insert(t.clone());
                                    })
                                    .or_insert(HashSet::from([t.clone()]));
                            }
                            Term::NonTerminal(nt) => {
                                let mut first = first_set
                                    .get(nt)
                                    .expect(&format!(
                                        "FIRST of {} should have beed calculated already",
                                        nt
                                    ))
                                    .clone();
                                if first.contains("''") {
                                    first.remove("''");
                                    compute_follow_set(nt, grammar, first_set, follow_set);
                                    let follow = follow_set.get(nt).expect(&format!(
                                        "FIRST of {} should have beed calculated already",
                                        nt
                                    ));
                                    first.extend(follow.clone());
                                }
                                follow_set
                                    .entry(non_term.clone())
                                    .and_modify(|e| {
                                        e.extend(first.clone());
                                    })
                                    .or_insert(first);
                            }
                            _ => {}
                        });
                    match terms.last() {
                        Some(Term::NonTerminal(nt)) if nt == non_term => {
                            compute_follow_set(&rule.0, grammar, first_set, follow_set);
                            let set = follow_set
                                .get(&rule.0)
                                .expect(&format!(
                                    "FOLLOW of {} should have beed calculated already",
                                    nt
                                ))
                                .clone();
                            follow_set
                                .entry(non_term.clone())
                                .and_modify(|e| {
                                    e.extend(set.clone());
                                })
                                .or_insert(set);
                        }
                        _ => {}
                    }
                }
                _ => panic!("All rules in the computation of FOLLOW should be concatinations"),
            };
        });
}

fn compute_first_set(non_term: &String, grammar: &Grammar, first_set: &mut FirstSet) {
    if let Some(_) = first_set.get(non_term) {
        return;
    }
    first_set.insert(non_term.clone(), HashSet::new());
    grammar
        .rules
        .iter()
        .filter_map(|rule| match rule {
            Rule(lhs, rhs) if lhs == non_term => Some(rhs),
            _ => None,
        })
        .for_each(|production| {
            let set = get_first(production, grammar, first_set);
            first_set
                .entry(non_term.to_string())
                .and_modify(|e| {
                    e.extend(set.clone());
                })
                .or_insert(set);
        });
}

fn get_first(term: &Term, grammar: &Grammar, first_set: &mut FirstSet) -> HashSet<String> {
    match term {
        Term::Terminal(term) => HashSet::from([term.clone()]),
        Term::NonTerminal(x) => {
            compute_first_set(x, grammar, first_set);
            first_set
                .get(x)
                .expect(&format!("First set entry for {} should be computed", x))
                .clone()
        }
        Term::Concatination(terms) => {
            let mut accu_set: HashSet<String> = HashSet::new();
            terms
                .iter()
                .scan(&mut accu_set, |accu, elem| {
                    let set = get_first(elem, grammar, first_set);
                    accu.extend(set);
                    if accu.contains("''") {
                        Some(())
                    } else {
                        None
                    }
                })
                .last();
            accu_set
        }
        Term::Optional(_) | Term::Kleene(_) | Term::Plus(_) | Term::EOF | Term::Alternation(_) => {
            panic!("FIRST set can not be computet for EBNF grammars")
        }
    }
}
