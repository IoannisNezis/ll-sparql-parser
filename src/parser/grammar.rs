#![allow(non_snake_case)]
use crate::SyntaxKind;

use super::Parser;
/// [0] QueryUnit -> Query
pub(super) fn parse_QueryUnit(p: &mut Parser) {
    let marker = p.open();
    parse_Query(p);
    p.close(marker, SyntaxKind::QueryUnit);
}
/// [1] Query -> Prologue (SelectQuery | ConstructQuery | DescribeQuery | AskQuery) ValuesClause
pub(super) fn parse_Query(p: &mut Parser) {
    let marker = p.open();
    parse_Prologue(p);
    match p.nth(0) {
        SyntaxKind::SELECT => {
            parse_SelectQuery(p);
        }
        SyntaxKind::CONSTRUCT => {
            parse_ConstructQuery(p);
        }
        SyntaxKind::DESCRIBE => {
            parse_DescribeQuery(p);
        }
        SyntaxKind::ASK => {
            parse_AskQuery(p);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    parse_ValuesClause(p);
    p.close(marker, SyntaxKind::Query);
}
/// [2] Prologue -> (BaseDecl | PrefixDecl)*
pub(super) fn parse_Prologue(p: &mut Parser) {
    let marker = p.open();
    while [SyntaxKind::BASE, SyntaxKind::PREFIX].contains(&p.nth(0)) {
        match p.nth(0) {
            SyntaxKind::BASE => {
                parse_BaseDecl(p);
            }
            SyntaxKind::PREFIX => {
                parse_PrefixDecl(p);
            }
            SyntaxKind::Eof => {
                eprintln!("Unexpected Eof");
                p.close(marker, SyntaxKind::Error);
                return;
            }
            _ => {
                p.advance_with_error("Expected ....");
            }
        };
    }
    p.close(marker, SyntaxKind::Prologue);
}
/// [3] SelectQuery -> SelectClause DatasetClause* WhereClause SolutionModifier
pub(super) fn parse_SelectQuery(p: &mut Parser) {
    let marker = p.open();
    parse_SelectClause(p);
    while [SyntaxKind::FROM].contains(&p.nth(0)) {
        parse_DatasetClause(p);
    }
    parse_WhereClause(p);
    parse_SolutionModifier(p);
    p.close(marker, SyntaxKind::SelectQuery);
}
/// [4] ConstructQuery -> 'CONSTRUCT' (ConstructTemplate DatasetClause* WhereClause SolutionModifier | DatasetClause* 'WHERE' '{' TriplesTemplate? '}' SolutionModifier)
pub(super) fn parse_ConstructQuery(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::CONSTRUCT);
    match p.nth(0) {
        SyntaxKind::LCurly => {
            parse_ConstructTemplate(p);
            while [SyntaxKind::FROM].contains(&p.nth(0)) {
                parse_DatasetClause(p);
            }
            parse_WhereClause(p);
            parse_SolutionModifier(p);
        }
        SyntaxKind::WHERE | SyntaxKind::FROM => {
            while [SyntaxKind::FROM].contains(&p.nth(0)) {
                parse_DatasetClause(p);
            }
            p.expect(SyntaxKind::WHERE);
            p.expect(SyntaxKind::LCurly);
            if p.at_any(&[
                SyntaxKind::True,
                SyntaxKind::DECIMAL,
                SyntaxKind::DECIMAL_POSITIVE,
                SyntaxKind::IRIREF,
                SyntaxKind::INTEGER_NEGATIVE,
                SyntaxKind::STRING_LITERAL_LONG2,
                SyntaxKind::STRING_LITERAL_LONG1,
                SyntaxKind::BLANK_NODE_LABEL,
                SyntaxKind::INTEGER_POSITIVE,
                SyntaxKind::LParen,
                SyntaxKind::VAR2,
                SyntaxKind::PNAME_LN,
                SyntaxKind::NIL,
                SyntaxKind::DECIMAL_NEGATIVE,
                SyntaxKind::LBrack,
                SyntaxKind::ANON,
                SyntaxKind::INTEGER,
                SyntaxKind::STRING_LITERAL2,
                SyntaxKind::DOUBLE,
                SyntaxKind::DOUBLE_POSITIVE,
                SyntaxKind::PNAME_NS,
                SyntaxKind::VAR1,
                SyntaxKind::False,
                SyntaxKind::STRING_LITERAL1,
                SyntaxKind::DOUBLE_NEGATIVE,
            ]) {
                parse_TriplesTemplate(p);
            }
            p.expect(SyntaxKind::RCurly);
            parse_SolutionModifier(p);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::ConstructQuery);
}
/// [5] DescribeQuery -> 'DESCRIBE' (VarOrIri VarOrIri* | '*') DatasetClause* WhereClause? SolutionModifier
pub(super) fn parse_DescribeQuery(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::DESCRIBE);
    match p.nth(0) {
        SyntaxKind::PNAME_LN
        | SyntaxKind::IRIREF
        | SyntaxKind::VAR1
        | SyntaxKind::PNAME_NS
        | SyntaxKind::VAR2 => {
            parse_VarOrIri(p);
            while [
                SyntaxKind::VAR1,
                SyntaxKind::PNAME_LN,
                SyntaxKind::IRIREF,
                SyntaxKind::PNAME_NS,
                SyntaxKind::VAR2,
            ]
            .contains(&p.nth(0))
            {
                parse_VarOrIri(p);
            }
        }
        SyntaxKind::Star => {
            p.expect(SyntaxKind::Star);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    while [SyntaxKind::FROM].contains(&p.nth(0)) {
        parse_DatasetClause(p);
    }
    if p.at_any(&[SyntaxKind::WHERE, SyntaxKind::LCurly]) {
        parse_WhereClause(p);
    }
    parse_SolutionModifier(p);
    p.close(marker, SyntaxKind::DescribeQuery);
}
/// [6] AskQuery -> 'ASK' DatasetClause* WhereClause SolutionModifier
pub(super) fn parse_AskQuery(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::ASK);
    while [SyntaxKind::FROM].contains(&p.nth(0)) {
        parse_DatasetClause(p);
    }
    parse_WhereClause(p);
    parse_SolutionModifier(p);
    p.close(marker, SyntaxKind::AskQuery);
}
/// [7] ValuesClause -> ('VALUES' DataBlock)?
pub(super) fn parse_ValuesClause(p: &mut Parser) {
    let marker = p.open();
    if p.at_any(&[SyntaxKind::VALUES]) {
        p.expect(SyntaxKind::VALUES);
        parse_DataBlock(p);
    }
    p.close(marker, SyntaxKind::ValuesClause);
}
/// [8] UpdateUnit -> Update
pub(super) fn parse_UpdateUnit(p: &mut Parser) {
    let marker = p.open();
    parse_Update(p);
    p.close(marker, SyntaxKind::UpdateUnit);
}
/// [9] Update -> Prologue (UpdateOne (';' Update)?)?
pub(super) fn parse_Update(p: &mut Parser) {
    let marker = p.open();
    parse_Prologue(p);
    if p.at_any(&[
        SyntaxKind::ADD,
        SyntaxKind::INSERT_DATA,
        SyntaxKind::DELETE_WHERE,
        SyntaxKind::CREATE,
        SyntaxKind::DROP,
        SyntaxKind::MOVE,
        SyntaxKind::WITH,
        SyntaxKind::CLEAR,
        SyntaxKind::COPY,
        SyntaxKind::DELETE,
        SyntaxKind::INSERT,
        SyntaxKind::LOAD,
        SyntaxKind::DELETE_DATA,
    ]) {
        parse_UpdateOne(p);
        if p.at_any(&[SyntaxKind::Semicolon]) {
            p.expect(SyntaxKind::Semicolon);
            parse_Update(p);
        }
    }
    p.close(marker, SyntaxKind::Update);
}
/// [10] BaseDecl -> 'BASE' 'IRIREF'
pub(super) fn parse_BaseDecl(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::BASE);
    p.expect(SyntaxKind::IRIREF);
    p.close(marker, SyntaxKind::BaseDecl);
}
/// [11] PrefixDecl -> 'PREFIX' 'PNAME_NS' 'IRIREF'
pub(super) fn parse_PrefixDecl(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::PREFIX);
    p.expect(SyntaxKind::PNAME_NS);
    p.expect(SyntaxKind::IRIREF);
    p.close(marker, SyntaxKind::PrefixDecl);
}
/// [12] SelectClause -> 'SELECT' ('DISTINCT' | 'REDUCED')? ((Var | '(' Expression 'AS' Var ')') (Var | '(' Expression 'AS' Var ')')* | '*')
pub(super) fn parse_SelectClause(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::SELECT);
    if p.at_any(&[SyntaxKind::REDUCED, SyntaxKind::DISTINCT]) {
        match p.nth(0) {
            SyntaxKind::DISTINCT => {
                p.expect(SyntaxKind::DISTINCT);
            }
            SyntaxKind::REDUCED => {
                p.expect(SyntaxKind::REDUCED);
            }
            SyntaxKind::Eof => {
                eprintln!("Unexpected Eof");
                p.close(marker, SyntaxKind::Error);
                return;
            }
            _ => {
                p.advance_with_error("Expected ....");
            }
        };
    }
    match p.nth(0) {
        SyntaxKind::VAR2 | SyntaxKind::VAR1 | SyntaxKind::LParen => {
            match p.nth(0) {
                SyntaxKind::VAR1 | SyntaxKind::VAR2 => {
                    parse_Var(p);
                }
                SyntaxKind::LParen => {
                    p.expect(SyntaxKind::LParen);
                    parse_Expression(p);
                    p.expect(SyntaxKind::AS);
                    parse_Var(p);
                    p.expect(SyntaxKind::RParen);
                }
                SyntaxKind::Eof => {
                    eprintln!("Unexpected Eof");
                    p.close(marker, SyntaxKind::Error);
                    return;
                }
                _ => {
                    p.advance_with_error("Expected ....");
                }
            };
            while [SyntaxKind::VAR2, SyntaxKind::VAR1, SyntaxKind::LParen].contains(&p.nth(0)) {
                match p.nth(0) {
                    SyntaxKind::VAR1 | SyntaxKind::VAR2 => {
                        parse_Var(p);
                    }
                    SyntaxKind::LParen => {
                        p.expect(SyntaxKind::LParen);
                        parse_Expression(p);
                        p.expect(SyntaxKind::AS);
                        parse_Var(p);
                        p.expect(SyntaxKind::RParen);
                    }
                    SyntaxKind::Eof => {
                        eprintln!("Unexpected Eof");
                        p.close(marker, SyntaxKind::Error);
                        return;
                    }
                    _ => {
                        p.advance_with_error("Expected ....");
                    }
                };
            }
        }
        SyntaxKind::Star => {
            p.expect(SyntaxKind::Star);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::SelectClause);
}
/// [13] DatasetClause -> 'FROM' (DefaultGraphClause | NamedGraphClause)
pub(super) fn parse_DatasetClause(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::FROM);
    match p.nth(0) {
        SyntaxKind::PNAME_LN | SyntaxKind::IRIREF | SyntaxKind::PNAME_NS => {
            parse_DefaultGraphClause(p);
        }
        SyntaxKind::NAMED => {
            parse_NamedGraphClause(p);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::DatasetClause);
}
/// [14] WhereClause -> 'WHERE'? GroupGraphPattern
pub(super) fn parse_WhereClause(p: &mut Parser) {
    let marker = p.open();
    if p.at_any(&[SyntaxKind::WHERE]) {
        p.expect(SyntaxKind::WHERE);
    }
    parse_GroupGraphPattern(p);
    p.close(marker, SyntaxKind::WhereClause);
}
/// [15] SolutionModifier -> GroupClause? HavingClause? OrderClause? LimitOffsetClauses?
pub(super) fn parse_SolutionModifier(p: &mut Parser) {
    let marker = p.open();
    if p.at_any(&[SyntaxKind::GROUP]) {
        parse_GroupClause(p);
    }
    if p.at_any(&[SyntaxKind::HAVING]) {
        parse_HavingClause(p);
    }
    if p.at_any(&[SyntaxKind::ORDER]) {
        parse_OrderClause(p);
    }
    if p.at_any(&[SyntaxKind::LIMIT, SyntaxKind::OFFSET]) {
        parse_LimitOffsetClauses(p);
    }
    p.close(marker, SyntaxKind::SolutionModifier);
}
/// [16] SubSelect -> SelectClause WhereClause SolutionModifier ValuesClause
pub(super) fn parse_SubSelect(p: &mut Parser) {
    let marker = p.open();
    parse_SelectClause(p);
    parse_WhereClause(p);
    parse_SolutionModifier(p);
    parse_ValuesClause(p);
    p.close(marker, SyntaxKind::SubSelect);
}
/// [17] Var -> 'VAR1' | 'VAR2'
pub(super) fn parse_Var(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::VAR1 => {
            p.expect(SyntaxKind::VAR1);
        }
        SyntaxKind::VAR2 => {
            p.expect(SyntaxKind::VAR2);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::Var);
}
/// [18] Expression -> ConditionalOrExpression
pub(super) fn parse_Expression(p: &mut Parser) {
    let marker = p.open();
    parse_ConditionalOrExpression(p);
    p.close(marker, SyntaxKind::Expression);
}
/// [19] ConstructTemplate -> '{' ConstructTriples? '}'
pub(super) fn parse_ConstructTemplate(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::LCurly);
    if p.at_any(&[
        SyntaxKind::DECIMAL_NEGATIVE,
        SyntaxKind::DOUBLE,
        SyntaxKind::STRING_LITERAL1,
        SyntaxKind::DOUBLE_NEGATIVE,
        SyntaxKind::IRIREF,
        SyntaxKind::True,
        SyntaxKind::DECIMAL,
        SyntaxKind::INTEGER_NEGATIVE,
        SyntaxKind::STRING_LITERAL_LONG2,
        SyntaxKind::STRING_LITERAL_LONG1,
        SyntaxKind::ANON,
        SyntaxKind::DOUBLE_POSITIVE,
        SyntaxKind::DECIMAL_POSITIVE,
        SyntaxKind::PNAME_NS,
        SyntaxKind::False,
        SyntaxKind::LBrack,
        SyntaxKind::VAR2,
        SyntaxKind::NIL,
        SyntaxKind::BLANK_NODE_LABEL,
        SyntaxKind::INTEGER_POSITIVE,
        SyntaxKind::VAR1,
        SyntaxKind::LParen,
        SyntaxKind::STRING_LITERAL2,
        SyntaxKind::INTEGER,
        SyntaxKind::PNAME_LN,
    ]) {
        parse_ConstructTriples(p);
    }
    p.expect(SyntaxKind::RCurly);
    p.close(marker, SyntaxKind::ConstructTemplate);
}
/// [20] TriplesTemplate -> TriplesSameSubject ('.' TriplesTemplate?)?
pub(super) fn parse_TriplesTemplate(p: &mut Parser) {
    let marker = p.open();
    parse_TriplesSameSubject(p);
    if p.at_any(&[SyntaxKind::Dot]) {
        p.expect(SyntaxKind::Dot);
        if p.at_any(&[
            SyntaxKind::True,
            SyntaxKind::DECIMAL,
            SyntaxKind::DECIMAL_POSITIVE,
            SyntaxKind::IRIREF,
            SyntaxKind::INTEGER_NEGATIVE,
            SyntaxKind::STRING_LITERAL_LONG2,
            SyntaxKind::STRING_LITERAL_LONG1,
            SyntaxKind::BLANK_NODE_LABEL,
            SyntaxKind::INTEGER_POSITIVE,
            SyntaxKind::LParen,
            SyntaxKind::VAR2,
            SyntaxKind::PNAME_LN,
            SyntaxKind::NIL,
            SyntaxKind::DECIMAL_NEGATIVE,
            SyntaxKind::LBrack,
            SyntaxKind::ANON,
            SyntaxKind::INTEGER,
            SyntaxKind::STRING_LITERAL2,
            SyntaxKind::DOUBLE,
            SyntaxKind::DOUBLE_POSITIVE,
            SyntaxKind::PNAME_NS,
            SyntaxKind::VAR1,
            SyntaxKind::False,
            SyntaxKind::STRING_LITERAL1,
            SyntaxKind::DOUBLE_NEGATIVE,
        ]) {
            parse_TriplesTemplate(p);
        }
    }
    p.close(marker, SyntaxKind::TriplesTemplate);
}
/// [21] VarOrIri -> Var | iri
pub(super) fn parse_VarOrIri(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::VAR1 | SyntaxKind::VAR2 => {
            parse_Var(p);
        }
        SyntaxKind::PNAME_LN | SyntaxKind::IRIREF | SyntaxKind::PNAME_NS => {
            parse_iri(p);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::VarOrIri);
}
/// [22] DefaultGraphClause -> SourceSelector
pub(super) fn parse_DefaultGraphClause(p: &mut Parser) {
    let marker = p.open();
    parse_SourceSelector(p);
    p.close(marker, SyntaxKind::DefaultGraphClause);
}
/// [23] NamedGraphClause -> 'NAMED' SourceSelector
pub(super) fn parse_NamedGraphClause(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::NAMED);
    parse_SourceSelector(p);
    p.close(marker, SyntaxKind::NamedGraphClause);
}
/// [24] SourceSelector -> iri
pub(super) fn parse_SourceSelector(p: &mut Parser) {
    let marker = p.open();
    parse_iri(p);
    p.close(marker, SyntaxKind::SourceSelector);
}
/// [25] iri -> 'IRIREF' | PrefixedName
pub(super) fn parse_iri(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::IRIREF => {
            p.expect(SyntaxKind::IRIREF);
        }
        SyntaxKind::PNAME_LN | SyntaxKind::PNAME_NS => {
            parse_PrefixedName(p);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::iri);
}
/// [26] GroupGraphPattern -> '{' (SubSelect | GroupGraphPatternSub) '}'
pub(super) fn parse_GroupGraphPattern(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::LCurly);
    match p.nth(0) {
        SyntaxKind::SELECT => {
            parse_SubSelect(p);
        }
        SyntaxKind::BIND
        | SyntaxKind::NIL
        | SyntaxKind::OPTIONAL
        | SyntaxKind::GRAPH
        | SyntaxKind::ANON
        | SyntaxKind::LCurly
        | SyntaxKind::MINUS
        | SyntaxKind::LBrack
        | SyntaxKind::STRING_LITERAL1
        | SyntaxKind::INTEGER_NEGATIVE
        | SyntaxKind::LParen
        | SyntaxKind::PNAME_NS
        | SyntaxKind::STRING_LITERAL_LONG1
        | SyntaxKind::True
        | SyntaxKind::BLANK_NODE_LABEL
        | SyntaxKind::VALUES
        | SyntaxKind::SERVICE
        | SyntaxKind::IRIREF
        | SyntaxKind::STRING_LITERAL2
        | SyntaxKind::DECIMAL_NEGATIVE
        | SyntaxKind::DOUBLE
        | SyntaxKind::DOUBLE_NEGATIVE
        | SyntaxKind::False
        | SyntaxKind::DOUBLE_POSITIVE
        | SyntaxKind::PNAME_LN
        | SyntaxKind::VAR2
        | SyntaxKind::DECIMAL_POSITIVE
        | SyntaxKind::INTEGER_POSITIVE
        | SyntaxKind::STRING_LITERAL_LONG2
        | SyntaxKind::INTEGER
        | SyntaxKind::FILTER
        | SyntaxKind::DECIMAL
        | SyntaxKind::VAR1 => {
            parse_GroupGraphPatternSub(p);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {}
    };
    p.expect(SyntaxKind::RCurly);
    p.close(marker, SyntaxKind::GroupGraphPattern);
}
/// [27] GroupClause -> 'GROUP' 'BY' GroupCondition GroupCondition*
pub(super) fn parse_GroupClause(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::GROUP);
    p.expect(SyntaxKind::BY);
    parse_GroupCondition(p);
    while [
        SyntaxKind::VAR2,
        SyntaxKind::STRSTARTS,
        SyntaxKind::STRUUID,
        SyntaxKind::LParen,
        SyntaxKind::STRDT,
        SyntaxKind::MIN,
        SyntaxKind::IRI,
        SyntaxKind::NOW,
        SyntaxKind::CONTAINS,
        SyntaxKind::IRIREF,
        SyntaxKind::isNUMERIC,
        SyntaxKind::GROUP_CONCAT,
        SyntaxKind::LANGMATCHES,
        SyntaxKind::isLITERAL,
        SyntaxKind::MAX,
        SyntaxKind::REGEX,
        SyntaxKind::isURI,
        SyntaxKind::MONTH,
        SyntaxKind::STR,
        SyntaxKind::ABS,
        SyntaxKind::COALESCE,
        SyntaxKind::UCASE,
        SyntaxKind::FLOOR,
        SyntaxKind::HOURS,
        SyntaxKind::SHA384,
        SyntaxKind::TZ,
        SyntaxKind::SUBSTR,
        SyntaxKind::ENCODE_FOR_URI,
        SyntaxKind::STRLANG,
        SyntaxKind::UUID,
        SyntaxKind::RAND,
        SyntaxKind::STRENDS,
        SyntaxKind::PNAME_LN,
        SyntaxKind::SECONDS,
        SyntaxKind::ROUND,
        SyntaxKind::sameTerm,
        SyntaxKind::BNODE,
        SyntaxKind::AVG,
        SyntaxKind::LCASE,
        SyntaxKind::DAY,
        SyntaxKind::LANG,
        SyntaxKind::NOT,
        SyntaxKind::COUNT,
        SyntaxKind::STRLEN,
        SyntaxKind::MD5,
        SyntaxKind::YEAR,
        SyntaxKind::MINUTES,
        SyntaxKind::TIMEZONE,
        SyntaxKind::SAMPLE,
        SyntaxKind::SUM,
        SyntaxKind::REPLACE,
        SyntaxKind::isIRI,
        SyntaxKind::SHA512,
        SyntaxKind::SHA256,
        SyntaxKind::STRAFTER,
        SyntaxKind::CONCAT,
        SyntaxKind::URI,
        SyntaxKind::PNAME_NS,
        SyntaxKind::IF,
        SyntaxKind::CEIL,
        SyntaxKind::BOUND,
        SyntaxKind::SHA1,
        SyntaxKind::STRBEFORE,
        SyntaxKind::VAR1,
        SyntaxKind::isBLANK,
        SyntaxKind::DATATYPE,
        SyntaxKind::EXISTS,
    ]
    .contains(&p.nth(0))
    {
        parse_GroupCondition(p);
    }
    p.close(marker, SyntaxKind::GroupClause);
}
/// [28] HavingClause -> 'HAVING' HavingCondition HavingCondition*
pub(super) fn parse_HavingClause(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::HAVING);
    parse_HavingCondition(p);
    while [
        SyntaxKind::IRI,
        SyntaxKind::IF,
        SyntaxKind::LANGMATCHES,
        SyntaxKind::CEIL,
        SyntaxKind::ROUND,
        SyntaxKind::STRAFTER,
        SyntaxKind::FLOOR,
        SyntaxKind::STRLEN,
        SyntaxKind::MONTH,
        SyntaxKind::COALESCE,
        SyntaxKind::isIRI,
        SyntaxKind::AVG,
        SyntaxKind::GROUP_CONCAT,
        SyntaxKind::sameTerm,
        SyntaxKind::ENCODE_FOR_URI,
        SyntaxKind::SHA256,
        SyntaxKind::SUM,
        SyntaxKind::RAND,
        SyntaxKind::HOURS,
        SyntaxKind::STRUUID,
        SyntaxKind::MINUTES,
        SyntaxKind::SUBSTR,
        SyntaxKind::CONCAT,
        SyntaxKind::NOW,
        SyntaxKind::STRDT,
        SyntaxKind::DAY,
        SyntaxKind::REGEX,
        SyntaxKind::LParen,
        SyntaxKind::LANG,
        SyntaxKind::TIMEZONE,
        SyntaxKind::isURI,
        SyntaxKind::DATATYPE,
        SyntaxKind::MAX,
        SyntaxKind::UUID,
        SyntaxKind::isNUMERIC,
        SyntaxKind::REPLACE,
        SyntaxKind::TZ,
        SyntaxKind::SHA384,
        SyntaxKind::isBLANK,
        SyntaxKind::MD5,
        SyntaxKind::SECONDS,
        SyntaxKind::MIN,
        SyntaxKind::COUNT,
        SyntaxKind::STRLANG,
        SyntaxKind::SAMPLE,
        SyntaxKind::ABS,
        SyntaxKind::STR,
        SyntaxKind::IRIREF,
        SyntaxKind::URI,
        SyntaxKind::STRBEFORE,
        SyntaxKind::YEAR,
        SyntaxKind::BNODE,
        SyntaxKind::CONTAINS,
        SyntaxKind::EXISTS,
        SyntaxKind::NOT,
        SyntaxKind::STRSTARTS,
        SyntaxKind::STRENDS,
        SyntaxKind::UCASE,
        SyntaxKind::LCASE,
        SyntaxKind::SHA512,
        SyntaxKind::PNAME_NS,
        SyntaxKind::isLITERAL,
        SyntaxKind::SHA1,
        SyntaxKind::BOUND,
        SyntaxKind::PNAME_LN,
    ]
    .contains(&p.nth(0))
    {
        parse_HavingCondition(p);
    }
    p.close(marker, SyntaxKind::HavingClause);
}
/// [29] OrderClause -> 'ORDER' 'BY' OrderCondition OrderCondition*
pub(super) fn parse_OrderClause(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::ORDER);
    p.expect(SyntaxKind::BY);
    parse_OrderCondition(p);
    while [
        SyntaxKind::VAR1,
        SyntaxKind::NOT,
        SyntaxKind::isIRI,
        SyntaxKind::REGEX,
        SyntaxKind::COUNT,
        SyntaxKind::STRBEFORE,
        SyntaxKind::COALESCE,
        SyntaxKind::DATATYPE,
        SyntaxKind::FLOOR,
        SyntaxKind::AVG,
        SyntaxKind::isURI,
        SyntaxKind::SUBSTR,
        SyntaxKind::MONTH,
        SyntaxKind::IRIREF,
        SyntaxKind::SAMPLE,
        SyntaxKind::STRSTARTS,
        SyntaxKind::VAR2,
        SyntaxKind::EXISTS,
        SyntaxKind::UUID,
        SyntaxKind::URI,
        SyntaxKind::STRUUID,
        SyntaxKind::CEIL,
        SyntaxKind::MINUTES,
        SyntaxKind::TZ,
        SyntaxKind::isNUMERIC,
        SyntaxKind::isLITERAL,
        SyntaxKind::MAX,
        SyntaxKind::STRLANG,
        SyntaxKind::GROUP_CONCAT,
        SyntaxKind::SHA256,
        SyntaxKind::STRENDS,
        SyntaxKind::SECONDS,
        SyntaxKind::TIMEZONE,
        SyntaxKind::sameTerm,
        SyntaxKind::STR,
        SyntaxKind::UCASE,
        SyntaxKind::IF,
        SyntaxKind::LParen,
        SyntaxKind::REPLACE,
        SyntaxKind::PNAME_NS,
        SyntaxKind::PNAME_LN,
        SyntaxKind::YEAR,
        SyntaxKind::STRDT,
        SyntaxKind::ROUND,
        SyntaxKind::MD5,
        SyntaxKind::CONCAT,
        SyntaxKind::SHA512,
        SyntaxKind::SHA384,
        SyntaxKind::DESC,
        SyntaxKind::LANGMATCHES,
        SyntaxKind::STRAFTER,
        SyntaxKind::CONTAINS,
        SyntaxKind::DAY,
        SyntaxKind::LANG,
        SyntaxKind::STRLEN,
        SyntaxKind::IRI,
        SyntaxKind::ASC,
        SyntaxKind::ABS,
        SyntaxKind::NOW,
        SyntaxKind::ENCODE_FOR_URI,
        SyntaxKind::SHA1,
        SyntaxKind::HOURS,
        SyntaxKind::RAND,
        SyntaxKind::SUM,
        SyntaxKind::LCASE,
        SyntaxKind::MIN,
        SyntaxKind::BOUND,
        SyntaxKind::BNODE,
        SyntaxKind::isBLANK,
    ]
    .contains(&p.nth(0))
    {
        parse_OrderCondition(p);
    }
    p.close(marker, SyntaxKind::OrderClause);
}
/// [30] LimitOffsetClauses -> LimitClause OffsetClause? | OffsetClause LimitClause?
pub(super) fn parse_LimitOffsetClauses(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::LIMIT => {
            parse_LimitClause(p);
            if p.at_any(&[SyntaxKind::OFFSET]) {
                parse_OffsetClause(p);
            }
        }
        SyntaxKind::OFFSET => {
            parse_OffsetClause(p);
            if p.at_any(&[SyntaxKind::LIMIT]) {
                parse_LimitClause(p);
            }
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::LimitOffsetClauses);
}
/// [31] GroupCondition -> BuiltInCall | FunctionCall | '(' Expression ('AS' Var)? ')' | Var
pub(super) fn parse_GroupCondition(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::DATATYPE
        | SyntaxKind::STR
        | SyntaxKind::COUNT
        | SyntaxKind::URI
        | SyntaxKind::STRSTARTS
        | SyntaxKind::SHA256
        | SyntaxKind::GROUP_CONCAT
        | SyntaxKind::UCASE
        | SyntaxKind::SHA384
        | SyntaxKind::ABS
        | SyntaxKind::STRAFTER
        | SyntaxKind::STRLANG
        | SyntaxKind::LANGMATCHES
        | SyntaxKind::STRENDS
        | SyntaxKind::IF
        | SyntaxKind::sameTerm
        | SyntaxKind::isIRI
        | SyntaxKind::isLITERAL
        | SyntaxKind::NOT
        | SyntaxKind::MD5
        | SyntaxKind::SUBSTR
        | SyntaxKind::CEIL
        | SyntaxKind::BNODE
        | SyntaxKind::NOW
        | SyntaxKind::STRDT
        | SyntaxKind::isNUMERIC
        | SyntaxKind::STRLEN
        | SyntaxKind::ENCODE_FOR_URI
        | SyntaxKind::SHA1
        | SyntaxKind::SECONDS
        | SyntaxKind::LANG
        | SyntaxKind::AVG
        | SyntaxKind::DAY
        | SyntaxKind::ROUND
        | SyntaxKind::IRI
        | SyntaxKind::SUM
        | SyntaxKind::MAX
        | SyntaxKind::FLOOR
        | SyntaxKind::COALESCE
        | SyntaxKind::REGEX
        | SyntaxKind::CONTAINS
        | SyntaxKind::UUID
        | SyntaxKind::isURI
        | SyntaxKind::TIMEZONE
        | SyntaxKind::YEAR
        | SyntaxKind::RAND
        | SyntaxKind::TZ
        | SyntaxKind::STRBEFORE
        | SyntaxKind::LCASE
        | SyntaxKind::isBLANK
        | SyntaxKind::MONTH
        | SyntaxKind::HOURS
        | SyntaxKind::REPLACE
        | SyntaxKind::CONCAT
        | SyntaxKind::STRUUID
        | SyntaxKind::EXISTS
        | SyntaxKind::BOUND
        | SyntaxKind::SHA512
        | SyntaxKind::MIN
        | SyntaxKind::SAMPLE
        | SyntaxKind::MINUTES => {
            parse_BuiltInCall(p);
        }
        SyntaxKind::PNAME_NS | SyntaxKind::PNAME_LN | SyntaxKind::IRIREF => {
            parse_FunctionCall(p);
        }
        SyntaxKind::LParen => {
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            if p.at_any(&[SyntaxKind::AS]) {
                p.expect(SyntaxKind::AS);
                parse_Var(p);
            }
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::VAR1 | SyntaxKind::VAR2 => {
            parse_Var(p);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::GroupCondition);
}
/// [32] BuiltInCall -> Aggregate | 'STR' '(' Expression ')' | 'LANG' '(' Expression ')' | 'LANGMATCHES' '(' Expression ',' Expression ')' | 'DATATYPE' '(' Expression ')' | 'BOUND' '(' Var ')' | 'IRI' '(' Expression ')' | 'URI' '(' Expression ')' | 'BNODE' ('(' Expression ')' | 'NIL') | 'RAND' 'NIL' | 'ABS' '(' Expression ')' | 'CEIL' '(' Expression ')' | 'FLOOR' '(' Expression ')' | 'ROUND' '(' Expression ')' | 'CONCAT' ExpressionList | SubstringExpression | 'STRLEN' '(' Expression ')' | StrReplaceExpression | 'UCASE' '(' Expression ')' | 'LCASE' '(' Expression ')' | 'ENCODE_FOR_URI' '(' Expression ')' | 'CONTAINS' '(' Expression ',' Expression ')' | 'STRSTARTS' '(' Expression ',' Expression ')' | 'STRENDS' '(' Expression ',' Expression ')' | 'STRBEFORE' '(' Expression ',' Expression ')' | 'STRAFTER' '(' Expression ',' Expression ')' | 'YEAR' '(' Expression ')' | 'MONTH' '(' Expression ')' | 'DAY' '(' Expression ')' | 'HOURS' '(' Expression ')' | 'MINUTES' '(' Expression ')' | 'SECONDS' '(' Expression ')' | 'TIMEZONE' '(' Expression ')' | 'TZ' '(' Expression ')' | 'NOW' 'NIL' | 'UUID' 'NIL' | 'STRUUID' 'NIL' | 'MD5' '(' Expression ')' | 'SHA1' '(' Expression ')' | 'SHA256' '(' Expression ')' | 'SHA384' '(' Expression ')' | 'SHA512' '(' Expression ')' | 'COALESCE' ExpressionList | 'IF' '(' Expression ',' Expression ',' Expression ')' | 'STRLANG' '(' Expression ',' Expression ')' | 'STRDT' '(' Expression ',' Expression ')' | 'sameTerm' '(' Expression ',' Expression ')' | 'isIRI' '(' Expression ')' | 'isURI' '(' Expression ')' | 'isBLANK' '(' Expression ')' | 'isLITERAL' '(' Expression ')' | 'isNUMERIC' '(' Expression ')' | RegexExpression | ExistsFunc | NotExistsFunc
pub(super) fn parse_BuiltInCall(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::COUNT
        | SyntaxKind::MIN
        | SyntaxKind::MAX
        | SyntaxKind::AVG
        | SyntaxKind::GROUP_CONCAT
        | SyntaxKind::SAMPLE
        | SyntaxKind::SUM => {
            parse_Aggregate(p);
        }
        SyntaxKind::STR => {
            p.expect(SyntaxKind::STR);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::LANG => {
            p.expect(SyntaxKind::LANG);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::LANGMATCHES => {
            p.expect(SyntaxKind::LANGMATCHES);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::Colon);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::DATATYPE => {
            p.expect(SyntaxKind::DATATYPE);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::BOUND => {
            p.expect(SyntaxKind::BOUND);
            p.expect(SyntaxKind::LParen);
            parse_Var(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::IRI => {
            p.expect(SyntaxKind::IRI);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::URI => {
            p.expect(SyntaxKind::URI);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::BNODE => {
            p.expect(SyntaxKind::BNODE);
            match p.nth(0) {
                SyntaxKind::LParen => {
                    p.expect(SyntaxKind::LParen);
                    parse_Expression(p);
                    p.expect(SyntaxKind::RParen);
                }
                SyntaxKind::NIL => {
                    p.expect(SyntaxKind::NIL);
                }
                SyntaxKind::Eof => {
                    eprintln!("Unexpected Eof");
                    p.close(marker, SyntaxKind::Error);
                    return;
                }
                _ => {
                    p.advance_with_error("Expected ....");
                }
            };
        }
        SyntaxKind::RAND => {
            p.expect(SyntaxKind::RAND);
            p.expect(SyntaxKind::NIL);
        }
        SyntaxKind::ABS => {
            p.expect(SyntaxKind::ABS);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::CEIL => {
            p.expect(SyntaxKind::CEIL);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::FLOOR => {
            p.expect(SyntaxKind::FLOOR);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::ROUND => {
            p.expect(SyntaxKind::ROUND);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::CONCAT => {
            p.expect(SyntaxKind::CONCAT);
            parse_ExpressionList(p);
        }
        SyntaxKind::SUBSTR => {
            parse_SubstringExpression(p);
        }
        SyntaxKind::STRLEN => {
            p.expect(SyntaxKind::STRLEN);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::REPLACE => {
            parse_StrReplaceExpression(p);
        }
        SyntaxKind::UCASE => {
            p.expect(SyntaxKind::UCASE);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::LCASE => {
            p.expect(SyntaxKind::LCASE);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::ENCODE_FOR_URI => {
            p.expect(SyntaxKind::ENCODE_FOR_URI);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::CONTAINS => {
            p.expect(SyntaxKind::CONTAINS);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::Colon);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::STRSTARTS => {
            p.expect(SyntaxKind::STRSTARTS);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::Colon);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::STRENDS => {
            p.expect(SyntaxKind::STRENDS);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::Colon);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::STRBEFORE => {
            p.expect(SyntaxKind::STRBEFORE);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::Colon);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::STRAFTER => {
            p.expect(SyntaxKind::STRAFTER);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::Colon);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::YEAR => {
            p.expect(SyntaxKind::YEAR);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::MONTH => {
            p.expect(SyntaxKind::MONTH);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::DAY => {
            p.expect(SyntaxKind::DAY);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::HOURS => {
            p.expect(SyntaxKind::HOURS);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::MINUTES => {
            p.expect(SyntaxKind::MINUTES);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::SECONDS => {
            p.expect(SyntaxKind::SECONDS);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::TIMEZONE => {
            p.expect(SyntaxKind::TIMEZONE);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::TZ => {
            p.expect(SyntaxKind::TZ);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::NOW => {
            p.expect(SyntaxKind::NOW);
            p.expect(SyntaxKind::NIL);
        }
        SyntaxKind::UUID => {
            p.expect(SyntaxKind::UUID);
            p.expect(SyntaxKind::NIL);
        }
        SyntaxKind::STRUUID => {
            p.expect(SyntaxKind::STRUUID);
            p.expect(SyntaxKind::NIL);
        }
        SyntaxKind::MD5 => {
            p.expect(SyntaxKind::MD5);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::SHA1 => {
            p.expect(SyntaxKind::SHA1);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::SHA256 => {
            p.expect(SyntaxKind::SHA256);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::SHA384 => {
            p.expect(SyntaxKind::SHA384);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::SHA512 => {
            p.expect(SyntaxKind::SHA512);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::COALESCE => {
            p.expect(SyntaxKind::COALESCE);
            parse_ExpressionList(p);
        }
        SyntaxKind::IF => {
            p.expect(SyntaxKind::IF);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::Colon);
            parse_Expression(p);
            p.expect(SyntaxKind::Colon);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::STRLANG => {
            p.expect(SyntaxKind::STRLANG);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::Colon);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::STRDT => {
            p.expect(SyntaxKind::STRDT);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::Colon);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::sameTerm => {
            p.expect(SyntaxKind::sameTerm);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::Colon);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::isIRI => {
            p.expect(SyntaxKind::isIRI);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::isURI => {
            p.expect(SyntaxKind::isURI);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::isBLANK => {
            p.expect(SyntaxKind::isBLANK);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::isLITERAL => {
            p.expect(SyntaxKind::isLITERAL);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::isNUMERIC => {
            p.expect(SyntaxKind::isNUMERIC);
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::REGEX => {
            parse_RegexExpression(p);
        }
        SyntaxKind::EXISTS => {
            parse_ExistsFunc(p);
        }
        SyntaxKind::NOT => {
            parse_NotExistsFunc(p);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::BuiltInCall);
}
/// [33] FunctionCall -> iri ArgList
pub(super) fn parse_FunctionCall(p: &mut Parser) {
    let marker = p.open();
    parse_iri(p);
    parse_ArgList(p);
    p.close(marker, SyntaxKind::FunctionCall);
}
/// [34] HavingCondition -> Constraint
pub(super) fn parse_HavingCondition(p: &mut Parser) {
    let marker = p.open();
    parse_Constraint(p);
    p.close(marker, SyntaxKind::HavingCondition);
}
/// [35] Constraint -> BrackettedExpression | BuiltInCall | FunctionCall
pub(super) fn parse_Constraint(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::LParen => {
            parse_BrackettedExpression(p);
        }
        SyntaxKind::DATATYPE
        | SyntaxKind::STR
        | SyntaxKind::COUNT
        | SyntaxKind::URI
        | SyntaxKind::STRSTARTS
        | SyntaxKind::SHA256
        | SyntaxKind::GROUP_CONCAT
        | SyntaxKind::UCASE
        | SyntaxKind::SHA384
        | SyntaxKind::ABS
        | SyntaxKind::STRAFTER
        | SyntaxKind::STRLANG
        | SyntaxKind::LANGMATCHES
        | SyntaxKind::STRENDS
        | SyntaxKind::IF
        | SyntaxKind::sameTerm
        | SyntaxKind::isIRI
        | SyntaxKind::isLITERAL
        | SyntaxKind::NOT
        | SyntaxKind::MD5
        | SyntaxKind::SUBSTR
        | SyntaxKind::CEIL
        | SyntaxKind::BNODE
        | SyntaxKind::NOW
        | SyntaxKind::STRDT
        | SyntaxKind::isNUMERIC
        | SyntaxKind::STRLEN
        | SyntaxKind::ENCODE_FOR_URI
        | SyntaxKind::SHA1
        | SyntaxKind::SECONDS
        | SyntaxKind::LANG
        | SyntaxKind::AVG
        | SyntaxKind::DAY
        | SyntaxKind::ROUND
        | SyntaxKind::IRI
        | SyntaxKind::SUM
        | SyntaxKind::MAX
        | SyntaxKind::FLOOR
        | SyntaxKind::COALESCE
        | SyntaxKind::REGEX
        | SyntaxKind::CONTAINS
        | SyntaxKind::UUID
        | SyntaxKind::isURI
        | SyntaxKind::TIMEZONE
        | SyntaxKind::YEAR
        | SyntaxKind::RAND
        | SyntaxKind::TZ
        | SyntaxKind::STRBEFORE
        | SyntaxKind::LCASE
        | SyntaxKind::isBLANK
        | SyntaxKind::MONTH
        | SyntaxKind::HOURS
        | SyntaxKind::REPLACE
        | SyntaxKind::CONCAT
        | SyntaxKind::STRUUID
        | SyntaxKind::EXISTS
        | SyntaxKind::BOUND
        | SyntaxKind::SHA512
        | SyntaxKind::MIN
        | SyntaxKind::SAMPLE
        | SyntaxKind::MINUTES => {
            parse_BuiltInCall(p);
        }
        SyntaxKind::PNAME_NS | SyntaxKind::PNAME_LN | SyntaxKind::IRIREF => {
            parse_FunctionCall(p);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::Constraint);
}
/// [36] OrderCondition -> ('ASC' | 'DESC') BrackettedExpression | Constraint | Var
pub(super) fn parse_OrderCondition(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::ASC | SyntaxKind::DESC => {
            match p.nth(0) {
                SyntaxKind::ASC => {
                    p.expect(SyntaxKind::ASC);
                }
                SyntaxKind::DESC => {
                    p.expect(SyntaxKind::DESC);
                }
                SyntaxKind::Eof => {
                    eprintln!("Unexpected Eof");
                    p.close(marker, SyntaxKind::Error);
                    return;
                }
                _ => {
                    p.advance_with_error("Expected ....");
                }
            };
            parse_BrackettedExpression(p);
        }
        SyntaxKind::TIMEZONE
        | SyntaxKind::SAMPLE
        | SyntaxKind::isLITERAL
        | SyntaxKind::DATATYPE
        | SyntaxKind::STRDT
        | SyntaxKind::ABS
        | SyntaxKind::STRUUID
        | SyntaxKind::UCASE
        | SyntaxKind::STRBEFORE
        | SyntaxKind::COUNT
        | SyntaxKind::YEAR
        | SyntaxKind::SHA512
        | SyntaxKind::sameTerm
        | SyntaxKind::LANG
        | SyntaxKind::isNUMERIC
        | SyntaxKind::TZ
        | SyntaxKind::SECONDS
        | SyntaxKind::CEIL
        | SyntaxKind::MINUTES
        | SyntaxKind::ROUND
        | SyntaxKind::CONTAINS
        | SyntaxKind::SHA1
        | SyntaxKind::MIN
        | SyntaxKind::FLOOR
        | SyntaxKind::SUBSTR
        | SyntaxKind::EXISTS
        | SyntaxKind::STR
        | SyntaxKind::CONCAT
        | SyntaxKind::IRI
        | SyntaxKind::GROUP_CONCAT
        | SyntaxKind::HOURS
        | SyntaxKind::LANGMATCHES
        | SyntaxKind::ENCODE_FOR_URI
        | SyntaxKind::STRENDS
        | SyntaxKind::REGEX
        | SyntaxKind::isURI
        | SyntaxKind::BOUND
        | SyntaxKind::RAND
        | SyntaxKind::BNODE
        | SyntaxKind::PNAME_NS
        | SyntaxKind::IF
        | SyntaxKind::URI
        | SyntaxKind::STRLEN
        | SyntaxKind::isBLANK
        | SyntaxKind::STRSTARTS
        | SyntaxKind::LCASE
        | SyntaxKind::SHA256
        | SyntaxKind::MAX
        | SyntaxKind::AVG
        | SyntaxKind::SHA384
        | SyntaxKind::VAR1
        | SyntaxKind::COALESCE
        | SyntaxKind::STRLANG
        | SyntaxKind::REPLACE
        | SyntaxKind::DAY
        | SyntaxKind::PNAME_LN
        | SyntaxKind::isIRI
        | SyntaxKind::MONTH
        | SyntaxKind::LParen
        | SyntaxKind::SUM
        | SyntaxKind::UUID
        | SyntaxKind::VAR2
        | SyntaxKind::STRAFTER
        | SyntaxKind::MD5
        | SyntaxKind::NOW
        | SyntaxKind::IRIREF
        | SyntaxKind::NOT => {
            match p.nth(0) {
                SyntaxKind::IRI
                | SyntaxKind::IF
                | SyntaxKind::LANGMATCHES
                | SyntaxKind::CEIL
                | SyntaxKind::ROUND
                | SyntaxKind::STRAFTER
                | SyntaxKind::FLOOR
                | SyntaxKind::STRLEN
                | SyntaxKind::MONTH
                | SyntaxKind::COALESCE
                | SyntaxKind::isIRI
                | SyntaxKind::AVG
                | SyntaxKind::GROUP_CONCAT
                | SyntaxKind::sameTerm
                | SyntaxKind::ENCODE_FOR_URI
                | SyntaxKind::SHA256
                | SyntaxKind::SUM
                | SyntaxKind::RAND
                | SyntaxKind::HOURS
                | SyntaxKind::STRUUID
                | SyntaxKind::MINUTES
                | SyntaxKind::SUBSTR
                | SyntaxKind::CONCAT
                | SyntaxKind::NOW
                | SyntaxKind::STRDT
                | SyntaxKind::DAY
                | SyntaxKind::REGEX
                | SyntaxKind::LParen
                | SyntaxKind::LANG
                | SyntaxKind::TIMEZONE
                | SyntaxKind::isURI
                | SyntaxKind::DATATYPE
                | SyntaxKind::MAX
                | SyntaxKind::UUID
                | SyntaxKind::isNUMERIC
                | SyntaxKind::REPLACE
                | SyntaxKind::TZ
                | SyntaxKind::SHA384
                | SyntaxKind::isBLANK
                | SyntaxKind::MD5
                | SyntaxKind::SECONDS
                | SyntaxKind::MIN
                | SyntaxKind::COUNT
                | SyntaxKind::STRLANG
                | SyntaxKind::SAMPLE
                | SyntaxKind::ABS
                | SyntaxKind::STR
                | SyntaxKind::IRIREF
                | SyntaxKind::URI
                | SyntaxKind::STRBEFORE
                | SyntaxKind::YEAR
                | SyntaxKind::BNODE
                | SyntaxKind::CONTAINS
                | SyntaxKind::EXISTS
                | SyntaxKind::NOT
                | SyntaxKind::STRSTARTS
                | SyntaxKind::STRENDS
                | SyntaxKind::UCASE
                | SyntaxKind::LCASE
                | SyntaxKind::SHA512
                | SyntaxKind::PNAME_NS
                | SyntaxKind::isLITERAL
                | SyntaxKind::SHA1
                | SyntaxKind::BOUND
                | SyntaxKind::PNAME_LN => {
                    parse_Constraint(p);
                }
                SyntaxKind::VAR1 | SyntaxKind::VAR2 => {
                    parse_Var(p);
                }
                SyntaxKind::Eof => {
                    eprintln!("Unexpected Eof");
                    p.close(marker, SyntaxKind::Error);
                    return;
                }
                _ => {
                    p.advance_with_error("Expected ....");
                }
            };
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::OrderCondition);
}
/// [37] BrackettedExpression -> '(' Expression ')'
pub(super) fn parse_BrackettedExpression(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::LParen);
    parse_Expression(p);
    p.expect(SyntaxKind::RParen);
    p.close(marker, SyntaxKind::BrackettedExpression);
}
/// [38] LimitClause -> 'LIMIT' 'INTEGER'
pub(super) fn parse_LimitClause(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::LIMIT);
    p.expect(SyntaxKind::INTEGER);
    p.close(marker, SyntaxKind::LimitClause);
}
/// [39] OffsetClause -> 'OFFSET' 'INTEGER'
pub(super) fn parse_OffsetClause(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::OFFSET);
    p.expect(SyntaxKind::INTEGER);
    p.close(marker, SyntaxKind::OffsetClause);
}
/// [40] DataBlock -> InlineDataOneVar | InlineDataFull
pub(super) fn parse_DataBlock(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::VAR1 | SyntaxKind::VAR2 => {
            parse_InlineDataOneVar(p);
        }
        SyntaxKind::NIL | SyntaxKind::LParen => {
            parse_InlineDataFull(p);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::DataBlock);
}
/// [41] UpdateOne -> Load | Clear | Drop | Add | Move | Copy | Create | InsertData | DeleteData | DeleteWhere | Modify
pub(super) fn parse_UpdateOne(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::LOAD => {
            parse_Load(p);
        }
        SyntaxKind::CLEAR => {
            parse_Clear(p);
        }
        SyntaxKind::DROP => {
            parse_Drop(p);
        }
        SyntaxKind::ADD => {
            parse_Add(p);
        }
        SyntaxKind::MOVE => {
            parse_Move(p);
        }
        SyntaxKind::COPY => {
            parse_Copy(p);
        }
        SyntaxKind::CREATE => {
            parse_Create(p);
        }
        SyntaxKind::INSERT_DATA => {
            parse_InsertData(p);
        }
        SyntaxKind::DELETE_DATA => {
            parse_DeleteData(p);
        }
        SyntaxKind::DELETE_WHERE => {
            parse_DeleteWhere(p);
        }
        SyntaxKind::WITH | SyntaxKind::DELETE | SyntaxKind::INSERT => {
            parse_Modify(p);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::UpdateOne);
}
/// [42] Load -> 'LOAD' 'SILENT'? iri ('INTO' GraphRef)?
pub(super) fn parse_Load(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::LOAD);
    if p.at_any(&[SyntaxKind::SILENT]) {
        p.expect(SyntaxKind::SILENT);
    }
    parse_iri(p);
    if p.at_any(&[SyntaxKind::INTO]) {
        p.expect(SyntaxKind::INTO);
        parse_GraphRef(p);
    }
    p.close(marker, SyntaxKind::Load);
}
/// [43] Clear -> 'CLEAR' 'SILENT'? GraphRefAll
pub(super) fn parse_Clear(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::CLEAR);
    if p.at_any(&[SyntaxKind::SILENT]) {
        p.expect(SyntaxKind::SILENT);
    }
    parse_GraphRefAll(p);
    p.close(marker, SyntaxKind::Clear);
}
/// [44] Drop -> 'DROP' 'SILENT'? GraphRefAll
pub(super) fn parse_Drop(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::DROP);
    if p.at_any(&[SyntaxKind::SILENT]) {
        p.expect(SyntaxKind::SILENT);
    }
    parse_GraphRefAll(p);
    p.close(marker, SyntaxKind::Drop);
}
/// [45] Add -> 'ADD' 'SILENT'? GraphOrDefault 'TO' GraphOrDefault
pub(super) fn parse_Add(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::ADD);
    if p.at_any(&[SyntaxKind::SILENT]) {
        p.expect(SyntaxKind::SILENT);
    }
    parse_GraphOrDefault(p);
    p.expect(SyntaxKind::TO);
    parse_GraphOrDefault(p);
    p.close(marker, SyntaxKind::Add);
}
/// [46] Move -> 'MOVE' 'SILENT'? GraphOrDefault 'TO' GraphOrDefault
pub(super) fn parse_Move(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::MOVE);
    if p.at_any(&[SyntaxKind::SILENT]) {
        p.expect(SyntaxKind::SILENT);
    }
    parse_GraphOrDefault(p);
    p.expect(SyntaxKind::TO);
    parse_GraphOrDefault(p);
    p.close(marker, SyntaxKind::Move);
}
/// [47] Copy -> 'COPY' 'SILENT'? GraphOrDefault 'TO' GraphOrDefault
pub(super) fn parse_Copy(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::COPY);
    if p.at_any(&[SyntaxKind::SILENT]) {
        p.expect(SyntaxKind::SILENT);
    }
    parse_GraphOrDefault(p);
    p.expect(SyntaxKind::TO);
    parse_GraphOrDefault(p);
    p.close(marker, SyntaxKind::Copy);
}
/// [48] Create -> 'CREATE' 'SILENT'? GraphRef
pub(super) fn parse_Create(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::CREATE);
    if p.at_any(&[SyntaxKind::SILENT]) {
        p.expect(SyntaxKind::SILENT);
    }
    parse_GraphRef(p);
    p.close(marker, SyntaxKind::Create);
}
/// [49] InsertData -> 'INSERT_DATA' QuadData
pub(super) fn parse_InsertData(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::INSERT_DATA);
    parse_QuadData(p);
    p.close(marker, SyntaxKind::InsertData);
}
/// [50] DeleteData -> 'DELETE_DATA' QuadData
pub(super) fn parse_DeleteData(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::DELETE_DATA);
    parse_QuadData(p);
    p.close(marker, SyntaxKind::DeleteData);
}
/// [51] DeleteWhere -> 'DELETE_WHERE' QuadPattern
pub(super) fn parse_DeleteWhere(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::DELETE_WHERE);
    parse_QuadPattern(p);
    p.close(marker, SyntaxKind::DeleteWhere);
}
/// [52] Modify -> ('WITH' iri)? (DeleteClause InsertClause? | InsertClause) UsingClause* 'WHERE' GroupGraphPattern
pub(super) fn parse_Modify(p: &mut Parser) {
    let marker = p.open();
    if p.at_any(&[SyntaxKind::WITH]) {
        p.expect(SyntaxKind::WITH);
        parse_iri(p);
    }
    match p.nth(0) {
        SyntaxKind::DELETE => {
            parse_DeleteClause(p);
            if p.at_any(&[SyntaxKind::INSERT]) {
                parse_InsertClause(p);
            }
        }
        SyntaxKind::INSERT => {
            parse_InsertClause(p);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    while [SyntaxKind::USING].contains(&p.nth(0)) {
        parse_UsingClause(p);
    }
    p.expect(SyntaxKind::WHERE);
    parse_GroupGraphPattern(p);
    p.close(marker, SyntaxKind::Modify);
}
/// [53] GraphRef -> 'GRAPH' iri
pub(super) fn parse_GraphRef(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::GRAPH);
    parse_iri(p);
    p.close(marker, SyntaxKind::GraphRef);
}
/// [54] GraphRefAll -> GraphRef | 'DEFAULT' | 'NAMED' | 'ALL'
pub(super) fn parse_GraphRefAll(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::GRAPH => {
            parse_GraphRef(p);
        }
        SyntaxKind::DEFAULT => {
            p.expect(SyntaxKind::DEFAULT);
        }
        SyntaxKind::NAMED => {
            p.expect(SyntaxKind::NAMED);
        }
        SyntaxKind::ALL => {
            p.expect(SyntaxKind::ALL);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::GraphRefAll);
}
/// [55] GraphOrDefault -> 'DEFAULT' | 'GRAPH'? iri
pub(super) fn parse_GraphOrDefault(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::DEFAULT => {
            p.expect(SyntaxKind::DEFAULT);
        }
        SyntaxKind::GRAPH | SyntaxKind::PNAME_LN | SyntaxKind::PNAME_NS | SyntaxKind::IRIREF => {
            if p.at_any(&[SyntaxKind::GRAPH]) {
                p.expect(SyntaxKind::GRAPH);
            }
            parse_iri(p);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::GraphOrDefault);
}
/// [56] QuadData -> '{' Quads '}'
pub(super) fn parse_QuadData(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::LCurly);
    parse_Quads(p);
    p.expect(SyntaxKind::RCurly);
    p.close(marker, SyntaxKind::QuadData);
}
/// [57] QuadPattern -> '{' Quads '}'
pub(super) fn parse_QuadPattern(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::LCurly);
    parse_Quads(p);
    p.expect(SyntaxKind::RCurly);
    p.close(marker, SyntaxKind::QuadPattern);
}
/// [58] DeleteClause -> 'DELETE' QuadPattern
pub(super) fn parse_DeleteClause(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::DELETE);
    parse_QuadPattern(p);
    p.close(marker, SyntaxKind::DeleteClause);
}
/// [59] InsertClause -> 'INSERT' QuadPattern
pub(super) fn parse_InsertClause(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::INSERT);
    parse_QuadPattern(p);
    p.close(marker, SyntaxKind::InsertClause);
}
/// [60] UsingClause -> 'USING' (iri | 'NAMED' iri)
pub(super) fn parse_UsingClause(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::USING);
    match p.nth(0) {
        SyntaxKind::PNAME_LN | SyntaxKind::IRIREF | SyntaxKind::PNAME_NS => {
            parse_iri(p);
        }
        SyntaxKind::NAMED => {
            p.expect(SyntaxKind::NAMED);
            parse_iri(p);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::UsingClause);
}
/// [61] Quads -> TriplesTemplate? (QuadsNotTriples '.'? TriplesTemplate?)*
pub(super) fn parse_Quads(p: &mut Parser) {
    let marker = p.open();
    if p.at_any(&[
        SyntaxKind::True,
        SyntaxKind::DECIMAL,
        SyntaxKind::DECIMAL_POSITIVE,
        SyntaxKind::IRIREF,
        SyntaxKind::INTEGER_NEGATIVE,
        SyntaxKind::STRING_LITERAL_LONG2,
        SyntaxKind::STRING_LITERAL_LONG1,
        SyntaxKind::BLANK_NODE_LABEL,
        SyntaxKind::INTEGER_POSITIVE,
        SyntaxKind::LParen,
        SyntaxKind::VAR2,
        SyntaxKind::PNAME_LN,
        SyntaxKind::NIL,
        SyntaxKind::DECIMAL_NEGATIVE,
        SyntaxKind::LBrack,
        SyntaxKind::ANON,
        SyntaxKind::INTEGER,
        SyntaxKind::STRING_LITERAL2,
        SyntaxKind::DOUBLE,
        SyntaxKind::DOUBLE_POSITIVE,
        SyntaxKind::PNAME_NS,
        SyntaxKind::VAR1,
        SyntaxKind::False,
        SyntaxKind::STRING_LITERAL1,
        SyntaxKind::DOUBLE_NEGATIVE,
    ]) {
        parse_TriplesTemplate(p);
    }
    while [SyntaxKind::GRAPH].contains(&p.nth(0)) {
        parse_QuadsNotTriples(p);
        if p.at_any(&[SyntaxKind::Dot]) {
            p.expect(SyntaxKind::Dot);
        }
        if p.at_any(&[
            SyntaxKind::True,
            SyntaxKind::DECIMAL,
            SyntaxKind::DECIMAL_POSITIVE,
            SyntaxKind::IRIREF,
            SyntaxKind::INTEGER_NEGATIVE,
            SyntaxKind::STRING_LITERAL_LONG2,
            SyntaxKind::STRING_LITERAL_LONG1,
            SyntaxKind::BLANK_NODE_LABEL,
            SyntaxKind::INTEGER_POSITIVE,
            SyntaxKind::LParen,
            SyntaxKind::VAR2,
            SyntaxKind::PNAME_LN,
            SyntaxKind::NIL,
            SyntaxKind::DECIMAL_NEGATIVE,
            SyntaxKind::LBrack,
            SyntaxKind::ANON,
            SyntaxKind::INTEGER,
            SyntaxKind::STRING_LITERAL2,
            SyntaxKind::DOUBLE,
            SyntaxKind::DOUBLE_POSITIVE,
            SyntaxKind::PNAME_NS,
            SyntaxKind::VAR1,
            SyntaxKind::False,
            SyntaxKind::STRING_LITERAL1,
            SyntaxKind::DOUBLE_NEGATIVE,
        ]) {
            parse_TriplesTemplate(p);
        }
    }
    p.close(marker, SyntaxKind::Quads);
}
/// [62] QuadsNotTriples -> 'GRAPH' VarOrIri '{' TriplesTemplate? '}'
pub(super) fn parse_QuadsNotTriples(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::GRAPH);
    parse_VarOrIri(p);
    p.expect(SyntaxKind::LCurly);
    if p.at_any(&[
        SyntaxKind::True,
        SyntaxKind::DECIMAL,
        SyntaxKind::DECIMAL_POSITIVE,
        SyntaxKind::IRIREF,
        SyntaxKind::INTEGER_NEGATIVE,
        SyntaxKind::STRING_LITERAL_LONG2,
        SyntaxKind::STRING_LITERAL_LONG1,
        SyntaxKind::BLANK_NODE_LABEL,
        SyntaxKind::INTEGER_POSITIVE,
        SyntaxKind::LParen,
        SyntaxKind::VAR2,
        SyntaxKind::PNAME_LN,
        SyntaxKind::NIL,
        SyntaxKind::DECIMAL_NEGATIVE,
        SyntaxKind::LBrack,
        SyntaxKind::ANON,
        SyntaxKind::INTEGER,
        SyntaxKind::STRING_LITERAL2,
        SyntaxKind::DOUBLE,
        SyntaxKind::DOUBLE_POSITIVE,
        SyntaxKind::PNAME_NS,
        SyntaxKind::VAR1,
        SyntaxKind::False,
        SyntaxKind::STRING_LITERAL1,
        SyntaxKind::DOUBLE_NEGATIVE,
    ]) {
        parse_TriplesTemplate(p);
    }
    p.expect(SyntaxKind::RCurly);
    p.close(marker, SyntaxKind::QuadsNotTriples);
}
/// [63] TriplesSameSubject -> VarOrTerm PropertyListNotEmpty | TriplesNode PropertyList
pub(super) fn parse_TriplesSameSubject(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::PNAME_LN
        | SyntaxKind::DECIMAL_POSITIVE
        | SyntaxKind::STRING_LITERAL_LONG2
        | SyntaxKind::VAR2
        | SyntaxKind::STRING_LITERAL_LONG1
        | SyntaxKind::BLANK_NODE_LABEL
        | SyntaxKind::DECIMAL_NEGATIVE
        | SyntaxKind::INTEGER_POSITIVE
        | SyntaxKind::DOUBLE_NEGATIVE
        | SyntaxKind::STRING_LITERAL1
        | SyntaxKind::PNAME_NS
        | SyntaxKind::VAR1
        | SyntaxKind::DECIMAL
        | SyntaxKind::ANON
        | SyntaxKind::INTEGER_NEGATIVE
        | SyntaxKind::STRING_LITERAL2
        | SyntaxKind::DOUBLE
        | SyntaxKind::DOUBLE_POSITIVE
        | SyntaxKind::NIL
        | SyntaxKind::INTEGER
        | SyntaxKind::False
        | SyntaxKind::IRIREF
        | SyntaxKind::True => {
            parse_VarOrTerm(p);
            parse_PropertyListNotEmpty(p);
        }
        SyntaxKind::LParen | SyntaxKind::LBrack => {
            parse_TriplesNode(p);
            parse_PropertyList(p);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::TriplesSameSubject);
}
/// [64] GroupGraphPatternSub -> TriplesBlock? (GraphPatternNotTriples '.'? TriplesBlock?)*
pub(super) fn parse_GroupGraphPatternSub(p: &mut Parser) {
    let marker = p.open();
    if p.at_any(&[
        SyntaxKind::DECIMAL,
        SyntaxKind::PNAME_LN,
        SyntaxKind::DOUBLE_POSITIVE,
        SyntaxKind::DOUBLE_NEGATIVE,
        SyntaxKind::STRING_LITERAL1,
        SyntaxKind::IRIREF,
        SyntaxKind::PNAME_NS,
        SyntaxKind::INTEGER_NEGATIVE,
        SyntaxKind::DECIMAL_POSITIVE,
        SyntaxKind::LParen,
        SyntaxKind::INTEGER_POSITIVE,
        SyntaxKind::VAR2,
        SyntaxKind::STRING_LITERAL2,
        SyntaxKind::STRING_LITERAL_LONG1,
        SyntaxKind::True,
        SyntaxKind::DOUBLE,
        SyntaxKind::VAR1,
        SyntaxKind::LBrack,
        SyntaxKind::INTEGER,
        SyntaxKind::False,
        SyntaxKind::DECIMAL_NEGATIVE,
        SyntaxKind::BLANK_NODE_LABEL,
        SyntaxKind::STRING_LITERAL_LONG2,
        SyntaxKind::ANON,
        SyntaxKind::NIL,
    ]) {
        parse_TriplesBlock(p);
    }
    while [
        SyntaxKind::MINUS,
        SyntaxKind::OPTIONAL,
        SyntaxKind::VALUES,
        SyntaxKind::FILTER,
        SyntaxKind::LCurly,
        SyntaxKind::GRAPH,
        SyntaxKind::BIND,
        SyntaxKind::SERVICE,
    ]
    .contains(&p.nth(0))
    {
        parse_GraphPatternNotTriples(p);
        if p.at_any(&[SyntaxKind::Dot]) {
            p.expect(SyntaxKind::Dot);
        }
        if p.at_any(&[
            SyntaxKind::DECIMAL,
            SyntaxKind::PNAME_LN,
            SyntaxKind::DOUBLE_POSITIVE,
            SyntaxKind::DOUBLE_NEGATIVE,
            SyntaxKind::STRING_LITERAL1,
            SyntaxKind::IRIREF,
            SyntaxKind::PNAME_NS,
            SyntaxKind::INTEGER_NEGATIVE,
            SyntaxKind::DECIMAL_POSITIVE,
            SyntaxKind::LParen,
            SyntaxKind::INTEGER_POSITIVE,
            SyntaxKind::VAR2,
            SyntaxKind::STRING_LITERAL2,
            SyntaxKind::STRING_LITERAL_LONG1,
            SyntaxKind::True,
            SyntaxKind::DOUBLE,
            SyntaxKind::VAR1,
            SyntaxKind::LBrack,
            SyntaxKind::INTEGER,
            SyntaxKind::False,
            SyntaxKind::DECIMAL_NEGATIVE,
            SyntaxKind::BLANK_NODE_LABEL,
            SyntaxKind::STRING_LITERAL_LONG2,
            SyntaxKind::ANON,
            SyntaxKind::NIL,
        ]) {
            parse_TriplesBlock(p);
        }
    }
    p.close(marker, SyntaxKind::GroupGraphPatternSub);
}
/// [65] TriplesBlock -> TriplesSameSubjectPath ('.' TriplesBlock?)?
pub(super) fn parse_TriplesBlock(p: &mut Parser) {
    let marker = p.open();
    parse_TriplesSameSubjectPath(p);
    if p.at_any(&[SyntaxKind::Dot]) {
        p.expect(SyntaxKind::Dot);
        if p.at_any(&[
            SyntaxKind::DECIMAL,
            SyntaxKind::PNAME_LN,
            SyntaxKind::DOUBLE_POSITIVE,
            SyntaxKind::DOUBLE_NEGATIVE,
            SyntaxKind::STRING_LITERAL1,
            SyntaxKind::IRIREF,
            SyntaxKind::PNAME_NS,
            SyntaxKind::INTEGER_NEGATIVE,
            SyntaxKind::DECIMAL_POSITIVE,
            SyntaxKind::LParen,
            SyntaxKind::INTEGER_POSITIVE,
            SyntaxKind::VAR2,
            SyntaxKind::STRING_LITERAL2,
            SyntaxKind::STRING_LITERAL_LONG1,
            SyntaxKind::True,
            SyntaxKind::DOUBLE,
            SyntaxKind::VAR1,
            SyntaxKind::LBrack,
            SyntaxKind::INTEGER,
            SyntaxKind::False,
            SyntaxKind::DECIMAL_NEGATIVE,
            SyntaxKind::BLANK_NODE_LABEL,
            SyntaxKind::STRING_LITERAL_LONG2,
            SyntaxKind::ANON,
            SyntaxKind::NIL,
        ]) {
            parse_TriplesBlock(p);
        }
    }
    p.close(marker, SyntaxKind::TriplesBlock);
}
/// [66] GraphPatternNotTriples -> GroupOrUnionGraphPattern | OptionalGraphPattern | MinusGraphPattern | GraphGraphPattern | ServiceGraphPattern | Filter | Bind | InlineData
pub(super) fn parse_GraphPatternNotTriples(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::LCurly => {
            parse_GroupOrUnionGraphPattern(p);
        }
        SyntaxKind::OPTIONAL => {
            parse_OptionalGraphPattern(p);
        }
        SyntaxKind::MINUS => {
            parse_MinusGraphPattern(p);
        }
        SyntaxKind::GRAPH => {
            parse_GraphGraphPattern(p);
        }
        SyntaxKind::SERVICE => {
            parse_ServiceGraphPattern(p);
        }
        SyntaxKind::FILTER => {
            parse_Filter(p);
        }
        SyntaxKind::BIND => {
            parse_Bind(p);
        }
        SyntaxKind::VALUES => {
            parse_InlineData(p);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::GraphPatternNotTriples);
}
/// [67] TriplesSameSubjectPath -> VarOrTerm PropertyListPathNotEmpty | TriplesNodePath PropertyListPath
pub(super) fn parse_TriplesSameSubjectPath(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::INTEGER
        | SyntaxKind::PNAME_NS
        | SyntaxKind::DECIMAL
        | SyntaxKind::DOUBLE_POSITIVE
        | SyntaxKind::IRIREF
        | SyntaxKind::VAR1
        | SyntaxKind::STRING_LITERAL1
        | SyntaxKind::BLANK_NODE_LABEL
        | SyntaxKind::ANON
        | SyntaxKind::DECIMAL_POSITIVE
        | SyntaxKind::DOUBLE
        | SyntaxKind::STRING_LITERAL_LONG2
        | SyntaxKind::INTEGER_POSITIVE
        | SyntaxKind::True
        | SyntaxKind::STRING_LITERAL_LONG1
        | SyntaxKind::DOUBLE_NEGATIVE
        | SyntaxKind::False
        | SyntaxKind::DECIMAL_NEGATIVE
        | SyntaxKind::INTEGER_NEGATIVE
        | SyntaxKind::PNAME_LN
        | SyntaxKind::VAR2
        | SyntaxKind::STRING_LITERAL2
        | SyntaxKind::NIL => {
            parse_VarOrTerm(p);
            parse_PropertyListPathNotEmpty(p);
        }
        SyntaxKind::LBrack | SyntaxKind::LParen => {
            parse_TriplesNodePath(p);
            parse_PropertyListPath(p);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::TriplesSameSubjectPath);
}
/// [68] GroupOrUnionGraphPattern -> GroupGraphPattern ('UNION' GroupGraphPattern)*
pub(super) fn parse_GroupOrUnionGraphPattern(p: &mut Parser) {
    let marker = p.open();
    parse_GroupGraphPattern(p);
    while [SyntaxKind::UNION].contains(&p.nth(0)) {
        p.expect(SyntaxKind::UNION);
        parse_GroupGraphPattern(p);
    }
    p.close(marker, SyntaxKind::GroupOrUnionGraphPattern);
}
/// [69] OptionalGraphPattern -> 'OPTIONAL' GroupGraphPattern
pub(super) fn parse_OptionalGraphPattern(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::OPTIONAL);
    parse_GroupGraphPattern(p);
    p.close(marker, SyntaxKind::OptionalGraphPattern);
}
/// [70] MinusGraphPattern -> 'MINUS' GroupGraphPattern
pub(super) fn parse_MinusGraphPattern(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::MINUS);
    parse_GroupGraphPattern(p);
    p.close(marker, SyntaxKind::MinusGraphPattern);
}
/// [71] GraphGraphPattern -> 'GRAPH' VarOrIri GroupGraphPattern
pub(super) fn parse_GraphGraphPattern(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::GRAPH);
    parse_VarOrIri(p);
    parse_GroupGraphPattern(p);
    p.close(marker, SyntaxKind::GraphGraphPattern);
}
/// [72] ServiceGraphPattern -> 'SERVICE' 'SILENT'? VarOrIri GroupGraphPattern
pub(super) fn parse_ServiceGraphPattern(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::SERVICE);
    if p.at_any(&[SyntaxKind::SILENT]) {
        p.expect(SyntaxKind::SILENT);
    }
    parse_VarOrIri(p);
    parse_GroupGraphPattern(p);
    p.close(marker, SyntaxKind::ServiceGraphPattern);
}
/// [73] Filter -> 'FILTER' Constraint
pub(super) fn parse_Filter(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::FILTER);
    parse_Constraint(p);
    p.close(marker, SyntaxKind::Filter);
}
/// [74] Bind -> 'BIND' '(' Expression 'AS' Var ')'
pub(super) fn parse_Bind(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::BIND);
    p.expect(SyntaxKind::LParen);
    parse_Expression(p);
    p.expect(SyntaxKind::AS);
    parse_Var(p);
    p.expect(SyntaxKind::RParen);
    p.close(marker, SyntaxKind::Bind);
}
/// [75] InlineData -> 'VALUES' DataBlock
pub(super) fn parse_InlineData(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::VALUES);
    parse_DataBlock(p);
    p.close(marker, SyntaxKind::InlineData);
}
/// [76] InlineDataOneVar -> Var '{' DataBlockValue* '}'
pub(super) fn parse_InlineDataOneVar(p: &mut Parser) {
    let marker = p.open();
    parse_Var(p);
    p.expect(SyntaxKind::LCurly);
    while [
        SyntaxKind::PNAME_LN,
        SyntaxKind::STRING_LITERAL_LONG1,
        SyntaxKind::DECIMAL_POSITIVE,
        SyntaxKind::STRING_LITERAL1,
        SyntaxKind::DOUBLE_NEGATIVE,
        SyntaxKind::DECIMAL,
        SyntaxKind::False,
        SyntaxKind::True,
        SyntaxKind::STRING_LITERAL2,
        SyntaxKind::PNAME_NS,
        SyntaxKind::INTEGER,
        SyntaxKind::DOUBLE,
        SyntaxKind::INTEGER_POSITIVE,
        SyntaxKind::INTEGER_NEGATIVE,
        SyntaxKind::IRIREF,
        SyntaxKind::UNDEF,
        SyntaxKind::STRING_LITERAL_LONG2,
        SyntaxKind::DECIMAL_NEGATIVE,
        SyntaxKind::DOUBLE_POSITIVE,
    ]
    .contains(&p.nth(0))
    {
        parse_DataBlockValue(p);
    }
    p.expect(SyntaxKind::RCurly);
    p.close(marker, SyntaxKind::InlineDataOneVar);
}
/// [77] InlineDataFull -> ('NIL' | '(' Var* ')') '{' ('(' DataBlockValue* ')' | 'NIL')* '}'
pub(super) fn parse_InlineDataFull(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::NIL => {
            p.expect(SyntaxKind::NIL);
        }
        SyntaxKind::LParen => {
            p.expect(SyntaxKind::LParen);
            while [SyntaxKind::VAR1, SyntaxKind::VAR2].contains(&p.nth(0)) {
                parse_Var(p);
            }
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.expect(SyntaxKind::LCurly);
    while [SyntaxKind::NIL, SyntaxKind::LParen].contains(&p.nth(0)) {
        match p.nth(0) {
            SyntaxKind::LParen => {
                p.expect(SyntaxKind::LParen);
                while [
                    SyntaxKind::PNAME_LN,
                    SyntaxKind::STRING_LITERAL_LONG1,
                    SyntaxKind::DECIMAL_POSITIVE,
                    SyntaxKind::STRING_LITERAL1,
                    SyntaxKind::DOUBLE_NEGATIVE,
                    SyntaxKind::DECIMAL,
                    SyntaxKind::False,
                    SyntaxKind::True,
                    SyntaxKind::STRING_LITERAL2,
                    SyntaxKind::PNAME_NS,
                    SyntaxKind::INTEGER,
                    SyntaxKind::DOUBLE,
                    SyntaxKind::INTEGER_POSITIVE,
                    SyntaxKind::INTEGER_NEGATIVE,
                    SyntaxKind::IRIREF,
                    SyntaxKind::UNDEF,
                    SyntaxKind::STRING_LITERAL_LONG2,
                    SyntaxKind::DECIMAL_NEGATIVE,
                    SyntaxKind::DOUBLE_POSITIVE,
                ]
                .contains(&p.nth(0))
                {
                    parse_DataBlockValue(p);
                }
                p.expect(SyntaxKind::RParen);
            }
            SyntaxKind::NIL => {
                p.expect(SyntaxKind::NIL);
            }
            SyntaxKind::Eof => {
                eprintln!("Unexpected Eof");
                p.close(marker, SyntaxKind::Error);
                return;
            }
            _ => {
                p.advance_with_error("Expected ....");
            }
        };
    }
    p.expect(SyntaxKind::RCurly);
    p.close(marker, SyntaxKind::InlineDataFull);
}
/// [78] DataBlockValue -> iri | RDFLiteral | NumericLiteral | BooleanLiteral | 'UNDEF'
pub(super) fn parse_DataBlockValue(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::PNAME_LN | SyntaxKind::IRIREF | SyntaxKind::PNAME_NS => {
            parse_iri(p);
        }
        SyntaxKind::STRING_LITERAL2
        | SyntaxKind::STRING_LITERAL1
        | SyntaxKind::STRING_LITERAL_LONG1
        | SyntaxKind::STRING_LITERAL_LONG2 => {
            parse_RDFLiteral(p);
        }
        SyntaxKind::DECIMAL_POSITIVE
        | SyntaxKind::DOUBLE_POSITIVE
        | SyntaxKind::DOUBLE_NEGATIVE
        | SyntaxKind::DECIMAL_NEGATIVE
        | SyntaxKind::INTEGER
        | SyntaxKind::DOUBLE
        | SyntaxKind::INTEGER_POSITIVE
        | SyntaxKind::DECIMAL
        | SyntaxKind::INTEGER_NEGATIVE => {
            parse_NumericLiteral(p);
        }
        SyntaxKind::False | SyntaxKind::True => {
            parse_BooleanLiteral(p);
        }
        SyntaxKind::UNDEF => {
            p.expect(SyntaxKind::UNDEF);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::DataBlockValue);
}
/// [79] RDFLiteral -> String ('LANGTAG' | '^^' iri)?
pub(super) fn parse_RDFLiteral(p: &mut Parser) {
    let marker = p.open();
    parse_String(p);
    if p.at_any(&[SyntaxKind::DoubleZirkumflex, SyntaxKind::LANGTAG]) {
        match p.nth(0) {
            SyntaxKind::LANGTAG => {
                p.expect(SyntaxKind::LANGTAG);
            }
            SyntaxKind::DoubleZirkumflex => {
                p.expect(SyntaxKind::DoubleZirkumflex);
                parse_iri(p);
            }
            SyntaxKind::Eof => {
                eprintln!("Unexpected Eof");
                p.close(marker, SyntaxKind::Error);
                return;
            }
            _ => {
                p.advance_with_error("Expected ....");
            }
        };
    }
    p.close(marker, SyntaxKind::RDFLiteral);
}
/// [80] NumericLiteral -> NumericLiteralUnsigned | NumericLiteralPositive | NumericLiteralNegative
pub(super) fn parse_NumericLiteral(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::DOUBLE | SyntaxKind::INTEGER | SyntaxKind::DECIMAL => {
            parse_NumericLiteralUnsigned(p);
        }
        SyntaxKind::INTEGER_POSITIVE
        | SyntaxKind::DECIMAL_POSITIVE
        | SyntaxKind::DOUBLE_POSITIVE => {
            parse_NumericLiteralPositive(p);
        }
        SyntaxKind::DOUBLE_NEGATIVE
        | SyntaxKind::INTEGER_NEGATIVE
        | SyntaxKind::DECIMAL_NEGATIVE => {
            parse_NumericLiteralNegative(p);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::NumericLiteral);
}
/// [81] BooleanLiteral -> 'true' | 'false'
pub(super) fn parse_BooleanLiteral(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::True => {
            p.expect(SyntaxKind::True);
        }
        SyntaxKind::False => {
            p.expect(SyntaxKind::False);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::BooleanLiteral);
}
/// [82] ArgList -> 'NIL' | '(' 'DISTINCT'? Expression (',' Expression)* ')'
pub(super) fn parse_ArgList(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::NIL => {
            p.expect(SyntaxKind::NIL);
        }
        SyntaxKind::LParen => {
            p.expect(SyntaxKind::LParen);
            if p.at_any(&[SyntaxKind::DISTINCT]) {
                p.expect(SyntaxKind::DISTINCT);
            }
            parse_Expression(p);
            while [SyntaxKind::Colon].contains(&p.nth(0)) {
                p.expect(SyntaxKind::Colon);
                parse_Expression(p);
            }
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::ArgList);
}
/// [83] ExpressionList -> 'NIL' | '(' Expression (',' Expression)* ')'
pub(super) fn parse_ExpressionList(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::NIL => {
            p.expect(SyntaxKind::NIL);
        }
        SyntaxKind::LParen => {
            p.expect(SyntaxKind::LParen);
            parse_Expression(p);
            while [SyntaxKind::Colon].contains(&p.nth(0)) {
                p.expect(SyntaxKind::Colon);
                parse_Expression(p);
            }
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::ExpressionList);
}
/// [84] ConstructTriples -> TriplesSameSubject ('.' ConstructTriples?)?
pub(super) fn parse_ConstructTriples(p: &mut Parser) {
    let marker = p.open();
    parse_TriplesSameSubject(p);
    if p.at_any(&[SyntaxKind::Dot]) {
        p.expect(SyntaxKind::Dot);
        if p.at_any(&[
            SyntaxKind::DECIMAL_NEGATIVE,
            SyntaxKind::DOUBLE,
            SyntaxKind::STRING_LITERAL1,
            SyntaxKind::DOUBLE_NEGATIVE,
            SyntaxKind::IRIREF,
            SyntaxKind::True,
            SyntaxKind::DECIMAL,
            SyntaxKind::INTEGER_NEGATIVE,
            SyntaxKind::STRING_LITERAL_LONG2,
            SyntaxKind::STRING_LITERAL_LONG1,
            SyntaxKind::ANON,
            SyntaxKind::DOUBLE_POSITIVE,
            SyntaxKind::DECIMAL_POSITIVE,
            SyntaxKind::PNAME_NS,
            SyntaxKind::False,
            SyntaxKind::LBrack,
            SyntaxKind::VAR2,
            SyntaxKind::NIL,
            SyntaxKind::BLANK_NODE_LABEL,
            SyntaxKind::INTEGER_POSITIVE,
            SyntaxKind::VAR1,
            SyntaxKind::LParen,
            SyntaxKind::STRING_LITERAL2,
            SyntaxKind::INTEGER,
            SyntaxKind::PNAME_LN,
        ]) {
            parse_ConstructTriples(p);
        }
    }
    p.close(marker, SyntaxKind::ConstructTriples);
}
/// [85] VarOrTerm -> Var | GraphTerm
pub(super) fn parse_VarOrTerm(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::VAR1 | SyntaxKind::VAR2 => {
            parse_Var(p);
        }
        SyntaxKind::STRING_LITERAL1
        | SyntaxKind::DOUBLE
        | SyntaxKind::DECIMAL_NEGATIVE
        | SyntaxKind::DOUBLE_POSITIVE
        | SyntaxKind::False
        | SyntaxKind::INTEGER
        | SyntaxKind::STRING_LITERAL2
        | SyntaxKind::STRING_LITERAL_LONG1
        | SyntaxKind::DOUBLE_NEGATIVE
        | SyntaxKind::IRIREF
        | SyntaxKind::PNAME_NS
        | SyntaxKind::True
        | SyntaxKind::DECIMAL_POSITIVE
        | SyntaxKind::INTEGER_POSITIVE
        | SyntaxKind::DECIMAL
        | SyntaxKind::PNAME_LN
        | SyntaxKind::INTEGER_NEGATIVE
        | SyntaxKind::STRING_LITERAL_LONG2
        | SyntaxKind::ANON
        | SyntaxKind::BLANK_NODE_LABEL
        | SyntaxKind::NIL => {
            parse_GraphTerm(p);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::VarOrTerm);
}
/// [86] PropertyListNotEmpty -> Verb ObjectList (';' (Verb ObjectList)?)*
pub(super) fn parse_PropertyListNotEmpty(p: &mut Parser) {
    let marker = p.open();
    parse_Verb(p);
    parse_ObjectList(p);
    while [SyntaxKind::Semicolon].contains(&p.nth(0)) {
        p.expect(SyntaxKind::Semicolon);
        if p.at_any(&[
            SyntaxKind::PNAME_LN,
            SyntaxKind::a,
            SyntaxKind::PNAME_NS,
            SyntaxKind::IRIREF,
            SyntaxKind::VAR2,
            SyntaxKind::VAR1,
        ]) {
            parse_Verb(p);
            parse_ObjectList(p);
        }
    }
    p.close(marker, SyntaxKind::PropertyListNotEmpty);
}
/// [87] TriplesNode -> Collection | BlankNodePropertyList
pub(super) fn parse_TriplesNode(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::LParen => {
            parse_Collection(p);
        }
        SyntaxKind::LBrack => {
            parse_BlankNodePropertyList(p);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::TriplesNode);
}
/// [88] PropertyList -> PropertyListNotEmpty?
pub(super) fn parse_PropertyList(p: &mut Parser) {
    let marker = p.open();
    if p.at_any(&[
        SyntaxKind::PNAME_NS,
        SyntaxKind::IRIREF,
        SyntaxKind::PNAME_LN,
        SyntaxKind::a,
        SyntaxKind::VAR2,
        SyntaxKind::VAR1,
    ]) {
        parse_PropertyListNotEmpty(p);
    }
    p.close(marker, SyntaxKind::PropertyList);
}
/// [89] Verb -> VarOrIri | 'a'
pub(super) fn parse_Verb(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::VAR1
        | SyntaxKind::PNAME_LN
        | SyntaxKind::IRIREF
        | SyntaxKind::PNAME_NS
        | SyntaxKind::VAR2 => {
            parse_VarOrIri(p);
        }
        SyntaxKind::a => {
            p.expect(SyntaxKind::a);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::Verb);
}
/// [90] ObjectList -> Object (',' Object)*
pub(super) fn parse_ObjectList(p: &mut Parser) {
    let marker = p.open();
    parse_Object(p);
    while [SyntaxKind::Colon].contains(&p.nth(0)) {
        p.expect(SyntaxKind::Colon);
        parse_Object(p);
    }
    p.close(marker, SyntaxKind::ObjectList);
}
/// [91] Object -> GraphNode
pub(super) fn parse_Object(p: &mut Parser) {
    let marker = p.open();
    parse_GraphNode(p);
    p.close(marker, SyntaxKind::Object);
}
/// [92] GraphNode -> VarOrTerm | TriplesNode
pub(super) fn parse_GraphNode(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::BLANK_NODE_LABEL
        | SyntaxKind::PNAME_LN
        | SyntaxKind::DECIMAL_NEGATIVE
        | SyntaxKind::STRING_LITERAL1
        | SyntaxKind::DECIMAL
        | SyntaxKind::ANON
        | SyntaxKind::STRING_LITERAL2
        | SyntaxKind::True
        | SyntaxKind::NIL
        | SyntaxKind::DOUBLE
        | SyntaxKind::STRING_LITERAL_LONG2
        | SyntaxKind::DOUBLE_POSITIVE
        | SyntaxKind::INTEGER_POSITIVE
        | SyntaxKind::INTEGER
        | SyntaxKind::VAR2
        | SyntaxKind::STRING_LITERAL_LONG1
        | SyntaxKind::DOUBLE_NEGATIVE
        | SyntaxKind::VAR1
        | SyntaxKind::False
        | SyntaxKind::IRIREF
        | SyntaxKind::PNAME_NS
        | SyntaxKind::DECIMAL_POSITIVE
        | SyntaxKind::INTEGER_NEGATIVE => {
            parse_VarOrTerm(p);
        }
        SyntaxKind::LParen | SyntaxKind::LBrack => {
            parse_TriplesNode(p);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::GraphNode);
}
/// [93] PropertyListPathNotEmpty -> (VerbPath | VerbSimple) ObjectListPath (';' ((VerbPath | VerbSimple) ObjectList)?)*
pub(super) fn parse_PropertyListPathNotEmpty(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::PNAME_NS
        | SyntaxKind::Zirkumflex
        | SyntaxKind::PNAME_LN
        | SyntaxKind::a
        | SyntaxKind::ExclamationMark
        | SyntaxKind::IRIREF
        | SyntaxKind::LParen => {
            parse_VerbPath(p);
        }
        SyntaxKind::VAR1 | SyntaxKind::VAR2 => {
            parse_VerbSimple(p);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    parse_ObjectListPath(p);
    while [SyntaxKind::Semicolon].contains(&p.nth(0)) {
        p.expect(SyntaxKind::Semicolon);
        if p.at_any(&[
            SyntaxKind::PNAME_LN,
            SyntaxKind::LParen,
            SyntaxKind::VAR2,
            SyntaxKind::IRIREF,
            SyntaxKind::VAR1,
            SyntaxKind::a,
            SyntaxKind::Zirkumflex,
            SyntaxKind::PNAME_NS,
            SyntaxKind::ExclamationMark,
        ]) {
            match p.nth(0) {
                SyntaxKind::PNAME_NS
                | SyntaxKind::Zirkumflex
                | SyntaxKind::PNAME_LN
                | SyntaxKind::a
                | SyntaxKind::ExclamationMark
                | SyntaxKind::IRIREF
                | SyntaxKind::LParen => {
                    parse_VerbPath(p);
                }
                SyntaxKind::VAR1 | SyntaxKind::VAR2 => {
                    parse_VerbSimple(p);
                }
                SyntaxKind::Eof => {
                    eprintln!("Unexpected Eof");
                    p.close(marker, SyntaxKind::Error);
                    return;
                }
                _ => {
                    p.advance_with_error("Expected ....");
                }
            };
            parse_ObjectList(p);
        }
    }
    p.close(marker, SyntaxKind::PropertyListPathNotEmpty);
}
/// [94] TriplesNodePath -> CollectionPath | BlankNodePropertyListPath
pub(super) fn parse_TriplesNodePath(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::LParen => {
            parse_CollectionPath(p);
        }
        SyntaxKind::LBrack => {
            parse_BlankNodePropertyListPath(p);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::TriplesNodePath);
}
/// [95] PropertyListPath -> PropertyListPathNotEmpty?
pub(super) fn parse_PropertyListPath(p: &mut Parser) {
    let marker = p.open();
    if p.at_any(&[
        SyntaxKind::Zirkumflex,
        SyntaxKind::LParen,
        SyntaxKind::PNAME_NS,
        SyntaxKind::PNAME_LN,
        SyntaxKind::ExclamationMark,
        SyntaxKind::VAR1,
        SyntaxKind::a,
        SyntaxKind::IRIREF,
        SyntaxKind::VAR2,
    ]) {
        parse_PropertyListPathNotEmpty(p);
    }
    p.close(marker, SyntaxKind::PropertyListPath);
}
/// [96] VerbPath -> Path
pub(super) fn parse_VerbPath(p: &mut Parser) {
    let marker = p.open();
    parse_Path(p);
    p.close(marker, SyntaxKind::VerbPath);
}
/// [97] VerbSimple -> Var
pub(super) fn parse_VerbSimple(p: &mut Parser) {
    let marker = p.open();
    parse_Var(p);
    p.close(marker, SyntaxKind::VerbSimple);
}
/// [98] ObjectListPath -> ObjectPath (',' ObjectPath)*
pub(super) fn parse_ObjectListPath(p: &mut Parser) {
    let marker = p.open();
    parse_ObjectPath(p);
    while [SyntaxKind::Colon].contains(&p.nth(0)) {
        p.expect(SyntaxKind::Colon);
        parse_ObjectPath(p);
    }
    p.close(marker, SyntaxKind::ObjectListPath);
}
/// [99] Path -> PathAlternative
pub(super) fn parse_Path(p: &mut Parser) {
    let marker = p.open();
    parse_PathAlternative(p);
    p.close(marker, SyntaxKind::Path);
}
/// [100] ObjectPath -> GraphNodePath
pub(super) fn parse_ObjectPath(p: &mut Parser) {
    let marker = p.open();
    parse_GraphNodePath(p);
    p.close(marker, SyntaxKind::ObjectPath);
}
/// [101] GraphNodePath -> VarOrTerm | TriplesNodePath
pub(super) fn parse_GraphNodePath(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::BLANK_NODE_LABEL
        | SyntaxKind::PNAME_LN
        | SyntaxKind::DECIMAL_NEGATIVE
        | SyntaxKind::STRING_LITERAL1
        | SyntaxKind::DECIMAL
        | SyntaxKind::ANON
        | SyntaxKind::STRING_LITERAL2
        | SyntaxKind::True
        | SyntaxKind::NIL
        | SyntaxKind::DOUBLE
        | SyntaxKind::STRING_LITERAL_LONG2
        | SyntaxKind::DOUBLE_POSITIVE
        | SyntaxKind::INTEGER_POSITIVE
        | SyntaxKind::INTEGER
        | SyntaxKind::VAR2
        | SyntaxKind::STRING_LITERAL_LONG1
        | SyntaxKind::DOUBLE_NEGATIVE
        | SyntaxKind::VAR1
        | SyntaxKind::False
        | SyntaxKind::IRIREF
        | SyntaxKind::PNAME_NS
        | SyntaxKind::DECIMAL_POSITIVE
        | SyntaxKind::INTEGER_NEGATIVE => {
            parse_VarOrTerm(p);
        }
        SyntaxKind::LParen | SyntaxKind::LBrack => {
            parse_TriplesNodePath(p);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::GraphNodePath);
}
/// [102] PathAlternative -> PathSequence ('|' PathSequence)*
pub(super) fn parse_PathAlternative(p: &mut Parser) {
    let marker = p.open();
    parse_PathSequence(p);
    while [SyntaxKind::Pipe].contains(&p.nth(0)) {
        p.expect(SyntaxKind::Pipe);
        parse_PathSequence(p);
    }
    p.close(marker, SyntaxKind::PathAlternative);
}
/// [103] PathSequence -> PathEltOrInverse ('/' PathEltOrInverse)*
pub(super) fn parse_PathSequence(p: &mut Parser) {
    let marker = p.open();
    parse_PathEltOrInverse(p);
    while [SyntaxKind::Slash].contains(&p.nth(0)) {
        p.expect(SyntaxKind::Slash);
        parse_PathEltOrInverse(p);
    }
    p.close(marker, SyntaxKind::PathSequence);
}
/// [104] PathEltOrInverse -> PathElt | '^' PathElt
pub(super) fn parse_PathEltOrInverse(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::ExclamationMark
        | SyntaxKind::PNAME_LN
        | SyntaxKind::IRIREF
        | SyntaxKind::LParen
        | SyntaxKind::PNAME_NS
        | SyntaxKind::a => {
            parse_PathElt(p);
        }
        SyntaxKind::Zirkumflex => {
            p.expect(SyntaxKind::Zirkumflex);
            parse_PathElt(p);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::PathEltOrInverse);
}
/// [105] PathElt -> PathPrimary PathMod?
pub(super) fn parse_PathElt(p: &mut Parser) {
    let marker = p.open();
    parse_PathPrimary(p);
    if p.at_any(&[SyntaxKind::Star, SyntaxKind::QuestionMark, SyntaxKind::Plus]) {
        parse_PathMod(p);
    }
    p.close(marker, SyntaxKind::PathElt);
}
/// [106] PathPrimary -> iri | 'a' | '!' PathNegatedPropertySet | '(' Path ')'
pub(super) fn parse_PathPrimary(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::PNAME_LN | SyntaxKind::IRIREF | SyntaxKind::PNAME_NS => {
            parse_iri(p);
        }
        SyntaxKind::a => {
            p.expect(SyntaxKind::a);
        }
        SyntaxKind::ExclamationMark => {
            p.expect(SyntaxKind::ExclamationMark);
            parse_PathNegatedPropertySet(p);
        }
        SyntaxKind::LParen => {
            p.expect(SyntaxKind::LParen);
            parse_Path(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::PathPrimary);
}
/// [107] PathMod -> '?' | '*' | '+'
pub(super) fn parse_PathMod(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::QuestionMark => {
            p.expect(SyntaxKind::QuestionMark);
        }
        SyntaxKind::Star => {
            p.expect(SyntaxKind::Star);
        }
        SyntaxKind::Plus => {
            p.expect(SyntaxKind::Plus);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::PathMod);
}
/// [108] PathNegatedPropertySet -> PathOneInPropertySet | '(' (PathOneInPropertySet ('|' PathOneInPropertySet)*)? ')'
pub(super) fn parse_PathNegatedPropertySet(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::IRIREF
        | SyntaxKind::PNAME_LN
        | SyntaxKind::a
        | SyntaxKind::Zirkumflex
        | SyntaxKind::PNAME_NS => {
            parse_PathOneInPropertySet(p);
        }
        SyntaxKind::LParen => {
            p.expect(SyntaxKind::LParen);
            if p.at_any(&[
                SyntaxKind::IRIREF,
                SyntaxKind::a,
                SyntaxKind::PNAME_NS,
                SyntaxKind::PNAME_LN,
                SyntaxKind::Zirkumflex,
            ]) {
                parse_PathOneInPropertySet(p);
                while [SyntaxKind::Pipe].contains(&p.nth(0)) {
                    p.expect(SyntaxKind::Pipe);
                    parse_PathOneInPropertySet(p);
                }
            }
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::PathNegatedPropertySet);
}
/// [109] PathOneInPropertySet -> iri | 'a' | '^' (iri | 'a')
pub(super) fn parse_PathOneInPropertySet(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::PNAME_LN | SyntaxKind::IRIREF | SyntaxKind::PNAME_NS => {
            parse_iri(p);
        }
        SyntaxKind::a => {
            p.expect(SyntaxKind::a);
        }
        SyntaxKind::Zirkumflex => {
            p.expect(SyntaxKind::Zirkumflex);
            match p.nth(0) {
                SyntaxKind::PNAME_LN | SyntaxKind::IRIREF | SyntaxKind::PNAME_NS => {
                    parse_iri(p);
                }
                SyntaxKind::a => {
                    p.expect(SyntaxKind::a);
                }
                SyntaxKind::Eof => {
                    eprintln!("Unexpected Eof");
                    p.close(marker, SyntaxKind::Error);
                    return;
                }
                _ => {
                    p.advance_with_error("Expected ....");
                }
            };
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::PathOneInPropertySet);
}
/// [110] Integer -> 'INTEGER'
pub(super) fn parse_Integer(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::INTEGER);
    p.close(marker, SyntaxKind::Integer);
}
/// [111] Collection -> '(' GraphNode GraphNode* ')'
pub(super) fn parse_Collection(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::LParen);
    parse_GraphNode(p);
    while [
        SyntaxKind::True,
        SyntaxKind::STRING_LITERAL2,
        SyntaxKind::NIL,
        SyntaxKind::DECIMAL_POSITIVE,
        SyntaxKind::False,
        SyntaxKind::LParen,
        SyntaxKind::LBrack,
        SyntaxKind::VAR2,
        SyntaxKind::IRIREF,
        SyntaxKind::DOUBLE,
        SyntaxKind::PNAME_LN,
        SyntaxKind::DOUBLE_POSITIVE,
        SyntaxKind::INTEGER,
        SyntaxKind::VAR1,
        SyntaxKind::DECIMAL_NEGATIVE,
        SyntaxKind::STRING_LITERAL1,
        SyntaxKind::DECIMAL,
        SyntaxKind::INTEGER_POSITIVE,
        SyntaxKind::DOUBLE_NEGATIVE,
        SyntaxKind::PNAME_NS,
        SyntaxKind::STRING_LITERAL_LONG1,
        SyntaxKind::STRING_LITERAL_LONG2,
        SyntaxKind::INTEGER_NEGATIVE,
        SyntaxKind::BLANK_NODE_LABEL,
        SyntaxKind::ANON,
    ]
    .contains(&p.nth(0))
    {
        parse_GraphNode(p);
    }
    p.expect(SyntaxKind::RParen);
    p.close(marker, SyntaxKind::Collection);
}
/// [112] BlankNodePropertyList -> '[' PropertyListNotEmpty ']'
pub(super) fn parse_BlankNodePropertyList(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::LBrack);
    parse_PropertyListNotEmpty(p);
    p.expect(SyntaxKind::RBrack);
    p.close(marker, SyntaxKind::BlankNodePropertyList);
}
/// [113] CollectionPath -> '(' GraphNodePath GraphNodePath* ')'
pub(super) fn parse_CollectionPath(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::LParen);
    parse_GraphNodePath(p);
    while [
        SyntaxKind::DOUBLE_POSITIVE,
        SyntaxKind::STRING_LITERAL_LONG1,
        SyntaxKind::False,
        SyntaxKind::DECIMAL,
        SyntaxKind::IRIREF,
        SyntaxKind::LParen,
        SyntaxKind::BLANK_NODE_LABEL,
        SyntaxKind::INTEGER_POSITIVE,
        SyntaxKind::INTEGER,
        SyntaxKind::LBrack,
        SyntaxKind::STRING_LITERAL1,
        SyntaxKind::STRING_LITERAL2,
        SyntaxKind::DOUBLE_NEGATIVE,
        SyntaxKind::VAR1,
        SyntaxKind::NIL,
        SyntaxKind::DECIMAL_NEGATIVE,
        SyntaxKind::True,
        SyntaxKind::DOUBLE,
        SyntaxKind::PNAME_NS,
        SyntaxKind::ANON,
        SyntaxKind::VAR2,
        SyntaxKind::INTEGER_NEGATIVE,
        SyntaxKind::DECIMAL_POSITIVE,
        SyntaxKind::STRING_LITERAL_LONG2,
        SyntaxKind::PNAME_LN,
    ]
    .contains(&p.nth(0))
    {
        parse_GraphNodePath(p);
    }
    p.expect(SyntaxKind::RParen);
    p.close(marker, SyntaxKind::CollectionPath);
}
/// [114] BlankNodePropertyListPath -> '[' PropertyListPathNotEmpty ']'
pub(super) fn parse_BlankNodePropertyListPath(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::LBrack);
    parse_PropertyListPathNotEmpty(p);
    p.expect(SyntaxKind::RBrack);
    p.close(marker, SyntaxKind::BlankNodePropertyListPath);
}
/// [115] GraphTerm -> iri | RDFLiteral | NumericLiteral | BooleanLiteral | BlankNode | 'NIL'
pub(super) fn parse_GraphTerm(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::PNAME_LN | SyntaxKind::IRIREF | SyntaxKind::PNAME_NS => {
            parse_iri(p);
        }
        SyntaxKind::STRING_LITERAL2
        | SyntaxKind::STRING_LITERAL1
        | SyntaxKind::STRING_LITERAL_LONG1
        | SyntaxKind::STRING_LITERAL_LONG2 => {
            parse_RDFLiteral(p);
        }
        SyntaxKind::DECIMAL_POSITIVE
        | SyntaxKind::DOUBLE_POSITIVE
        | SyntaxKind::DOUBLE_NEGATIVE
        | SyntaxKind::DECIMAL_NEGATIVE
        | SyntaxKind::INTEGER
        | SyntaxKind::DOUBLE
        | SyntaxKind::INTEGER_POSITIVE
        | SyntaxKind::DECIMAL
        | SyntaxKind::INTEGER_NEGATIVE => {
            parse_NumericLiteral(p);
        }
        SyntaxKind::False | SyntaxKind::True => {
            parse_BooleanLiteral(p);
        }
        SyntaxKind::BLANK_NODE_LABEL | SyntaxKind::ANON => {
            parse_BlankNode(p);
        }
        SyntaxKind::NIL => {
            p.expect(SyntaxKind::NIL);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::GraphTerm);
}
/// [116] BlankNode -> 'BLANK_NODE_LABEL' | 'ANON'
pub(super) fn parse_BlankNode(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::BLANK_NODE_LABEL => {
            p.expect(SyntaxKind::BLANK_NODE_LABEL);
        }
        SyntaxKind::ANON => {
            p.expect(SyntaxKind::ANON);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::BlankNode);
}
/// [117] ConditionalOrExpression -> ConditionalAndExpression ('||' ConditionalAndExpression)*
pub(super) fn parse_ConditionalOrExpression(p: &mut Parser) {
    let marker = p.open();
    parse_ConditionalAndExpression(p);
    while [SyntaxKind::DoublePipe].contains(&p.nth(0)) {
        p.expect(SyntaxKind::DoublePipe);
        parse_ConditionalAndExpression(p);
    }
    p.close(marker, SyntaxKind::ConditionalOrExpression);
}
/// [118] ConditionalAndExpression -> ValueLogical ('&&' ValueLogical)*
pub(super) fn parse_ConditionalAndExpression(p: &mut Parser) {
    let marker = p.open();
    parse_ValueLogical(p);
    while [SyntaxKind::DoubleAnd].contains(&p.nth(0)) {
        p.expect(SyntaxKind::DoubleAnd);
        parse_ValueLogical(p);
    }
    p.close(marker, SyntaxKind::ConditionalAndExpression);
}
/// [119] ValueLogical -> RelationalExpression
pub(super) fn parse_ValueLogical(p: &mut Parser) {
    let marker = p.open();
    parse_RelationalExpression(p);
    p.close(marker, SyntaxKind::ValueLogical);
}
/// [120] RelationalExpression -> NumericExpression ('=' NumericExpression | '!=' NumericExpression | '<' NumericExpression | '>' NumericExpression | '<=' NumericExpression | '>=' NumericExpression | 'IN' ExpressionList | 'NOT' 'IN' ExpressionList)?
pub(super) fn parse_RelationalExpression(p: &mut Parser) {
    let marker = p.open();
    parse_NumericExpression(p);
    if p.at_any(&[
        SyntaxKind::More,
        SyntaxKind::NOT,
        SyntaxKind::LessEquals,
        SyntaxKind::MoreEquals,
        SyntaxKind::ExclamationMarkEquals,
        SyntaxKind::Equals,
        SyntaxKind::Less,
        SyntaxKind::IN,
    ]) {
        match p.nth(0) {
            SyntaxKind::Equals => {
                p.expect(SyntaxKind::Equals);
                parse_NumericExpression(p);
            }
            SyntaxKind::ExclamationMarkEquals => {
                p.expect(SyntaxKind::ExclamationMarkEquals);
                parse_NumericExpression(p);
            }
            SyntaxKind::Less => {
                p.expect(SyntaxKind::Less);
                parse_NumericExpression(p);
            }
            SyntaxKind::More => {
                p.expect(SyntaxKind::More);
                parse_NumericExpression(p);
            }
            SyntaxKind::LessEquals => {
                p.expect(SyntaxKind::LessEquals);
                parse_NumericExpression(p);
            }
            SyntaxKind::MoreEquals => {
                p.expect(SyntaxKind::MoreEquals);
                parse_NumericExpression(p);
            }
            SyntaxKind::IN => {
                p.expect(SyntaxKind::IN);
                parse_ExpressionList(p);
            }
            SyntaxKind::NOT => {
                p.expect(SyntaxKind::NOT);
                p.expect(SyntaxKind::IN);
                parse_ExpressionList(p);
            }
            SyntaxKind::Eof => {
                eprintln!("Unexpected Eof");
                p.close(marker, SyntaxKind::Error);
                return;
            }
            _ => {
                p.advance_with_error("Expected ....");
            }
        };
    }
    p.close(marker, SyntaxKind::RelationalExpression);
}
/// [121] NumericExpression -> AdditiveExpression
pub(super) fn parse_NumericExpression(p: &mut Parser) {
    let marker = p.open();
    parse_AdditiveExpression(p);
    p.close(marker, SyntaxKind::NumericExpression);
}
/// [122] AdditiveExpression -> MultiplicativeExpression ('+' MultiplicativeExpression | '-' MultiplicativeExpression | (NumericLiteralPositive | NumericLiteralNegative) ('*' UnaryExpression | '/' UnaryExpression)*)*
pub(super) fn parse_AdditiveExpression(p: &mut Parser) {
    let marker = p.open();
    parse_MultiplicativeExpression(p);
    while [
        SyntaxKind::INTEGER_POSITIVE,
        SyntaxKind::Plus,
        SyntaxKind::Minus,
        SyntaxKind::DOUBLE_NEGATIVE,
        SyntaxKind::DECIMAL_POSITIVE,
        SyntaxKind::INTEGER_NEGATIVE,
        SyntaxKind::DECIMAL_NEGATIVE,
        SyntaxKind::DOUBLE_POSITIVE,
    ]
    .contains(&p.nth(0))
    {
        match p.nth(0) {
            SyntaxKind::Plus => {
                p.expect(SyntaxKind::Plus);
                parse_MultiplicativeExpression(p);
            }
            SyntaxKind::Minus => {
                p.expect(SyntaxKind::Minus);
                parse_MultiplicativeExpression(p);
            }
            SyntaxKind::DECIMAL_NEGATIVE
            | SyntaxKind::DOUBLE_NEGATIVE
            | SyntaxKind::INTEGER_NEGATIVE
            | SyntaxKind::DOUBLE_POSITIVE
            | SyntaxKind::DECIMAL_POSITIVE
            | SyntaxKind::INTEGER_POSITIVE => {
                match p.nth(0) {
                    SyntaxKind::INTEGER_POSITIVE
                    | SyntaxKind::DECIMAL_POSITIVE
                    | SyntaxKind::DOUBLE_POSITIVE => {
                        parse_NumericLiteralPositive(p);
                    }
                    SyntaxKind::DOUBLE_NEGATIVE
                    | SyntaxKind::INTEGER_NEGATIVE
                    | SyntaxKind::DECIMAL_NEGATIVE => {
                        parse_NumericLiteralNegative(p);
                    }
                    SyntaxKind::Eof => {
                        eprintln!("Unexpected Eof");
                        p.close(marker, SyntaxKind::Error);
                        return;
                    }
                    _ => {
                        p.advance_with_error("Expected ....");
                    }
                };
                while [SyntaxKind::Slash, SyntaxKind::Star].contains(&p.nth(0)) {
                    match p.nth(0) {
                        SyntaxKind::Star => {
                            p.expect(SyntaxKind::Star);
                            parse_UnaryExpression(p);
                        }
                        SyntaxKind::Slash => {
                            p.expect(SyntaxKind::Slash);
                            parse_UnaryExpression(p);
                        }
                        SyntaxKind::Eof => {
                            eprintln!("Unexpected Eof");
                            p.close(marker, SyntaxKind::Error);
                            return;
                        }
                        _ => {
                            p.advance_with_error("Expected ....");
                        }
                    };
                }
            }
            SyntaxKind::Eof => {
                eprintln!("Unexpected Eof");
                p.close(marker, SyntaxKind::Error);
                return;
            }
            _ => {
                p.advance_with_error("Expected ....");
            }
        };
    }
    p.close(marker, SyntaxKind::AdditiveExpression);
}
/// [123] MultiplicativeExpression -> UnaryExpression ('*' UnaryExpression | '/' UnaryExpression)*
pub(super) fn parse_MultiplicativeExpression(p: &mut Parser) {
    let marker = p.open();
    parse_UnaryExpression(p);
    while [SyntaxKind::Star, SyntaxKind::Slash].contains(&p.nth(0)) {
        match p.nth(0) {
            SyntaxKind::Star => {
                p.expect(SyntaxKind::Star);
                parse_UnaryExpression(p);
            }
            SyntaxKind::Slash => {
                p.expect(SyntaxKind::Slash);
                parse_UnaryExpression(p);
            }
            SyntaxKind::Eof => {
                eprintln!("Unexpected Eof");
                p.close(marker, SyntaxKind::Error);
                return;
            }
            _ => {
                p.advance_with_error("Expected ....");
            }
        };
    }
    p.close(marker, SyntaxKind::MultiplicativeExpression);
}
/// [124] NumericLiteralPositive -> 'INTEGER_POSITIVE' | 'DECIMAL_POSITIVE' | 'DOUBLE_POSITIVE'
pub(super) fn parse_NumericLiteralPositive(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::INTEGER_POSITIVE => {
            p.expect(SyntaxKind::INTEGER_POSITIVE);
        }
        SyntaxKind::DECIMAL_POSITIVE => {
            p.expect(SyntaxKind::DECIMAL_POSITIVE);
        }
        SyntaxKind::DOUBLE_POSITIVE => {
            p.expect(SyntaxKind::DOUBLE_POSITIVE);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::NumericLiteralPositive);
}
/// [125] NumericLiteralNegative -> 'INTEGER_NEGATIVE' | 'DECIMAL_NEGATIVE' | 'DOUBLE_NEGATIVE'
pub(super) fn parse_NumericLiteralNegative(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::INTEGER_NEGATIVE => {
            p.expect(SyntaxKind::INTEGER_NEGATIVE);
        }
        SyntaxKind::DECIMAL_NEGATIVE => {
            p.expect(SyntaxKind::DECIMAL_NEGATIVE);
        }
        SyntaxKind::DOUBLE_NEGATIVE => {
            p.expect(SyntaxKind::DOUBLE_NEGATIVE);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::NumericLiteralNegative);
}
/// [126] UnaryExpression -> '!' PrimaryExpression | '+' PrimaryExpression | '-' PrimaryExpression | PrimaryExpression
pub(super) fn parse_UnaryExpression(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::ExclamationMark => {
            p.expect(SyntaxKind::ExclamationMark);
            parse_PrimaryExpression(p);
        }
        SyntaxKind::Plus => {
            p.expect(SyntaxKind::Plus);
            parse_PrimaryExpression(p);
        }
        SyntaxKind::Minus => {
            p.expect(SyntaxKind::Minus);
            parse_PrimaryExpression(p);
        }
        SyntaxKind::SUM
        | SyntaxKind::SECONDS
        | SyntaxKind::STRING_LITERAL_LONG2
        | SyntaxKind::STRLEN
        | SyntaxKind::HOURS
        | SyntaxKind::STRING_LITERAL1
        | SyntaxKind::REPLACE
        | SyntaxKind::FLOOR
        | SyntaxKind::SHA256
        | SyntaxKind::STRAFTER
        | SyntaxKind::MIN
        | SyntaxKind::True
        | SyntaxKind::SHA1
        | SyntaxKind::IRIREF
        | SyntaxKind::isURI
        | SyntaxKind::NOW
        | SyntaxKind::isLITERAL
        | SyntaxKind::CONCAT
        | SyntaxKind::AVG
        | SyntaxKind::sameTerm
        | SyntaxKind::ROUND
        | SyntaxKind::STRBEFORE
        | SyntaxKind::DOUBLE
        | SyntaxKind::YEAR
        | SyntaxKind::GROUP_CONCAT
        | SyntaxKind::MONTH
        | SyntaxKind::STRUUID
        | SyntaxKind::URI
        | SyntaxKind::LCASE
        | SyntaxKind::BNODE
        | SyntaxKind::SAMPLE
        | SyntaxKind::INTEGER
        | SyntaxKind::LANGMATCHES
        | SyntaxKind::INTEGER_POSITIVE
        | SyntaxKind::INTEGER_NEGATIVE
        | SyntaxKind::STRDT
        | SyntaxKind::False
        | SyntaxKind::STRENDS
        | SyntaxKind::PNAME_LN
        | SyntaxKind::LParen
        | SyntaxKind::STRLANG
        | SyntaxKind::COALESCE
        | SyntaxKind::DOUBLE_POSITIVE
        | SyntaxKind::isNUMERIC
        | SyntaxKind::LANG
        | SyntaxKind::ABS
        | SyntaxKind::STR
        | SyntaxKind::COUNT
        | SyntaxKind::IF
        | SyntaxKind::TZ
        | SyntaxKind::VAR1
        | SyntaxKind::REGEX
        | SyntaxKind::NOT
        | SyntaxKind::BOUND
        | SyntaxKind::SHA512
        | SyntaxKind::isIRI
        | SyntaxKind::MAX
        | SyntaxKind::STRING_LITERAL2
        | SyntaxKind::EXISTS
        | SyntaxKind::DOUBLE_NEGATIVE
        | SyntaxKind::DECIMAL_NEGATIVE
        | SyntaxKind::TIMEZONE
        | SyntaxKind::IRI
        | SyntaxKind::VAR2
        | SyntaxKind::CONTAINS
        | SyntaxKind::STRING_LITERAL_LONG1
        | SyntaxKind::CEIL
        | SyntaxKind::SHA384
        | SyntaxKind::SUBSTR
        | SyntaxKind::MINUTES
        | SyntaxKind::DECIMAL_POSITIVE
        | SyntaxKind::DATATYPE
        | SyntaxKind::UCASE
        | SyntaxKind::MD5
        | SyntaxKind::ENCODE_FOR_URI
        | SyntaxKind::STRSTARTS
        | SyntaxKind::DECIMAL
        | SyntaxKind::PNAME_NS
        | SyntaxKind::UUID
        | SyntaxKind::DAY
        | SyntaxKind::isBLANK
        | SyntaxKind::RAND => {
            parse_PrimaryExpression(p);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::UnaryExpression);
}
/// [127] PrimaryExpression -> BrackettedExpression | BuiltInCall | iriOrFunction | RDFLiteral | NumericLiteral | BooleanLiteral | Var
pub(super) fn parse_PrimaryExpression(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::LParen => {
            parse_BrackettedExpression(p);
        }
        SyntaxKind::DATATYPE
        | SyntaxKind::STR
        | SyntaxKind::COUNT
        | SyntaxKind::URI
        | SyntaxKind::STRSTARTS
        | SyntaxKind::SHA256
        | SyntaxKind::GROUP_CONCAT
        | SyntaxKind::UCASE
        | SyntaxKind::SHA384
        | SyntaxKind::ABS
        | SyntaxKind::STRAFTER
        | SyntaxKind::STRLANG
        | SyntaxKind::LANGMATCHES
        | SyntaxKind::STRENDS
        | SyntaxKind::IF
        | SyntaxKind::sameTerm
        | SyntaxKind::isIRI
        | SyntaxKind::isLITERAL
        | SyntaxKind::NOT
        | SyntaxKind::MD5
        | SyntaxKind::SUBSTR
        | SyntaxKind::CEIL
        | SyntaxKind::BNODE
        | SyntaxKind::NOW
        | SyntaxKind::STRDT
        | SyntaxKind::isNUMERIC
        | SyntaxKind::STRLEN
        | SyntaxKind::ENCODE_FOR_URI
        | SyntaxKind::SHA1
        | SyntaxKind::SECONDS
        | SyntaxKind::LANG
        | SyntaxKind::AVG
        | SyntaxKind::DAY
        | SyntaxKind::ROUND
        | SyntaxKind::IRI
        | SyntaxKind::SUM
        | SyntaxKind::MAX
        | SyntaxKind::FLOOR
        | SyntaxKind::COALESCE
        | SyntaxKind::REGEX
        | SyntaxKind::CONTAINS
        | SyntaxKind::UUID
        | SyntaxKind::isURI
        | SyntaxKind::TIMEZONE
        | SyntaxKind::YEAR
        | SyntaxKind::RAND
        | SyntaxKind::TZ
        | SyntaxKind::STRBEFORE
        | SyntaxKind::LCASE
        | SyntaxKind::isBLANK
        | SyntaxKind::MONTH
        | SyntaxKind::HOURS
        | SyntaxKind::REPLACE
        | SyntaxKind::CONCAT
        | SyntaxKind::STRUUID
        | SyntaxKind::EXISTS
        | SyntaxKind::BOUND
        | SyntaxKind::SHA512
        | SyntaxKind::MIN
        | SyntaxKind::SAMPLE
        | SyntaxKind::MINUTES => {
            parse_BuiltInCall(p);
        }
        SyntaxKind::PNAME_LN | SyntaxKind::IRIREF | SyntaxKind::PNAME_NS => {
            parse_iriOrFunction(p);
        }
        SyntaxKind::STRING_LITERAL2
        | SyntaxKind::STRING_LITERAL1
        | SyntaxKind::STRING_LITERAL_LONG1
        | SyntaxKind::STRING_LITERAL_LONG2 => {
            parse_RDFLiteral(p);
        }
        SyntaxKind::DECIMAL_POSITIVE
        | SyntaxKind::DOUBLE_POSITIVE
        | SyntaxKind::DOUBLE_NEGATIVE
        | SyntaxKind::DECIMAL_NEGATIVE
        | SyntaxKind::INTEGER
        | SyntaxKind::DOUBLE
        | SyntaxKind::INTEGER_POSITIVE
        | SyntaxKind::DECIMAL
        | SyntaxKind::INTEGER_NEGATIVE => {
            parse_NumericLiteral(p);
        }
        SyntaxKind::False | SyntaxKind::True => {
            parse_BooleanLiteral(p);
        }
        SyntaxKind::VAR1 | SyntaxKind::VAR2 => {
            parse_Var(p);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::PrimaryExpression);
}
/// [128] iriOrFunction -> iri ArgList?
pub(super) fn parse_iriOrFunction(p: &mut Parser) {
    let marker = p.open();
    parse_iri(p);
    if p.at_any(&[SyntaxKind::LParen, SyntaxKind::NIL]) {
        parse_ArgList(p);
    }
    p.close(marker, SyntaxKind::iriOrFunction);
}
/// [129] Aggregate -> 'COUNT' '(' 'DISTINCT'? ('*' | Expression) ')' | 'SUM' '(' 'DISTINCT'? Expression ')' | 'MIN' '(' 'DISTINCT'? Expression ')' | 'MAX' '(' 'DISTINCT'? Expression ')' | 'AVG' '(' 'DISTINCT'? Expression ')' | 'SAMPLE' '(' 'DISTINCT'? Expression ')' | 'GROUP_CONCAT' '(' 'DISTINCT'? Expression (';' 'SEPARATOR' '=' String)? ')'
pub(super) fn parse_Aggregate(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::COUNT => {
            p.expect(SyntaxKind::COUNT);
            p.expect(SyntaxKind::LParen);
            if p.at_any(&[SyntaxKind::DISTINCT]) {
                p.expect(SyntaxKind::DISTINCT);
            }
            match p.nth(0) {
                SyntaxKind::Star => {
                    p.expect(SyntaxKind::Star);
                }
                SyntaxKind::INTEGER_NEGATIVE
                | SyntaxKind::SUBSTR
                | SyntaxKind::True
                | SyntaxKind::STRLEN
                | SyntaxKind::YEAR
                | SyntaxKind::STRSTARTS
                | SyntaxKind::NOW
                | SyntaxKind::VAR1
                | SyntaxKind::False
                | SyntaxKind::SAMPLE
                | SyntaxKind::UCASE
                | SyntaxKind::AVG
                | SyntaxKind::sameTerm
                | SyntaxKind::DOUBLE_NEGATIVE
                | SyntaxKind::GROUP_CONCAT
                | SyntaxKind::SUM
                | SyntaxKind::DECIMAL
                | SyntaxKind::COALESCE
                | SyntaxKind::PNAME_LN
                | SyntaxKind::SHA256
                | SyntaxKind::SHA1
                | SyntaxKind::ROUND
                | SyntaxKind::REPLACE
                | SyntaxKind::STRING_LITERAL1
                | SyntaxKind::MAX
                | SyntaxKind::STRDT
                | SyntaxKind::UUID
                | SyntaxKind::PNAME_NS
                | SyntaxKind::VAR2
                | SyntaxKind::CONTAINS
                | SyntaxKind::DECIMAL_NEGATIVE
                | SyntaxKind::ExclamationMark
                | SyntaxKind::LANGMATCHES
                | SyntaxKind::RAND
                | SyntaxKind::SECONDS
                | SyntaxKind::Plus
                | SyntaxKind::DECIMAL_POSITIVE
                | SyntaxKind::TIMEZONE
                | SyntaxKind::LANG
                | SyntaxKind::INTEGER
                | SyntaxKind::DOUBLE
                | SyntaxKind::STR
                | SyntaxKind::COUNT
                | SyntaxKind::DATATYPE
                | SyntaxKind::isIRI
                | SyntaxKind::isLITERAL
                | SyntaxKind::MD5
                | SyntaxKind::STRBEFORE
                | SyntaxKind::IRI
                | SyntaxKind::HOURS
                | SyntaxKind::TZ
                | SyntaxKind::BOUND
                | SyntaxKind::isBLANK
                | SyntaxKind::CONCAT
                | SyntaxKind::STRING_LITERAL_LONG2
                | SyntaxKind::DAY
                | SyntaxKind::LParen
                | SyntaxKind::CEIL
                | SyntaxKind::STRLANG
                | SyntaxKind::isURI
                | SyntaxKind::MINUTES
                | SyntaxKind::BNODE
                | SyntaxKind::FLOOR
                | SyntaxKind::NOT
                | SyntaxKind::IF
                | SyntaxKind::INTEGER_POSITIVE
                | SyntaxKind::SHA512
                | SyntaxKind::STRUUID
                | SyntaxKind::REGEX
                | SyntaxKind::MONTH
                | SyntaxKind::DOUBLE_POSITIVE
                | SyntaxKind::URI
                | SyntaxKind::IRIREF
                | SyntaxKind::MIN
                | SyntaxKind::STRING_LITERAL2
                | SyntaxKind::ENCODE_FOR_URI
                | SyntaxKind::STRAFTER
                | SyntaxKind::STRENDS
                | SyntaxKind::LCASE
                | SyntaxKind::isNUMERIC
                | SyntaxKind::STRING_LITERAL_LONG1
                | SyntaxKind::Minus
                | SyntaxKind::SHA384
                | SyntaxKind::EXISTS
                | SyntaxKind::ABS => {
                    parse_Expression(p);
                }
                SyntaxKind::Eof => {
                    eprintln!("Unexpected Eof");
                    p.close(marker, SyntaxKind::Error);
                    return;
                }
                _ => {
                    p.advance_with_error("Expected ....");
                }
            };
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::SUM => {
            p.expect(SyntaxKind::SUM);
            p.expect(SyntaxKind::LParen);
            if p.at_any(&[SyntaxKind::DISTINCT]) {
                p.expect(SyntaxKind::DISTINCT);
            }
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::MIN => {
            p.expect(SyntaxKind::MIN);
            p.expect(SyntaxKind::LParen);
            if p.at_any(&[SyntaxKind::DISTINCT]) {
                p.expect(SyntaxKind::DISTINCT);
            }
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::MAX => {
            p.expect(SyntaxKind::MAX);
            p.expect(SyntaxKind::LParen);
            if p.at_any(&[SyntaxKind::DISTINCT]) {
                p.expect(SyntaxKind::DISTINCT);
            }
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::AVG => {
            p.expect(SyntaxKind::AVG);
            p.expect(SyntaxKind::LParen);
            if p.at_any(&[SyntaxKind::DISTINCT]) {
                p.expect(SyntaxKind::DISTINCT);
            }
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::SAMPLE => {
            p.expect(SyntaxKind::SAMPLE);
            p.expect(SyntaxKind::LParen);
            if p.at_any(&[SyntaxKind::DISTINCT]) {
                p.expect(SyntaxKind::DISTINCT);
            }
            parse_Expression(p);
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::GROUP_CONCAT => {
            p.expect(SyntaxKind::GROUP_CONCAT);
            p.expect(SyntaxKind::LParen);
            if p.at_any(&[SyntaxKind::DISTINCT]) {
                p.expect(SyntaxKind::DISTINCT);
            }
            parse_Expression(p);
            if p.at_any(&[SyntaxKind::Semicolon]) {
                p.expect(SyntaxKind::Semicolon);
                p.expect(SyntaxKind::SEPARATOR);
                p.expect(SyntaxKind::Equals);
                parse_String(p);
            }
            p.expect(SyntaxKind::RParen);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::Aggregate);
}
/// [130] SubstringExpression -> 'SUBSTR' '(' Expression ',' Expression (',' Expression)? ')'
pub(super) fn parse_SubstringExpression(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::SUBSTR);
    p.expect(SyntaxKind::LParen);
    parse_Expression(p);
    p.expect(SyntaxKind::Colon);
    parse_Expression(p);
    if p.at_any(&[SyntaxKind::Colon]) {
        p.expect(SyntaxKind::Colon);
        parse_Expression(p);
    }
    p.expect(SyntaxKind::RParen);
    p.close(marker, SyntaxKind::SubstringExpression);
}
/// [131] StrReplaceExpression -> 'REPLACE' '(' Expression ',' Expression ',' Expression (',' Expression)? ')'
pub(super) fn parse_StrReplaceExpression(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::REPLACE);
    p.expect(SyntaxKind::LParen);
    parse_Expression(p);
    p.expect(SyntaxKind::Colon);
    parse_Expression(p);
    p.expect(SyntaxKind::Colon);
    parse_Expression(p);
    if p.at_any(&[SyntaxKind::Colon]) {
        p.expect(SyntaxKind::Colon);
        parse_Expression(p);
    }
    p.expect(SyntaxKind::RParen);
    p.close(marker, SyntaxKind::StrReplaceExpression);
}
/// [132] RegexExpression -> 'REGEX' '(' Expression ',' Expression (',' Expression)? ')'
pub(super) fn parse_RegexExpression(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::REGEX);
    p.expect(SyntaxKind::LParen);
    parse_Expression(p);
    p.expect(SyntaxKind::Colon);
    parse_Expression(p);
    if p.at_any(&[SyntaxKind::Colon]) {
        p.expect(SyntaxKind::Colon);
        parse_Expression(p);
    }
    p.expect(SyntaxKind::RParen);
    p.close(marker, SyntaxKind::RegexExpression);
}
/// [133] ExistsFunc -> 'EXISTS' GroupGraphPattern
pub(super) fn parse_ExistsFunc(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::EXISTS);
    parse_GroupGraphPattern(p);
    p.close(marker, SyntaxKind::ExistsFunc);
}
/// [134] NotExistsFunc -> 'NOT' 'EXISTS' GroupGraphPattern
pub(super) fn parse_NotExistsFunc(p: &mut Parser) {
    let marker = p.open();
    p.expect(SyntaxKind::NOT);
    p.expect(SyntaxKind::EXISTS);
    parse_GroupGraphPattern(p);
    p.close(marker, SyntaxKind::NotExistsFunc);
}
/// [135] String -> 'STRING_LITERAL1' | 'STRING_LITERAL2' | 'STRING_LITERAL_LONG1' | 'STRING_LITERAL_LONG2'
pub(super) fn parse_String(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::STRING_LITERAL1 => {
            p.expect(SyntaxKind::STRING_LITERAL1);
        }
        SyntaxKind::STRING_LITERAL2 => {
            p.expect(SyntaxKind::STRING_LITERAL2);
        }
        SyntaxKind::STRING_LITERAL_LONG1 => {
            p.expect(SyntaxKind::STRING_LITERAL_LONG1);
        }
        SyntaxKind::STRING_LITERAL_LONG2 => {
            p.expect(SyntaxKind::STRING_LITERAL_LONG2);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::String);
}
/// [136] NumericLiteralUnsigned -> 'INTEGER' | 'DECIMAL' | 'DOUBLE'
pub(super) fn parse_NumericLiteralUnsigned(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::INTEGER => {
            p.expect(SyntaxKind::INTEGER);
        }
        SyntaxKind::DECIMAL => {
            p.expect(SyntaxKind::DECIMAL);
        }
        SyntaxKind::DOUBLE => {
            p.expect(SyntaxKind::DOUBLE);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::NumericLiteralUnsigned);
}
/// [137] PrefixedName -> 'PNAME_LN' | 'PNAME_NS'
pub(super) fn parse_PrefixedName(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        SyntaxKind::PNAME_LN => {
            p.expect(SyntaxKind::PNAME_LN);
        }
        SyntaxKind::PNAME_NS => {
            p.expect(SyntaxKind::PNAME_NS);
        }
        SyntaxKind::Eof => {
            eprintln!("Unexpected Eof");
            p.close(marker, SyntaxKind::Error);
            return;
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, SyntaxKind::PrefixedName);
}
