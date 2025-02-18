#![allow(non_snake_case)]
use super::{
    syntax_tree::{TokenKind, TreeKind},
    Parser,
};
/// [0] QueryUnit -> Query
pub(super) fn parse_QueryUnit(p: &mut Parser) {
    let marker = p.open();
    parse_Query(p);
    p.close(marker, TreeKind::QueryUnit);
}
/// [1] Query -> Prologue (SelectQuery | ConstructQuery | DescribeQuery | AskQuery) ValuesClause
pub(super) fn parse_Query(p: &mut Parser) {
    let marker = p.open();
    parse_Prologue(p);
    match p.nth(0) {
        TokenKind::SELECT => {
            parse_SelectQuery(p);
        }
        TokenKind::CONSTRUCT => {
            parse_ConstructQuery(p);
        }
        TokenKind::DESCRIBE => {
            parse_DescribeQuery(p);
        }
        TokenKind::ASK => {
            parse_AskQuery(p);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    parse_ValuesClause(p);
    p.close(marker, TreeKind::Query);
}
/// [2] Prologue -> (BaseDecl | PrefixDecl)*
pub(super) fn parse_Prologue(p: &mut Parser) {
    let marker = p.open();
    while [TokenKind::BASE, TokenKind::PREFIX].contains(&p.nth(0)) {
        match p.nth(0) {
            TokenKind::BASE => {
                parse_BaseDecl(p);
            }
            TokenKind::PREFIX => {
                parse_PrefixDecl(p);
            }
            _ => {
                p.advance_with_error("Expected ....");
            }
        };
    }
    p.close(marker, TreeKind::Prologue);
}
/// [3] SelectQuery -> SelectClause DatasetClause* WhereClause SolutionModifier
pub(super) fn parse_SelectQuery(p: &mut Parser) {
    let marker = p.open();
    parse_SelectClause(p);
    while [TokenKind::FROM].contains(&p.nth(0)) {
        parse_DatasetClause(p);
    }
    parse_WhereClause(p);
    parse_SolutionModifier(p);
    p.close(marker, TreeKind::SelectQuery);
}
/// [4] ConstructQuery -> 'CONSTRUCT' (ConstructTemplate DatasetClause* WhereClause SolutionModifier | DatasetClause* 'WHERE' '{' TriplesTemplate? '}' SolutionModifier)
pub(super) fn parse_ConstructQuery(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::CONSTRUCT);
    match p.nth(0) {
        TokenKind::LCurly => {
            parse_ConstructTemplate(p);
            while [TokenKind::FROM].contains(&p.nth(0)) {
                parse_DatasetClause(p);
            }
            parse_WhereClause(p);
            parse_SolutionModifier(p);
        }
        TokenKind::FROM | TokenKind::WHERE => {
            while [TokenKind::FROM].contains(&p.nth(0)) {
                parse_DatasetClause(p);
            }
            p.expect(TokenKind::WHERE);
            p.expect(TokenKind::LCurly);
            if p
                .at_any(
                    &[
                        TokenKind::STRING_LITERAL1,
                        TokenKind::DOUBLE,
                        TokenKind::VAR2,
                        TokenKind::PNAME_NS,
                        TokenKind::PNAME_LN,
                        TokenKind::DOUBLE_NEGATIVE,
                        TokenKind::STRING_LITERAL2,
                        TokenKind::NIL,
                        TokenKind::ANON,
                        TokenKind::STRING_LITERAL_LONG2,
                        TokenKind::LParen,
                        TokenKind::DECIMAL_POSITIVE,
                        TokenKind::DOUBLE_POSITIVE,
                        TokenKind::INTEGER,
                        TokenKind::STRING_LITERAL_LONG1,
                        TokenKind::DECIMAL_NEGATIVE,
                        TokenKind::LBrack,
                        TokenKind::BLANK_NODE_LABEL,
                        TokenKind::False,
                        TokenKind::True,
                        TokenKind::INTEGER_POSITIVE,
                        TokenKind::IRIREF,
                        TokenKind::DECIMAL,
                        TokenKind::VAR1,
                        TokenKind::INTEGER_NEGATIVE,
                    ],
                )
            {
                parse_TriplesTemplate(p);
            }
            p.expect(TokenKind::RCurly);
            parse_SolutionModifier(p);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::ConstructQuery);
}
/// [5] DescribeQuery -> 'DESCRIBE' (VarOrIri VarOrIri* | '*') DatasetClause* WhereClause? SolutionModifier
pub(super) fn parse_DescribeQuery(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::DESCRIBE);
    match p.nth(0) {
        TokenKind::PNAME_NS
        | TokenKind::VAR2
        | TokenKind::VAR1
        | TokenKind::PNAME_LN
        | TokenKind::IRIREF => {
            parse_VarOrIri(p);
            while [
                TokenKind::VAR2,
                TokenKind::IRIREF,
                TokenKind::PNAME_NS,
                TokenKind::VAR1,
                TokenKind::PNAME_LN,
            ]
                .contains(&p.nth(0))
            {
                parse_VarOrIri(p);
            }
        }
        TokenKind::Star => {
            p.expect(TokenKind::Star);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    while [TokenKind::FROM].contains(&p.nth(0)) {
        parse_DatasetClause(p);
    }
    if p.at_any(&[TokenKind::LCurly, TokenKind::WHERE]) {
        parse_WhereClause(p);
    }
    parse_SolutionModifier(p);
    p.close(marker, TreeKind::DescribeQuery);
}
/// [6] AskQuery -> 'ASK' DatasetClause* WhereClause SolutionModifier
pub(super) fn parse_AskQuery(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::ASK);
    while [TokenKind::FROM].contains(&p.nth(0)) {
        parse_DatasetClause(p);
    }
    parse_WhereClause(p);
    parse_SolutionModifier(p);
    p.close(marker, TreeKind::AskQuery);
}
/// [7] ValuesClause -> ('VALUES' DataBlock)?
pub(super) fn parse_ValuesClause(p: &mut Parser) {
    let marker = p.open();
    if p.at_any(&[TokenKind::VALUES]) {
        p.expect(TokenKind::VALUES);
        parse_DataBlock(p);
    }
    p.close(marker, TreeKind::ValuesClause);
}
/// [8] UpdateUnit -> Update
pub(super) fn parse_UpdateUnit(p: &mut Parser) {
    let marker = p.open();
    parse_Update(p);
    p.close(marker, TreeKind::UpdateUnit);
}
/// [9] Update -> Prologue (UpdateOne (';' Update)?)?
pub(super) fn parse_Update(p: &mut Parser) {
    let marker = p.open();
    parse_Prologue(p);
    if p
        .at_any(
            &[
                TokenKind::WITH,
                TokenKind::CREATE,
                TokenKind::CLEAR,
                TokenKind::MOVE,
                TokenKind::LOAD,
                TokenKind::DELETE_DATA,
                TokenKind::DROP,
                TokenKind::DELETE,
                TokenKind::ADD,
                TokenKind::DELETE_WHERE,
                TokenKind::COPY,
                TokenKind::INSERT_DATA,
                TokenKind::INSERT,
            ],
        )
    {
        parse_UpdateOne(p);
        if p.at_any(&[TokenKind::Semicolon]) {
            p.expect(TokenKind::Semicolon);
            parse_Update(p);
        }
    }
    p.close(marker, TreeKind::Update);
}
/// [10] BaseDecl -> 'BASE' 'IRIREF'
pub(super) fn parse_BaseDecl(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::BASE);
    p.expect(TokenKind::IRIREF);
    p.close(marker, TreeKind::BaseDecl);
}
/// [11] PrefixDecl -> 'PREFIX' 'PNAME_NS' 'IRIREF'
pub(super) fn parse_PrefixDecl(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::PREFIX);
    p.expect(TokenKind::PNAME_NS);
    p.expect(TokenKind::IRIREF);
    p.close(marker, TreeKind::PrefixDecl);
}
/// [12] SelectClause -> 'SELECT' ('DISTINCT' | 'REDUCED')? ((Var | '(' Expression 'AS' Var ')') (Var | '(' Expression 'AS' Var ')')* | '*')
pub(super) fn parse_SelectClause(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::SELECT);
    if p.at_any(&[TokenKind::DISTINCT, TokenKind::REDUCED]) {
        match p.nth(0) {
            TokenKind::DISTINCT => {
                p.expect(TokenKind::DISTINCT);
            }
            TokenKind::REDUCED => {
                p.expect(TokenKind::REDUCED);
            }
            _ => {
                p.advance_with_error("Expected ....");
            }
        };
    }
    match p.nth(0) {
        TokenKind::LParen | TokenKind::VAR2 | TokenKind::VAR1 => {
            match p.nth(0) {
                TokenKind::VAR2 | TokenKind::VAR1 => {
                    parse_Var(p);
                }
                TokenKind::LParen => {
                    p.expect(TokenKind::LParen);
                    parse_Expression(p);
                    p.expect(TokenKind::AS);
                    parse_Var(p);
                    p.expect(TokenKind::RParen);
                }
                _ => {
                    p.advance_with_error("Expected ....");
                }
            };
            while [TokenKind::VAR1, TokenKind::VAR2, TokenKind::LParen]
                .contains(&p.nth(0))
            {
                match p.nth(0) {
                    TokenKind::VAR2 | TokenKind::VAR1 => {
                        parse_Var(p);
                    }
                    TokenKind::LParen => {
                        p.expect(TokenKind::LParen);
                        parse_Expression(p);
                        p.expect(TokenKind::AS);
                        parse_Var(p);
                        p.expect(TokenKind::RParen);
                    }
                    _ => {
                        p.advance_with_error("Expected ....");
                    }
                };
            }
        }
        TokenKind::Star => {
            p.expect(TokenKind::Star);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::SelectClause);
}
/// [13] DatasetClause -> 'FROM' (DefaultGraphClause | NamedGraphClause)
pub(super) fn parse_DatasetClause(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::FROM);
    match p.nth(0) {
        TokenKind::IRIREF | TokenKind::PNAME_LN | TokenKind::PNAME_NS => {
            parse_DefaultGraphClause(p);
        }
        TokenKind::NAMED => {
            parse_NamedGraphClause(p);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::DatasetClause);
}
/// [14] WhereClause -> 'WHERE'? GroupGraphPattern
pub(super) fn parse_WhereClause(p: &mut Parser) {
    let marker = p.open();
    if p.at_any(&[TokenKind::WHERE]) {
        p.expect(TokenKind::WHERE);
    }
    parse_GroupGraphPattern(p);
    p.close(marker, TreeKind::WhereClause);
}
/// [15] SolutionModifier -> GroupClause? HavingClause? OrderClause? LimitOffsetClauses?
pub(super) fn parse_SolutionModifier(p: &mut Parser) {
    let marker = p.open();
    if p.at_any(&[TokenKind::GROUP]) {
        parse_GroupClause(p);
    }
    if p.at_any(&[TokenKind::HAVING]) {
        parse_HavingClause(p);
    }
    if p.at_any(&[TokenKind::ORDER]) {
        parse_OrderClause(p);
    }
    if p.at_any(&[TokenKind::OFFSET, TokenKind::LIMIT]) {
        parse_LimitOffsetClauses(p);
    }
    p.close(marker, TreeKind::SolutionModifier);
}
/// [16] SubSelect -> SelectClause WhereClause SolutionModifier ValuesClause
pub(super) fn parse_SubSelect(p: &mut Parser) {
    let marker = p.open();
    parse_SelectClause(p);
    parse_WhereClause(p);
    parse_SolutionModifier(p);
    parse_ValuesClause(p);
    p.close(marker, TreeKind::SubSelect);
}
/// [17] Var -> 'VAR1' | 'VAR2'
pub(super) fn parse_Var(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::VAR1 => {
            p.expect(TokenKind::VAR1);
        }
        TokenKind::VAR2 => {
            p.expect(TokenKind::VAR2);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::Var);
}
/// [18] Expression -> ConditionalOrExpression
pub(super) fn parse_Expression(p: &mut Parser) {
    let marker = p.open();
    parse_ConditionalOrExpression(p);
    p.close(marker, TreeKind::Expression);
}
/// [19] ConstructTemplate -> '{' ConstructTriples? '}'
pub(super) fn parse_ConstructTemplate(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::LCurly);
    if p
        .at_any(
            &[
                TokenKind::STRING_LITERAL2,
                TokenKind::VAR2,
                TokenKind::IRIREF,
                TokenKind::DOUBLE,
                TokenKind::DECIMAL_NEGATIVE,
                TokenKind::DECIMAL,
                TokenKind::ANON,
                TokenKind::LBrack,
                TokenKind::STRING_LITERAL_LONG2,
                TokenKind::LParen,
                TokenKind::STRING_LITERAL_LONG1,
                TokenKind::BLANK_NODE_LABEL,
                TokenKind::STRING_LITERAL1,
                TokenKind::INTEGER_POSITIVE,
                TokenKind::DOUBLE_NEGATIVE,
                TokenKind::PNAME_NS,
                TokenKind::VAR1,
                TokenKind::True,
                TokenKind::False,
                TokenKind::DOUBLE_POSITIVE,
                TokenKind::INTEGER_NEGATIVE,
                TokenKind::INTEGER,
                TokenKind::NIL,
                TokenKind::PNAME_LN,
                TokenKind::DECIMAL_POSITIVE,
            ],
        )
    {
        parse_ConstructTriples(p);
    }
    p.expect(TokenKind::RCurly);
    p.close(marker, TreeKind::ConstructTemplate);
}
/// [20] TriplesTemplate -> TriplesSameSubject ('.' TriplesTemplate?)?
pub(super) fn parse_TriplesTemplate(p: &mut Parser) {
    let marker = p.open();
    parse_TriplesSameSubject(p);
    if p.at_any(&[TokenKind::Dot]) {
        p.expect(TokenKind::Dot);
        if p
            .at_any(
                &[
                    TokenKind::STRING_LITERAL1,
                    TokenKind::DOUBLE,
                    TokenKind::VAR2,
                    TokenKind::PNAME_NS,
                    TokenKind::PNAME_LN,
                    TokenKind::DOUBLE_NEGATIVE,
                    TokenKind::STRING_LITERAL2,
                    TokenKind::NIL,
                    TokenKind::ANON,
                    TokenKind::STRING_LITERAL_LONG2,
                    TokenKind::LParen,
                    TokenKind::DECIMAL_POSITIVE,
                    TokenKind::DOUBLE_POSITIVE,
                    TokenKind::INTEGER,
                    TokenKind::STRING_LITERAL_LONG1,
                    TokenKind::DECIMAL_NEGATIVE,
                    TokenKind::LBrack,
                    TokenKind::BLANK_NODE_LABEL,
                    TokenKind::False,
                    TokenKind::True,
                    TokenKind::INTEGER_POSITIVE,
                    TokenKind::IRIREF,
                    TokenKind::DECIMAL,
                    TokenKind::VAR1,
                    TokenKind::INTEGER_NEGATIVE,
                ],
            )
        {
            parse_TriplesTemplate(p);
        }
    }
    p.close(marker, TreeKind::TriplesTemplate);
}
/// [21] VarOrIri -> Var | iri
pub(super) fn parse_VarOrIri(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::VAR2 | TokenKind::VAR1 => {
            parse_Var(p);
        }
        TokenKind::IRIREF | TokenKind::PNAME_LN | TokenKind::PNAME_NS => {
            parse_iri(p);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::VarOrIri);
}
/// [22] DefaultGraphClause -> SourceSelector
pub(super) fn parse_DefaultGraphClause(p: &mut Parser) {
    let marker = p.open();
    parse_SourceSelector(p);
    p.close(marker, TreeKind::DefaultGraphClause);
}
/// [23] NamedGraphClause -> 'NAMED' SourceSelector
pub(super) fn parse_NamedGraphClause(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::NAMED);
    parse_SourceSelector(p);
    p.close(marker, TreeKind::NamedGraphClause);
}
/// [24] SourceSelector -> iri
pub(super) fn parse_SourceSelector(p: &mut Parser) {
    let marker = p.open();
    parse_iri(p);
    p.close(marker, TreeKind::SourceSelector);
}
/// [25] iri -> 'IRIREF' | PrefixedName
pub(super) fn parse_iri(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::IRIREF => {
            p.expect(TokenKind::IRIREF);
        }
        TokenKind::PNAME_LN | TokenKind::PNAME_NS => {
            parse_PrefixedName(p);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::iri);
}
/// [26] GroupGraphPattern -> '{' (SubSelect | GroupGraphPatternSub) '}'
pub(super) fn parse_GroupGraphPattern(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::LCurly);
    match p.nth(0) {
        TokenKind::SELECT => {
            parse_SubSelect(p);
        }
        TokenKind::INTEGER_NEGATIVE
        | TokenKind::ANON
        | TokenKind::STRING_LITERAL1
        | TokenKind::VALUES
        | TokenKind::LBrack
        | TokenKind::DECIMAL_NEGATIVE
        | TokenKind::PNAME_NS
        | TokenKind::DOUBLE_NEGATIVE
        | TokenKind::STRING_LITERAL_LONG2
        | TokenKind::BLANK_NODE_LABEL
        | TokenKind::STRING_LITERAL_LONG1
        | TokenKind::SERVICE
        | TokenKind::DOUBLE
        | TokenKind::DOUBLE_POSITIVE
        | TokenKind::LParen
        | TokenKind::False
        | TokenKind::FILTER
        | TokenKind::PNAME_LN
        | TokenKind::IRIREF
        | TokenKind::LCurly
        | TokenKind::STRING_LITERAL2
        | TokenKind::GRAPH
        | TokenKind::MINUS
        | TokenKind::BIND
        | TokenKind::VAR1
        | TokenKind::INTEGER_POSITIVE
        | TokenKind::NIL
        | TokenKind::DECIMAL_POSITIVE
        | TokenKind::VAR2
        | TokenKind::True
        | TokenKind::INTEGER
        | TokenKind::DECIMAL
        | TokenKind::OPTIONAL => {
            parse_GroupGraphPatternSub(p);
        }
        _ => {}
    };
    p.expect(TokenKind::RCurly);
    p.close(marker, TreeKind::GroupGraphPattern);
}
/// [27] GroupClause -> 'GROUP' 'BY' GroupCondition GroupCondition*
pub(super) fn parse_GroupClause(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::GROUP);
    p.expect(TokenKind::BY);
    parse_GroupCondition(p);
    while [
        TokenKind::NOW,
        TokenKind::PNAME_NS,
        TokenKind::isNUMERIC,
        TokenKind::ENCODE_FOR_URI,
        TokenKind::IF,
        TokenKind::LParen,
        TokenKind::STRUUID,
        TokenKind::TIMEZONE,
        TokenKind::DATATYPE,
        TokenKind::STRAFTER,
        TokenKind::LANGMATCHES,
        TokenKind::UUID,
        TokenKind::SHA256,
        TokenKind::isIRI,
        TokenKind::isLITERAL,
        TokenKind::UCASE,
        TokenKind::STRSTARTS,
        TokenKind::COALESCE,
        TokenKind::MIN,
        TokenKind::RAND,
        TokenKind::SHA384,
        TokenKind::MAX,
        TokenKind::GROUP_CONCAT,
        TokenKind::SECONDS,
        TokenKind::FLOOR,
        TokenKind::STRENDS,
        TokenKind::isBLANK,
        TokenKind::PNAME_LN,
        TokenKind::ROUND,
        TokenKind::HOURS,
        TokenKind::STRLANG,
        TokenKind::REPLACE,
        TokenKind::sameTerm,
        TokenKind::CONCAT,
        TokenKind::STRDT,
        TokenKind::IRI,
        TokenKind::YEAR,
        TokenKind::SHA1,
        TokenKind::NOT,
        TokenKind::MD5,
        TokenKind::REGEX,
        TokenKind::TZ,
        TokenKind::LANG,
        TokenKind::BOUND,
        TokenKind::MONTH,
        TokenKind::AVG,
        TokenKind::isURI,
        TokenKind::COUNT,
        TokenKind::VAR2,
        TokenKind::DAY,
        TokenKind::STRLEN,
        TokenKind::CONTAINS,
        TokenKind::STRBEFORE,
        TokenKind::SUM,
        TokenKind::SHA512,
        TokenKind::SUBSTR,
        TokenKind::STR,
        TokenKind::ABS,
        TokenKind::BNODE,
        TokenKind::URI,
        TokenKind::IRIREF,
        TokenKind::VAR1,
        TokenKind::LCASE,
        TokenKind::CEIL,
        TokenKind::EXISTS,
        TokenKind::MINUTES,
        TokenKind::SAMPLE,
    ]
        .contains(&p.nth(0))
    {
        parse_GroupCondition(p);
    }
    p.close(marker, TreeKind::GroupClause);
}
/// [28] HavingClause -> 'HAVING' HavingCondition HavingCondition*
pub(super) fn parse_HavingClause(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::HAVING);
    parse_HavingCondition(p);
    while [
        TokenKind::SECONDS,
        TokenKind::SUM,
        TokenKind::SHA1,
        TokenKind::LANGMATCHES,
        TokenKind::STR,
        TokenKind::STRAFTER,
        TokenKind::MAX,
        TokenKind::CONTAINS,
        TokenKind::MINUTES,
        TokenKind::MONTH,
        TokenKind::SHA512,
        TokenKind::ABS,
        TokenKind::BNODE,
        TokenKind::isBLANK,
        TokenKind::YEAR,
        TokenKind::IF,
        TokenKind::SHA256,
        TokenKind::AVG,
        TokenKind::DAY,
        TokenKind::ENCODE_FOR_URI,
        TokenKind::NOW,
        TokenKind::isNUMERIC,
        TokenKind::LCASE,
        TokenKind::REPLACE,
        TokenKind::MD5,
        TokenKind::STRLANG,
        TokenKind::UUID,
        TokenKind::PNAME_NS,
        TokenKind::MIN,
        TokenKind::sameTerm,
        TokenKind::SHA384,
        TokenKind::isLITERAL,
        TokenKind::UCASE,
        TokenKind::DATATYPE,
        TokenKind::COUNT,
        TokenKind::IRI,
        TokenKind::GROUP_CONCAT,
        TokenKind::STRLEN,
        TokenKind::SUBSTR,
        TokenKind::URI,
        TokenKind::SAMPLE,
        TokenKind::isURI,
        TokenKind::HOURS,
        TokenKind::IRIREF,
        TokenKind::isIRI,
        TokenKind::TIMEZONE,
        TokenKind::CONCAT,
        TokenKind::STRSTARTS,
        TokenKind::CEIL,
        TokenKind::COALESCE,
        TokenKind::LANG,
        TokenKind::BOUND,
        TokenKind::STRDT,
        TokenKind::TZ,
        TokenKind::STRUUID,
        TokenKind::STRENDS,
        TokenKind::LParen,
        TokenKind::STRBEFORE,
        TokenKind::FLOOR,
        TokenKind::EXISTS,
        TokenKind::ROUND,
        TokenKind::REGEX,
        TokenKind::RAND,
        TokenKind::NOT,
        TokenKind::PNAME_LN,
    ]
        .contains(&p.nth(0))
    {
        parse_HavingCondition(p);
    }
    p.close(marker, TreeKind::HavingClause);
}
/// [29] OrderClause -> 'ORDER' 'BY' OrderCondition OrderCondition*
pub(super) fn parse_OrderClause(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::ORDER);
    p.expect(TokenKind::BY);
    parse_OrderCondition(p);
    while [
        TokenKind::ASC,
        TokenKind::DESC,
        TokenKind::TZ,
        TokenKind::COALESCE,
        TokenKind::SUM,
        TokenKind::NOT,
        TokenKind::PNAME_NS,
        TokenKind::VAR1,
        TokenKind::STRBEFORE,
        TokenKind::STRSTARTS,
        TokenKind::isNUMERIC,
        TokenKind::SUBSTR,
        TokenKind::AVG,
        TokenKind::COUNT,
        TokenKind::RAND,
        TokenKind::isURI,
        TokenKind::MINUTES,
        TokenKind::STR,
        TokenKind::LCASE,
        TokenKind::STRLANG,
        TokenKind::MIN,
        TokenKind::BOUND,
        TokenKind::CEIL,
        TokenKind::DAY,
        TokenKind::NOW,
        TokenKind::VAR2,
        TokenKind::isLITERAL,
        TokenKind::STRDT,
        TokenKind::MONTH,
        TokenKind::MD5,
        TokenKind::LParen,
        TokenKind::MAX,
        TokenKind::REGEX,
        TokenKind::STRLEN,
        TokenKind::CONCAT,
        TokenKind::SAMPLE,
        TokenKind::EXISTS,
        TokenKind::TIMEZONE,
        TokenKind::REPLACE,
        TokenKind::CONTAINS,
        TokenKind::SHA512,
        TokenKind::SHA384,
        TokenKind::PNAME_LN,
        TokenKind::UCASE,
        TokenKind::URI,
        TokenKind::IRIREF,
        TokenKind::isBLANK,
        TokenKind::GROUP_CONCAT,
        TokenKind::isIRI,
        TokenKind::STRENDS,
        TokenKind::UUID,
        TokenKind::ABS,
        TokenKind::DATATYPE,
        TokenKind::SHA1,
        TokenKind::BNODE,
        TokenKind::SHA256,
        TokenKind::YEAR,
        TokenKind::IRI,
        TokenKind::SECONDS,
        TokenKind::STRAFTER,
        TokenKind::STRUUID,
        TokenKind::FLOOR,
        TokenKind::ENCODE_FOR_URI,
        TokenKind::HOURS,
        TokenKind::IF,
        TokenKind::LANGMATCHES,
        TokenKind::sameTerm,
        TokenKind::LANG,
        TokenKind::ROUND,
    ]
        .contains(&p.nth(0))
    {
        parse_OrderCondition(p);
    }
    p.close(marker, TreeKind::OrderClause);
}
/// [30] LimitOffsetClauses -> LimitClause OffsetClause? | OffsetClause LimitClause?
pub(super) fn parse_LimitOffsetClauses(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::LIMIT => {
            parse_LimitClause(p);
            if p.at_any(&[TokenKind::OFFSET]) {
                parse_OffsetClause(p);
            }
        }
        TokenKind::OFFSET => {
            parse_OffsetClause(p);
            if p.at_any(&[TokenKind::LIMIT]) {
                parse_LimitClause(p);
            }
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::LimitOffsetClauses);
}
/// [31] GroupCondition -> BuiltInCall | FunctionCall | '(' Expression ('AS' Var)? ')' | Var
pub(super) fn parse_GroupCondition(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::MINUTES
        | TokenKind::SECONDS
        | TokenKind::TIMEZONE
        | TokenKind::MD5
        | TokenKind::STRLANG
        | TokenKind::CONTAINS
        | TokenKind::isIRI
        | TokenKind::AVG
        | TokenKind::REGEX
        | TokenKind::isNUMERIC
        | TokenKind::BNODE
        | TokenKind::TZ
        | TokenKind::REPLACE
        | TokenKind::STRLEN
        | TokenKind::LANG
        | TokenKind::MAX
        | TokenKind::SUBSTR
        | TokenKind::RAND
        | TokenKind::IF
        | TokenKind::IRI
        | TokenKind::UCASE
        | TokenKind::DATATYPE
        | TokenKind::UUID
        | TokenKind::BOUND
        | TokenKind::CONCAT
        | TokenKind::STRBEFORE
        | TokenKind::SAMPLE
        | TokenKind::MIN
        | TokenKind::EXISTS
        | TokenKind::STRAFTER
        | TokenKind::MONTH
        | TokenKind::DAY
        | TokenKind::SHA1
        | TokenKind::STRENDS
        | TokenKind::ABS
        | TokenKind::SHA384
        | TokenKind::STRSTARTS
        | TokenKind::URI
        | TokenKind::isURI
        | TokenKind::sameTerm
        | TokenKind::ROUND
        | TokenKind::YEAR
        | TokenKind::SHA512
        | TokenKind::ENCODE_FOR_URI
        | TokenKind::NOW
        | TokenKind::LANGMATCHES
        | TokenKind::isBLANK
        | TokenKind::LCASE
        | TokenKind::SUM
        | TokenKind::SHA256
        | TokenKind::STRDT
        | TokenKind::COUNT
        | TokenKind::FLOOR
        | TokenKind::isLITERAL
        | TokenKind::STR
        | TokenKind::CEIL
        | TokenKind::NOT
        | TokenKind::GROUP_CONCAT
        | TokenKind::COALESCE
        | TokenKind::HOURS
        | TokenKind::STRUUID => {
            parse_BuiltInCall(p);
        }
        TokenKind::PNAME_NS | TokenKind::IRIREF | TokenKind::PNAME_LN => {
            parse_FunctionCall(p);
        }
        TokenKind::LParen => {
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            if p.at_any(&[TokenKind::AS]) {
                p.expect(TokenKind::AS);
                parse_Var(p);
            }
            p.expect(TokenKind::RParen);
        }
        TokenKind::VAR2 | TokenKind::VAR1 => {
            parse_Var(p);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::GroupCondition);
}
/// [32] BuiltInCall -> Aggregate | 'STR' '(' Expression ')' | 'LANG' '(' Expression ')' | 'LANGMATCHES' '(' Expression ',' Expression ')' | 'DATATYPE' '(' Expression ')' | 'BOUND' '(' Var ')' | 'IRI' '(' Expression ')' | 'URI' '(' Expression ')' | 'BNODE' ('(' Expression ')' | 'NIL') | 'RAND' 'NIL' | 'ABS' '(' Expression ')' | 'CEIL' '(' Expression ')' | 'FLOOR' '(' Expression ')' | 'ROUND' '(' Expression ')' | 'CONCAT' ExpressionList | SubstringExpression | 'STRLEN' '(' Expression ')' | StrReplaceExpression | 'UCASE' '(' Expression ')' | 'LCASE' '(' Expression ')' | 'ENCODE_FOR_URI' '(' Expression ')' | 'CONTAINS' '(' Expression ',' Expression ')' | 'STRSTARTS' '(' Expression ',' Expression ')' | 'STRENDS' '(' Expression ',' Expression ')' | 'STRBEFORE' '(' Expression ',' Expression ')' | 'STRAFTER' '(' Expression ',' Expression ')' | 'YEAR' '(' Expression ')' | 'MONTH' '(' Expression ')' | 'DAY' '(' Expression ')' | 'HOURS' '(' Expression ')' | 'MINUTES' '(' Expression ')' | 'SECONDS' '(' Expression ')' | 'TIMEZONE' '(' Expression ')' | 'TZ' '(' Expression ')' | 'NOW' 'NIL' | 'UUID' 'NIL' | 'STRUUID' 'NIL' | 'MD5' '(' Expression ')' | 'SHA1' '(' Expression ')' | 'SHA256' '(' Expression ')' | 'SHA384' '(' Expression ')' | 'SHA512' '(' Expression ')' | 'COALESCE' ExpressionList | 'IF' '(' Expression ',' Expression ',' Expression ')' | 'STRLANG' '(' Expression ',' Expression ')' | 'STRDT' '(' Expression ',' Expression ')' | 'sameTerm' '(' Expression ',' Expression ')' | 'isIRI' '(' Expression ')' | 'isURI' '(' Expression ')' | 'isBLANK' '(' Expression ')' | 'isLITERAL' '(' Expression ')' | 'isNUMERIC' '(' Expression ')' | RegexExpression | ExistsFunc | NotExistsFunc
pub(super) fn parse_BuiltInCall(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::SAMPLE
        | TokenKind::GROUP_CONCAT
        | TokenKind::MAX
        | TokenKind::MIN
        | TokenKind::COUNT
        | TokenKind::SUM
        | TokenKind::AVG => {
            parse_Aggregate(p);
        }
        TokenKind::STR => {
            p.expect(TokenKind::STR);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::LANG => {
            p.expect(TokenKind::LANG);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::LANGMATCHES => {
            p.expect(TokenKind::LANGMATCHES);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::Colon);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::DATATYPE => {
            p.expect(TokenKind::DATATYPE);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::BOUND => {
            p.expect(TokenKind::BOUND);
            p.expect(TokenKind::LParen);
            parse_Var(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::IRI => {
            p.expect(TokenKind::IRI);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::URI => {
            p.expect(TokenKind::URI);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::BNODE => {
            p.expect(TokenKind::BNODE);
            match p.nth(0) {
                TokenKind::LParen => {
                    p.expect(TokenKind::LParen);
                    parse_Expression(p);
                    p.expect(TokenKind::RParen);
                }
                TokenKind::NIL => {
                    p.expect(TokenKind::NIL);
                }
                _ => {
                    p.advance_with_error("Expected ....");
                }
            };
        }
        TokenKind::RAND => {
            p.expect(TokenKind::RAND);
            p.expect(TokenKind::NIL);
        }
        TokenKind::ABS => {
            p.expect(TokenKind::ABS);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::CEIL => {
            p.expect(TokenKind::CEIL);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::FLOOR => {
            p.expect(TokenKind::FLOOR);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::ROUND => {
            p.expect(TokenKind::ROUND);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::CONCAT => {
            p.expect(TokenKind::CONCAT);
            parse_ExpressionList(p);
        }
        TokenKind::SUBSTR => {
            parse_SubstringExpression(p);
        }
        TokenKind::STRLEN => {
            p.expect(TokenKind::STRLEN);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::REPLACE => {
            parse_StrReplaceExpression(p);
        }
        TokenKind::UCASE => {
            p.expect(TokenKind::UCASE);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::LCASE => {
            p.expect(TokenKind::LCASE);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::ENCODE_FOR_URI => {
            p.expect(TokenKind::ENCODE_FOR_URI);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::CONTAINS => {
            p.expect(TokenKind::CONTAINS);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::Colon);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::STRSTARTS => {
            p.expect(TokenKind::STRSTARTS);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::Colon);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::STRENDS => {
            p.expect(TokenKind::STRENDS);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::Colon);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::STRBEFORE => {
            p.expect(TokenKind::STRBEFORE);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::Colon);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::STRAFTER => {
            p.expect(TokenKind::STRAFTER);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::Colon);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::YEAR => {
            p.expect(TokenKind::YEAR);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::MONTH => {
            p.expect(TokenKind::MONTH);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::DAY => {
            p.expect(TokenKind::DAY);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::HOURS => {
            p.expect(TokenKind::HOURS);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::MINUTES => {
            p.expect(TokenKind::MINUTES);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::SECONDS => {
            p.expect(TokenKind::SECONDS);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::TIMEZONE => {
            p.expect(TokenKind::TIMEZONE);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::TZ => {
            p.expect(TokenKind::TZ);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::NOW => {
            p.expect(TokenKind::NOW);
            p.expect(TokenKind::NIL);
        }
        TokenKind::UUID => {
            p.expect(TokenKind::UUID);
            p.expect(TokenKind::NIL);
        }
        TokenKind::STRUUID => {
            p.expect(TokenKind::STRUUID);
            p.expect(TokenKind::NIL);
        }
        TokenKind::MD5 => {
            p.expect(TokenKind::MD5);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::SHA1 => {
            p.expect(TokenKind::SHA1);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::SHA256 => {
            p.expect(TokenKind::SHA256);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::SHA384 => {
            p.expect(TokenKind::SHA384);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::SHA512 => {
            p.expect(TokenKind::SHA512);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::COALESCE => {
            p.expect(TokenKind::COALESCE);
            parse_ExpressionList(p);
        }
        TokenKind::IF => {
            p.expect(TokenKind::IF);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::Colon);
            parse_Expression(p);
            p.expect(TokenKind::Colon);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::STRLANG => {
            p.expect(TokenKind::STRLANG);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::Colon);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::STRDT => {
            p.expect(TokenKind::STRDT);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::Colon);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::sameTerm => {
            p.expect(TokenKind::sameTerm);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::Colon);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::isIRI => {
            p.expect(TokenKind::isIRI);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::isURI => {
            p.expect(TokenKind::isURI);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::isBLANK => {
            p.expect(TokenKind::isBLANK);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::isLITERAL => {
            p.expect(TokenKind::isLITERAL);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::isNUMERIC => {
            p.expect(TokenKind::isNUMERIC);
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::REGEX => {
            parse_RegexExpression(p);
        }
        TokenKind::EXISTS => {
            parse_ExistsFunc(p);
        }
        TokenKind::NOT => {
            parse_NotExistsFunc(p);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::BuiltInCall);
}
/// [33] FunctionCall -> iri ArgList
pub(super) fn parse_FunctionCall(p: &mut Parser) {
    let marker = p.open();
    parse_iri(p);
    parse_ArgList(p);
    p.close(marker, TreeKind::FunctionCall);
}
/// [34] HavingCondition -> Constraint
pub(super) fn parse_HavingCondition(p: &mut Parser) {
    let marker = p.open();
    parse_Constraint(p);
    p.close(marker, TreeKind::HavingCondition);
}
/// [35] Constraint -> BrackettedExpression | BuiltInCall | FunctionCall
pub(super) fn parse_Constraint(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::LParen => {
            parse_BrackettedExpression(p);
        }
        TokenKind::MINUTES
        | TokenKind::SECONDS
        | TokenKind::TIMEZONE
        | TokenKind::MD5
        | TokenKind::STRLANG
        | TokenKind::CONTAINS
        | TokenKind::isIRI
        | TokenKind::AVG
        | TokenKind::REGEX
        | TokenKind::isNUMERIC
        | TokenKind::BNODE
        | TokenKind::TZ
        | TokenKind::REPLACE
        | TokenKind::STRLEN
        | TokenKind::LANG
        | TokenKind::MAX
        | TokenKind::SUBSTR
        | TokenKind::RAND
        | TokenKind::IF
        | TokenKind::IRI
        | TokenKind::UCASE
        | TokenKind::DATATYPE
        | TokenKind::UUID
        | TokenKind::BOUND
        | TokenKind::CONCAT
        | TokenKind::STRBEFORE
        | TokenKind::SAMPLE
        | TokenKind::MIN
        | TokenKind::EXISTS
        | TokenKind::STRAFTER
        | TokenKind::MONTH
        | TokenKind::DAY
        | TokenKind::SHA1
        | TokenKind::STRENDS
        | TokenKind::ABS
        | TokenKind::SHA384
        | TokenKind::STRSTARTS
        | TokenKind::URI
        | TokenKind::isURI
        | TokenKind::sameTerm
        | TokenKind::ROUND
        | TokenKind::YEAR
        | TokenKind::SHA512
        | TokenKind::ENCODE_FOR_URI
        | TokenKind::NOW
        | TokenKind::LANGMATCHES
        | TokenKind::isBLANK
        | TokenKind::LCASE
        | TokenKind::SUM
        | TokenKind::SHA256
        | TokenKind::STRDT
        | TokenKind::COUNT
        | TokenKind::FLOOR
        | TokenKind::isLITERAL
        | TokenKind::STR
        | TokenKind::CEIL
        | TokenKind::NOT
        | TokenKind::GROUP_CONCAT
        | TokenKind::COALESCE
        | TokenKind::HOURS
        | TokenKind::STRUUID => {
            parse_BuiltInCall(p);
        }
        TokenKind::PNAME_NS | TokenKind::IRIREF | TokenKind::PNAME_LN => {
            parse_FunctionCall(p);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::Constraint);
}
/// [36] OrderCondition -> ('ASC' | 'DESC') BrackettedExpression | Constraint | Var
pub(super) fn parse_OrderCondition(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::DESC | TokenKind::ASC => {
            match p.nth(0) {
                TokenKind::ASC => {
                    p.expect(TokenKind::ASC);
                }
                TokenKind::DESC => {
                    p.expect(TokenKind::DESC);
                }
                _ => {
                    p.advance_with_error("Expected ....");
                }
            };
            parse_BrackettedExpression(p);
        }
        TokenKind::TZ
        | TokenKind::CONTAINS
        | TokenKind::SHA384
        | TokenKind::LANGMATCHES
        | TokenKind::YEAR
        | TokenKind::STRSTARTS
        | TokenKind::LANG
        | TokenKind::SHA512
        | TokenKind::CONCAT
        | TokenKind::isLITERAL
        | TokenKind::IF
        | TokenKind::DAY
        | TokenKind::PNAME_NS
        | TokenKind::DATATYPE
        | TokenKind::FLOOR
        | TokenKind::isBLANK
        | TokenKind::IRI
        | TokenKind::sameTerm
        | TokenKind::EXISTS
        | TokenKind::SUM
        | TokenKind::STRENDS
        | TokenKind::BNODE
        | TokenKind::ABS
        | TokenKind::COALESCE
        | TokenKind::ROUND
        | TokenKind::UCASE
        | TokenKind::REPLACE
        | TokenKind::isURI
        | TokenKind::NOW
        | TokenKind::STRBEFORE
        | TokenKind::STRUUID
        | TokenKind::REGEX
        | TokenKind::isNUMERIC
        | TokenKind::VAR1
        | TokenKind::MIN
        | TokenKind::URI
        | TokenKind::IRIREF
        | TokenKind::BOUND
        | TokenKind::COUNT
        | TokenKind::NOT
        | TokenKind::SAMPLE
        | TokenKind::SUBSTR
        | TokenKind::CEIL
        | TokenKind::STRLANG
        | TokenKind::MINUTES
        | TokenKind::HOURS
        | TokenKind::LParen
        | TokenKind::VAR2
        | TokenKind::MONTH
        | TokenKind::MAX
        | TokenKind::RAND
        | TokenKind::STRAFTER
        | TokenKind::SHA256
        | TokenKind::SECONDS
        | TokenKind::LCASE
        | TokenKind::UUID
        | TokenKind::STRLEN
        | TokenKind::isIRI
        | TokenKind::STRDT
        | TokenKind::ENCODE_FOR_URI
        | TokenKind::AVG
        | TokenKind::SHA1
        | TokenKind::PNAME_LN
        | TokenKind::MD5
        | TokenKind::TIMEZONE
        | TokenKind::GROUP_CONCAT
        | TokenKind::STR => {
            match p.nth(0) {
                TokenKind::SECONDS
                | TokenKind::SUM
                | TokenKind::SHA1
                | TokenKind::LANGMATCHES
                | TokenKind::STR
                | TokenKind::STRAFTER
                | TokenKind::MAX
                | TokenKind::CONTAINS
                | TokenKind::MINUTES
                | TokenKind::MONTH
                | TokenKind::SHA512
                | TokenKind::ABS
                | TokenKind::BNODE
                | TokenKind::isBLANK
                | TokenKind::YEAR
                | TokenKind::IF
                | TokenKind::SHA256
                | TokenKind::AVG
                | TokenKind::DAY
                | TokenKind::ENCODE_FOR_URI
                | TokenKind::NOW
                | TokenKind::isNUMERIC
                | TokenKind::LCASE
                | TokenKind::REPLACE
                | TokenKind::MD5
                | TokenKind::STRLANG
                | TokenKind::UUID
                | TokenKind::PNAME_NS
                | TokenKind::MIN
                | TokenKind::sameTerm
                | TokenKind::SHA384
                | TokenKind::isLITERAL
                | TokenKind::UCASE
                | TokenKind::DATATYPE
                | TokenKind::COUNT
                | TokenKind::IRI
                | TokenKind::GROUP_CONCAT
                | TokenKind::STRLEN
                | TokenKind::SUBSTR
                | TokenKind::URI
                | TokenKind::SAMPLE
                | TokenKind::isURI
                | TokenKind::HOURS
                | TokenKind::IRIREF
                | TokenKind::isIRI
                | TokenKind::TIMEZONE
                | TokenKind::CONCAT
                | TokenKind::STRSTARTS
                | TokenKind::CEIL
                | TokenKind::COALESCE
                | TokenKind::LANG
                | TokenKind::BOUND
                | TokenKind::STRDT
                | TokenKind::TZ
                | TokenKind::STRUUID
                | TokenKind::STRENDS
                | TokenKind::LParen
                | TokenKind::STRBEFORE
                | TokenKind::FLOOR
                | TokenKind::EXISTS
                | TokenKind::ROUND
                | TokenKind::REGEX
                | TokenKind::RAND
                | TokenKind::NOT
                | TokenKind::PNAME_LN => {
                    parse_Constraint(p);
                }
                TokenKind::VAR2 | TokenKind::VAR1 => {
                    parse_Var(p);
                }
                _ => {
                    p.advance_with_error("Expected ....");
                }
            };
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::OrderCondition);
}
/// [37] BrackettedExpression -> '(' Expression ')'
pub(super) fn parse_BrackettedExpression(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::LParen);
    parse_Expression(p);
    p.expect(TokenKind::RParen);
    p.close(marker, TreeKind::BrackettedExpression);
}
/// [38] LimitClause -> 'LIMIT' 'INTEGER'
pub(super) fn parse_LimitClause(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::LIMIT);
    p.expect(TokenKind::INTEGER);
    p.close(marker, TreeKind::LimitClause);
}
/// [39] OffsetClause -> 'OFFSET' 'INTEGER'
pub(super) fn parse_OffsetClause(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::OFFSET);
    p.expect(TokenKind::INTEGER);
    p.close(marker, TreeKind::OffsetClause);
}
/// [40] DataBlock -> InlineDataOneVar | InlineDataFull
pub(super) fn parse_DataBlock(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::VAR1 | TokenKind::VAR2 => {
            parse_InlineDataOneVar(p);
        }
        TokenKind::NIL | TokenKind::LParen => {
            parse_InlineDataFull(p);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::DataBlock);
}
/// [41] UpdateOne -> Load | Clear | Drop | Add | Move | Copy | Create | InsertData | DeleteData | DeleteWhere | Modify
pub(super) fn parse_UpdateOne(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::LOAD => {
            parse_Load(p);
        }
        TokenKind::CLEAR => {
            parse_Clear(p);
        }
        TokenKind::DROP => {
            parse_Drop(p);
        }
        TokenKind::ADD => {
            parse_Add(p);
        }
        TokenKind::MOVE => {
            parse_Move(p);
        }
        TokenKind::COPY => {
            parse_Copy(p);
        }
        TokenKind::CREATE => {
            parse_Create(p);
        }
        TokenKind::INSERT_DATA => {
            parse_InsertData(p);
        }
        TokenKind::DELETE_DATA => {
            parse_DeleteData(p);
        }
        TokenKind::DELETE_WHERE => {
            parse_DeleteWhere(p);
        }
        TokenKind::INSERT | TokenKind::DELETE | TokenKind::WITH => {
            parse_Modify(p);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::UpdateOne);
}
/// [42] Load -> 'LOAD' 'SILENT'? iri ('INTO' GraphRef)?
pub(super) fn parse_Load(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::LOAD);
    if p.at_any(&[TokenKind::SILENT]) {
        p.expect(TokenKind::SILENT);
    }
    parse_iri(p);
    if p.at_any(&[TokenKind::INTO]) {
        p.expect(TokenKind::INTO);
        parse_GraphRef(p);
    }
    p.close(marker, TreeKind::Load);
}
/// [43] Clear -> 'CLEAR' 'SILENT'? GraphRefAll
pub(super) fn parse_Clear(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::CLEAR);
    if p.at_any(&[TokenKind::SILENT]) {
        p.expect(TokenKind::SILENT);
    }
    parse_GraphRefAll(p);
    p.close(marker, TreeKind::Clear);
}
/// [44] Drop -> 'DROP' 'SILENT'? GraphRefAll
pub(super) fn parse_Drop(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::DROP);
    if p.at_any(&[TokenKind::SILENT]) {
        p.expect(TokenKind::SILENT);
    }
    parse_GraphRefAll(p);
    p.close(marker, TreeKind::Drop);
}
/// [45] Add -> 'ADD' 'SILENT'? GraphOrDefault 'TO' GraphOrDefault
pub(super) fn parse_Add(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::ADD);
    if p.at_any(&[TokenKind::SILENT]) {
        p.expect(TokenKind::SILENT);
    }
    parse_GraphOrDefault(p);
    p.expect(TokenKind::TO);
    parse_GraphOrDefault(p);
    p.close(marker, TreeKind::Add);
}
/// [46] Move -> 'MOVE' 'SILENT'? GraphOrDefault 'TO' GraphOrDefault
pub(super) fn parse_Move(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::MOVE);
    if p.at_any(&[TokenKind::SILENT]) {
        p.expect(TokenKind::SILENT);
    }
    parse_GraphOrDefault(p);
    p.expect(TokenKind::TO);
    parse_GraphOrDefault(p);
    p.close(marker, TreeKind::Move);
}
/// [47] Copy -> 'COPY' 'SILENT'? GraphOrDefault 'TO' GraphOrDefault
pub(super) fn parse_Copy(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::COPY);
    if p.at_any(&[TokenKind::SILENT]) {
        p.expect(TokenKind::SILENT);
    }
    parse_GraphOrDefault(p);
    p.expect(TokenKind::TO);
    parse_GraphOrDefault(p);
    p.close(marker, TreeKind::Copy);
}
/// [48] Create -> 'CREATE' 'SILENT'? GraphRef
pub(super) fn parse_Create(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::CREATE);
    if p.at_any(&[TokenKind::SILENT]) {
        p.expect(TokenKind::SILENT);
    }
    parse_GraphRef(p);
    p.close(marker, TreeKind::Create);
}
/// [49] InsertData -> 'INSERT_DATA' QuadData
pub(super) fn parse_InsertData(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::INSERT_DATA);
    parse_QuadData(p);
    p.close(marker, TreeKind::InsertData);
}
/// [50] DeleteData -> 'DELETE_DATA' QuadData
pub(super) fn parse_DeleteData(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::DELETE_DATA);
    parse_QuadData(p);
    p.close(marker, TreeKind::DeleteData);
}
/// [51] DeleteWhere -> 'DELETE_WHERE' QuadPattern
pub(super) fn parse_DeleteWhere(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::DELETE_WHERE);
    parse_QuadPattern(p);
    p.close(marker, TreeKind::DeleteWhere);
}
/// [52] Modify -> ('WITH' iri)? (DeleteClause InsertClause? | InsertClause) UsingClause* 'WHERE' GroupGraphPattern
pub(super) fn parse_Modify(p: &mut Parser) {
    let marker = p.open();
    if p.at_any(&[TokenKind::WITH]) {
        p.expect(TokenKind::WITH);
        parse_iri(p);
    }
    match p.nth(0) {
        TokenKind::DELETE => {
            parse_DeleteClause(p);
            if p.at_any(&[TokenKind::INSERT]) {
                parse_InsertClause(p);
            }
        }
        TokenKind::INSERT => {
            parse_InsertClause(p);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    while [TokenKind::USING].contains(&p.nth(0)) {
        parse_UsingClause(p);
    }
    p.expect(TokenKind::WHERE);
    parse_GroupGraphPattern(p);
    p.close(marker, TreeKind::Modify);
}
/// [53] GraphRef -> 'GRAPH' iri
pub(super) fn parse_GraphRef(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::GRAPH);
    parse_iri(p);
    p.close(marker, TreeKind::GraphRef);
}
/// [54] GraphRefAll -> GraphRef | 'DEFAULT' | 'NAMED' | 'ALL'
pub(super) fn parse_GraphRefAll(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::GRAPH => {
            parse_GraphRef(p);
        }
        TokenKind::DEFAULT => {
            p.expect(TokenKind::DEFAULT);
        }
        TokenKind::NAMED => {
            p.expect(TokenKind::NAMED);
        }
        TokenKind::ALL => {
            p.expect(TokenKind::ALL);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::GraphRefAll);
}
/// [55] GraphOrDefault -> 'DEFAULT' | 'GRAPH'? iri
pub(super) fn parse_GraphOrDefault(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::DEFAULT => {
            p.expect(TokenKind::DEFAULT);
        }
        TokenKind::GRAPH
        | TokenKind::PNAME_NS
        | TokenKind::PNAME_LN
        | TokenKind::IRIREF => {
            if p.at_any(&[TokenKind::GRAPH]) {
                p.expect(TokenKind::GRAPH);
            }
            parse_iri(p);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::GraphOrDefault);
}
/// [56] QuadData -> '{' Quads '}'
pub(super) fn parse_QuadData(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::LCurly);
    parse_Quads(p);
    p.expect(TokenKind::RCurly);
    p.close(marker, TreeKind::QuadData);
}
/// [57] QuadPattern -> '{' Quads '}'
pub(super) fn parse_QuadPattern(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::LCurly);
    parse_Quads(p);
    p.expect(TokenKind::RCurly);
    p.close(marker, TreeKind::QuadPattern);
}
/// [58] DeleteClause -> 'DELETE' QuadPattern
pub(super) fn parse_DeleteClause(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::DELETE);
    parse_QuadPattern(p);
    p.close(marker, TreeKind::DeleteClause);
}
/// [59] InsertClause -> 'INSERT' QuadPattern
pub(super) fn parse_InsertClause(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::INSERT);
    parse_QuadPattern(p);
    p.close(marker, TreeKind::InsertClause);
}
/// [60] UsingClause -> 'USING' (iri | 'NAMED' iri)
pub(super) fn parse_UsingClause(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::USING);
    match p.nth(0) {
        TokenKind::IRIREF | TokenKind::PNAME_LN | TokenKind::PNAME_NS => {
            parse_iri(p);
        }
        TokenKind::NAMED => {
            p.expect(TokenKind::NAMED);
            parse_iri(p);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::UsingClause);
}
/// [61] Quads -> TriplesTemplate? (QuadsNotTriples '.'? TriplesTemplate?)*
pub(super) fn parse_Quads(p: &mut Parser) {
    let marker = p.open();
    if p
        .at_any(
            &[
                TokenKind::STRING_LITERAL1,
                TokenKind::DOUBLE,
                TokenKind::VAR2,
                TokenKind::PNAME_NS,
                TokenKind::PNAME_LN,
                TokenKind::DOUBLE_NEGATIVE,
                TokenKind::STRING_LITERAL2,
                TokenKind::NIL,
                TokenKind::ANON,
                TokenKind::STRING_LITERAL_LONG2,
                TokenKind::LParen,
                TokenKind::DECIMAL_POSITIVE,
                TokenKind::DOUBLE_POSITIVE,
                TokenKind::INTEGER,
                TokenKind::STRING_LITERAL_LONG1,
                TokenKind::DECIMAL_NEGATIVE,
                TokenKind::LBrack,
                TokenKind::BLANK_NODE_LABEL,
                TokenKind::False,
                TokenKind::True,
                TokenKind::INTEGER_POSITIVE,
                TokenKind::IRIREF,
                TokenKind::DECIMAL,
                TokenKind::VAR1,
                TokenKind::INTEGER_NEGATIVE,
            ],
        )
    {
        parse_TriplesTemplate(p);
    }
    while [TokenKind::GRAPH].contains(&p.nth(0)) {
        parse_QuadsNotTriples(p);
        if p.at_any(&[TokenKind::Dot]) {
            p.expect(TokenKind::Dot);
        }
        if p
            .at_any(
                &[
                    TokenKind::STRING_LITERAL1,
                    TokenKind::DOUBLE,
                    TokenKind::VAR2,
                    TokenKind::PNAME_NS,
                    TokenKind::PNAME_LN,
                    TokenKind::DOUBLE_NEGATIVE,
                    TokenKind::STRING_LITERAL2,
                    TokenKind::NIL,
                    TokenKind::ANON,
                    TokenKind::STRING_LITERAL_LONG2,
                    TokenKind::LParen,
                    TokenKind::DECIMAL_POSITIVE,
                    TokenKind::DOUBLE_POSITIVE,
                    TokenKind::INTEGER,
                    TokenKind::STRING_LITERAL_LONG1,
                    TokenKind::DECIMAL_NEGATIVE,
                    TokenKind::LBrack,
                    TokenKind::BLANK_NODE_LABEL,
                    TokenKind::False,
                    TokenKind::True,
                    TokenKind::INTEGER_POSITIVE,
                    TokenKind::IRIREF,
                    TokenKind::DECIMAL,
                    TokenKind::VAR1,
                    TokenKind::INTEGER_NEGATIVE,
                ],
            )
        {
            parse_TriplesTemplate(p);
        }
    }
    p.close(marker, TreeKind::Quads);
}
/// [62] QuadsNotTriples -> 'GRAPH' VarOrIri '{' TriplesTemplate? '}'
pub(super) fn parse_QuadsNotTriples(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::GRAPH);
    parse_VarOrIri(p);
    p.expect(TokenKind::LCurly);
    if p
        .at_any(
            &[
                TokenKind::STRING_LITERAL1,
                TokenKind::DOUBLE,
                TokenKind::VAR2,
                TokenKind::PNAME_NS,
                TokenKind::PNAME_LN,
                TokenKind::DOUBLE_NEGATIVE,
                TokenKind::STRING_LITERAL2,
                TokenKind::NIL,
                TokenKind::ANON,
                TokenKind::STRING_LITERAL_LONG2,
                TokenKind::LParen,
                TokenKind::DECIMAL_POSITIVE,
                TokenKind::DOUBLE_POSITIVE,
                TokenKind::INTEGER,
                TokenKind::STRING_LITERAL_LONG1,
                TokenKind::DECIMAL_NEGATIVE,
                TokenKind::LBrack,
                TokenKind::BLANK_NODE_LABEL,
                TokenKind::False,
                TokenKind::True,
                TokenKind::INTEGER_POSITIVE,
                TokenKind::IRIREF,
                TokenKind::DECIMAL,
                TokenKind::VAR1,
                TokenKind::INTEGER_NEGATIVE,
            ],
        )
    {
        parse_TriplesTemplate(p);
    }
    p.expect(TokenKind::RCurly);
    p.close(marker, TreeKind::QuadsNotTriples);
}
/// [63] TriplesSameSubject -> VarOrTerm PropertyListNotEmpty | TriplesNode PropertyList
pub(super) fn parse_TriplesSameSubject(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::INTEGER_POSITIVE
        | TokenKind::STRING_LITERAL1
        | TokenKind::DECIMAL_POSITIVE
        | TokenKind::STRING_LITERAL_LONG1
        | TokenKind::ANON
        | TokenKind::STRING_LITERAL_LONG2
        | TokenKind::DOUBLE_POSITIVE
        | TokenKind::INTEGER
        | TokenKind::DOUBLE
        | TokenKind::NIL
        | TokenKind::STRING_LITERAL2
        | TokenKind::INTEGER_NEGATIVE
        | TokenKind::False
        | TokenKind::PNAME_LN
        | TokenKind::IRIREF
        | TokenKind::VAR1
        | TokenKind::BLANK_NODE_LABEL
        | TokenKind::PNAME_NS
        | TokenKind::True
        | TokenKind::VAR2
        | TokenKind::DECIMAL_NEGATIVE
        | TokenKind::DOUBLE_NEGATIVE
        | TokenKind::DECIMAL => {
            parse_VarOrTerm(p);
            parse_PropertyListNotEmpty(p);
        }
        TokenKind::LBrack | TokenKind::LParen => {
            parse_TriplesNode(p);
            parse_PropertyList(p);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::TriplesSameSubject);
}
/// [64] GroupGraphPatternSub -> TriplesBlock? (GraphPatternNotTriples '.'? TriplesBlock?)*
pub(super) fn parse_GroupGraphPatternSub(p: &mut Parser) {
    let marker = p.open();
    if p
        .at_any(
            &[
                TokenKind::BLANK_NODE_LABEL,
                TokenKind::INTEGER,
                TokenKind::DECIMAL_NEGATIVE,
                TokenKind::False,
                TokenKind::DOUBLE,
                TokenKind::LParen,
                TokenKind::True,
                TokenKind::INTEGER_POSITIVE,
                TokenKind::STRING_LITERAL_LONG2,
                TokenKind::DECIMAL_POSITIVE,
                TokenKind::VAR1,
                TokenKind::STRING_LITERAL_LONG1,
                TokenKind::PNAME_NS,
                TokenKind::INTEGER_NEGATIVE,
                TokenKind::DOUBLE_NEGATIVE,
                TokenKind::NIL,
                TokenKind::DECIMAL,
                TokenKind::STRING_LITERAL1,
                TokenKind::LBrack,
                TokenKind::VAR2,
                TokenKind::PNAME_LN,
                TokenKind::DOUBLE_POSITIVE,
                TokenKind::IRIREF,
                TokenKind::ANON,
                TokenKind::STRING_LITERAL2,
            ],
        )
    {
        parse_TriplesBlock(p);
    }
    while [
        TokenKind::GRAPH,
        TokenKind::MINUS,
        TokenKind::FILTER,
        TokenKind::VALUES,
        TokenKind::BIND,
        TokenKind::SERVICE,
        TokenKind::LCurly,
        TokenKind::OPTIONAL,
    ]
        .contains(&p.nth(0))
    {
        parse_GraphPatternNotTriples(p);
        if p.at_any(&[TokenKind::Dot]) {
            p.expect(TokenKind::Dot);
        }
        if p
            .at_any(
                &[
                    TokenKind::BLANK_NODE_LABEL,
                    TokenKind::INTEGER,
                    TokenKind::DECIMAL_NEGATIVE,
                    TokenKind::False,
                    TokenKind::DOUBLE,
                    TokenKind::LParen,
                    TokenKind::True,
                    TokenKind::INTEGER_POSITIVE,
                    TokenKind::STRING_LITERAL_LONG2,
                    TokenKind::DECIMAL_POSITIVE,
                    TokenKind::VAR1,
                    TokenKind::STRING_LITERAL_LONG1,
                    TokenKind::PNAME_NS,
                    TokenKind::INTEGER_NEGATIVE,
                    TokenKind::DOUBLE_NEGATIVE,
                    TokenKind::NIL,
                    TokenKind::DECIMAL,
                    TokenKind::STRING_LITERAL1,
                    TokenKind::LBrack,
                    TokenKind::VAR2,
                    TokenKind::PNAME_LN,
                    TokenKind::DOUBLE_POSITIVE,
                    TokenKind::IRIREF,
                    TokenKind::ANON,
                    TokenKind::STRING_LITERAL2,
                ],
            )
        {
            parse_TriplesBlock(p);
        }
    }
    p.close(marker, TreeKind::GroupGraphPatternSub);
}
/// [65] TriplesBlock -> TriplesSameSubjectPath ('.' TriplesBlock?)?
pub(super) fn parse_TriplesBlock(p: &mut Parser) {
    let marker = p.open();
    parse_TriplesSameSubjectPath(p);
    if p.at_any(&[TokenKind::Dot]) {
        p.expect(TokenKind::Dot);
        if p
            .at_any(
                &[
                    TokenKind::BLANK_NODE_LABEL,
                    TokenKind::INTEGER,
                    TokenKind::DECIMAL_NEGATIVE,
                    TokenKind::False,
                    TokenKind::DOUBLE,
                    TokenKind::LParen,
                    TokenKind::True,
                    TokenKind::INTEGER_POSITIVE,
                    TokenKind::STRING_LITERAL_LONG2,
                    TokenKind::DECIMAL_POSITIVE,
                    TokenKind::VAR1,
                    TokenKind::STRING_LITERAL_LONG1,
                    TokenKind::PNAME_NS,
                    TokenKind::INTEGER_NEGATIVE,
                    TokenKind::DOUBLE_NEGATIVE,
                    TokenKind::NIL,
                    TokenKind::DECIMAL,
                    TokenKind::STRING_LITERAL1,
                    TokenKind::LBrack,
                    TokenKind::VAR2,
                    TokenKind::PNAME_LN,
                    TokenKind::DOUBLE_POSITIVE,
                    TokenKind::IRIREF,
                    TokenKind::ANON,
                    TokenKind::STRING_LITERAL2,
                ],
            )
        {
            parse_TriplesBlock(p);
        }
    }
    p.close(marker, TreeKind::TriplesBlock);
}
/// [66] GraphPatternNotTriples -> GroupOrUnionGraphPattern | OptionalGraphPattern | MinusGraphPattern | GraphGraphPattern | ServiceGraphPattern | Filter | Bind | InlineData
pub(super) fn parse_GraphPatternNotTriples(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::LCurly => {
            parse_GroupOrUnionGraphPattern(p);
        }
        TokenKind::OPTIONAL => {
            parse_OptionalGraphPattern(p);
        }
        TokenKind::MINUS => {
            parse_MinusGraphPattern(p);
        }
        TokenKind::GRAPH => {
            parse_GraphGraphPattern(p);
        }
        TokenKind::SERVICE => {
            parse_ServiceGraphPattern(p);
        }
        TokenKind::FILTER => {
            parse_Filter(p);
        }
        TokenKind::BIND => {
            parse_Bind(p);
        }
        TokenKind::VALUES => {
            parse_InlineData(p);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::GraphPatternNotTriples);
}
/// [67] TriplesSameSubjectPath -> VarOrTerm PropertyListPathNotEmpty | TriplesNodePath PropertyListPath
pub(super) fn parse_TriplesSameSubjectPath(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::NIL
        | TokenKind::VAR2
        | TokenKind::STRING_LITERAL_LONG1
        | TokenKind::STRING_LITERAL1
        | TokenKind::False
        | TokenKind::DOUBLE
        | TokenKind::DOUBLE_POSITIVE
        | TokenKind::IRIREF
        | TokenKind::PNAME_NS
        | TokenKind::True
        | TokenKind::INTEGER
        | TokenKind::PNAME_LN
        | TokenKind::DECIMAL_NEGATIVE
        | TokenKind::DECIMAL_POSITIVE
        | TokenKind::DOUBLE_NEGATIVE
        | TokenKind::BLANK_NODE_LABEL
        | TokenKind::STRING_LITERAL_LONG2
        | TokenKind::DECIMAL
        | TokenKind::STRING_LITERAL2
        | TokenKind::INTEGER_POSITIVE
        | TokenKind::ANON
        | TokenKind::INTEGER_NEGATIVE
        | TokenKind::VAR1 => {
            parse_VarOrTerm(p);
            parse_PropertyListPathNotEmpty(p);
        }
        TokenKind::LBrack | TokenKind::LParen => {
            parse_TriplesNodePath(p);
            parse_PropertyListPath(p);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::TriplesSameSubjectPath);
}
/// [68] GroupOrUnionGraphPattern -> GroupGraphPattern ('UNION' GroupGraphPattern)*
pub(super) fn parse_GroupOrUnionGraphPattern(p: &mut Parser) {
    let marker = p.open();
    parse_GroupGraphPattern(p);
    while [TokenKind::UNION].contains(&p.nth(0)) {
        p.expect(TokenKind::UNION);
        parse_GroupGraphPattern(p);
    }
    p.close(marker, TreeKind::GroupOrUnionGraphPattern);
}
/// [69] OptionalGraphPattern -> 'OPTIONAL' GroupGraphPattern
pub(super) fn parse_OptionalGraphPattern(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::OPTIONAL);
    parse_GroupGraphPattern(p);
    p.close(marker, TreeKind::OptionalGraphPattern);
}
/// [70] MinusGraphPattern -> 'MINUS' GroupGraphPattern
pub(super) fn parse_MinusGraphPattern(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::MINUS);
    parse_GroupGraphPattern(p);
    p.close(marker, TreeKind::MinusGraphPattern);
}
/// [71] GraphGraphPattern -> 'GRAPH' VarOrIri GroupGraphPattern
pub(super) fn parse_GraphGraphPattern(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::GRAPH);
    parse_VarOrIri(p);
    parse_GroupGraphPattern(p);
    p.close(marker, TreeKind::GraphGraphPattern);
}
/// [72] ServiceGraphPattern -> 'SERVICE' 'SILENT'? VarOrIri GroupGraphPattern
pub(super) fn parse_ServiceGraphPattern(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::SERVICE);
    if p.at_any(&[TokenKind::SILENT]) {
        p.expect(TokenKind::SILENT);
    }
    parse_VarOrIri(p);
    parse_GroupGraphPattern(p);
    p.close(marker, TreeKind::ServiceGraphPattern);
}
/// [73] Filter -> 'FILTER' Constraint
pub(super) fn parse_Filter(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::FILTER);
    parse_Constraint(p);
    p.close(marker, TreeKind::Filter);
}
/// [74] Bind -> 'BIND' '(' Expression 'AS' Var ')'
pub(super) fn parse_Bind(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::BIND);
    p.expect(TokenKind::LParen);
    parse_Expression(p);
    p.expect(TokenKind::AS);
    parse_Var(p);
    p.expect(TokenKind::RParen);
    p.close(marker, TreeKind::Bind);
}
/// [75] InlineData -> 'VALUES' DataBlock
pub(super) fn parse_InlineData(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::VALUES);
    parse_DataBlock(p);
    p.close(marker, TreeKind::InlineData);
}
/// [76] InlineDataOneVar -> Var '{' DataBlockValue* '}'
pub(super) fn parse_InlineDataOneVar(p: &mut Parser) {
    let marker = p.open();
    parse_Var(p);
    p.expect(TokenKind::LCurly);
    while [
        TokenKind::DOUBLE,
        TokenKind::True,
        TokenKind::INTEGER,
        TokenKind::DECIMAL_POSITIVE,
        TokenKind::False,
        TokenKind::UNDEF,
        TokenKind::DECIMAL_NEGATIVE,
        TokenKind::INTEGER_POSITIVE,
        TokenKind::STRING_LITERAL_LONG2,
        TokenKind::STRING_LITERAL2,
        TokenKind::STRING_LITERAL1,
        TokenKind::DOUBLE_POSITIVE,
        TokenKind::STRING_LITERAL_LONG1,
        TokenKind::INTEGER_NEGATIVE,
        TokenKind::IRIREF,
        TokenKind::DOUBLE_NEGATIVE,
        TokenKind::DECIMAL,
        TokenKind::PNAME_NS,
        TokenKind::PNAME_LN,
    ]
        .contains(&p.nth(0))
    {
        parse_DataBlockValue(p);
    }
    p.expect(TokenKind::RCurly);
    p.close(marker, TreeKind::InlineDataOneVar);
}
/// [77] InlineDataFull -> ('NIL' | '(' Var* ')') '{' ('(' DataBlockValue* ')' | 'NIL')* '}'
pub(super) fn parse_InlineDataFull(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::NIL => {
            p.expect(TokenKind::NIL);
        }
        TokenKind::LParen => {
            p.expect(TokenKind::LParen);
            while [TokenKind::VAR2, TokenKind::VAR1].contains(&p.nth(0)) {
                parse_Var(p);
            }
            p.expect(TokenKind::RParen);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.expect(TokenKind::LCurly);
    while [TokenKind::LParen, TokenKind::NIL].contains(&p.nth(0)) {
        match p.nth(0) {
            TokenKind::LParen => {
                p.expect(TokenKind::LParen);
                while [
                    TokenKind::DOUBLE,
                    TokenKind::True,
                    TokenKind::INTEGER,
                    TokenKind::DECIMAL_POSITIVE,
                    TokenKind::False,
                    TokenKind::UNDEF,
                    TokenKind::DECIMAL_NEGATIVE,
                    TokenKind::INTEGER_POSITIVE,
                    TokenKind::STRING_LITERAL_LONG2,
                    TokenKind::STRING_LITERAL2,
                    TokenKind::STRING_LITERAL1,
                    TokenKind::DOUBLE_POSITIVE,
                    TokenKind::STRING_LITERAL_LONG1,
                    TokenKind::INTEGER_NEGATIVE,
                    TokenKind::IRIREF,
                    TokenKind::DOUBLE_NEGATIVE,
                    TokenKind::DECIMAL,
                    TokenKind::PNAME_NS,
                    TokenKind::PNAME_LN,
                ]
                    .contains(&p.nth(0))
                {
                    parse_DataBlockValue(p);
                }
                p.expect(TokenKind::RParen);
            }
            TokenKind::NIL => {
                p.expect(TokenKind::NIL);
            }
            _ => {
                p.advance_with_error("Expected ....");
            }
        };
    }
    p.expect(TokenKind::RCurly);
    p.close(marker, TreeKind::InlineDataFull);
}
/// [78] DataBlockValue -> iri | RDFLiteral | NumericLiteral | BooleanLiteral | 'UNDEF'
pub(super) fn parse_DataBlockValue(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::IRIREF | TokenKind::PNAME_LN | TokenKind::PNAME_NS => {
            parse_iri(p);
        }
        TokenKind::STRING_LITERAL_LONG1
        | TokenKind::STRING_LITERAL1
        | TokenKind::STRING_LITERAL2
        | TokenKind::STRING_LITERAL_LONG2 => {
            parse_RDFLiteral(p);
        }
        TokenKind::DOUBLE_POSITIVE
        | TokenKind::DECIMAL_NEGATIVE
        | TokenKind::DOUBLE_NEGATIVE
        | TokenKind::DECIMAL
        | TokenKind::INTEGER
        | TokenKind::DECIMAL_POSITIVE
        | TokenKind::INTEGER_POSITIVE
        | TokenKind::INTEGER_NEGATIVE
        | TokenKind::DOUBLE => {
            parse_NumericLiteral(p);
        }
        TokenKind::True | TokenKind::False => {
            parse_BooleanLiteral(p);
        }
        TokenKind::UNDEF => {
            p.expect(TokenKind::UNDEF);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::DataBlockValue);
}
/// [79] RDFLiteral -> String ('LANGTAG' | '^^' iri)?
pub(super) fn parse_RDFLiteral(p: &mut Parser) {
    let marker = p.open();
    parse_String(p);
    if p.at_any(&[TokenKind::LANGTAG, TokenKind::DoubleZirkumflex]) {
        match p.nth(0) {
            TokenKind::LANGTAG => {
                p.expect(TokenKind::LANGTAG);
            }
            TokenKind::DoubleZirkumflex => {
                p.expect(TokenKind::DoubleZirkumflex);
                parse_iri(p);
            }
            _ => {
                p.advance_with_error("Expected ....");
            }
        };
    }
    p.close(marker, TreeKind::RDFLiteral);
}
/// [80] NumericLiteral -> NumericLiteralUnsigned | NumericLiteralPositive | NumericLiteralNegative
pub(super) fn parse_NumericLiteral(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::DECIMAL | TokenKind::DOUBLE | TokenKind::INTEGER => {
            parse_NumericLiteralUnsigned(p);
        }
        TokenKind::DECIMAL_POSITIVE
        | TokenKind::DOUBLE_POSITIVE
        | TokenKind::INTEGER_POSITIVE => {
            parse_NumericLiteralPositive(p);
        }
        TokenKind::INTEGER_NEGATIVE
        | TokenKind::DECIMAL_NEGATIVE
        | TokenKind::DOUBLE_NEGATIVE => {
            parse_NumericLiteralNegative(p);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::NumericLiteral);
}
/// [81] BooleanLiteral -> 'true' | 'false'
pub(super) fn parse_BooleanLiteral(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::True => {
            p.expect(TokenKind::True);
        }
        TokenKind::False => {
            p.expect(TokenKind::False);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::BooleanLiteral);
}
/// [82] ArgList -> 'NIL' | '(' 'DISTINCT'? Expression (',' Expression)* ')'
pub(super) fn parse_ArgList(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::NIL => {
            p.expect(TokenKind::NIL);
        }
        TokenKind::LParen => {
            p.expect(TokenKind::LParen);
            if p.at_any(&[TokenKind::DISTINCT]) {
                p.expect(TokenKind::DISTINCT);
            }
            parse_Expression(p);
            while [TokenKind::Colon].contains(&p.nth(0)) {
                p.expect(TokenKind::Colon);
                parse_Expression(p);
            }
            p.expect(TokenKind::RParen);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::ArgList);
}
/// [83] ExpressionList -> 'NIL' | '(' Expression (',' Expression)* ')'
pub(super) fn parse_ExpressionList(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::NIL => {
            p.expect(TokenKind::NIL);
        }
        TokenKind::LParen => {
            p.expect(TokenKind::LParen);
            parse_Expression(p);
            while [TokenKind::Colon].contains(&p.nth(0)) {
                p.expect(TokenKind::Colon);
                parse_Expression(p);
            }
            p.expect(TokenKind::RParen);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::ExpressionList);
}
/// [84] ConstructTriples -> TriplesSameSubject ('.' ConstructTriples?)?
pub(super) fn parse_ConstructTriples(p: &mut Parser) {
    let marker = p.open();
    parse_TriplesSameSubject(p);
    if p.at_any(&[TokenKind::Dot]) {
        p.expect(TokenKind::Dot);
        if p
            .at_any(
                &[
                    TokenKind::STRING_LITERAL2,
                    TokenKind::VAR2,
                    TokenKind::IRIREF,
                    TokenKind::DOUBLE,
                    TokenKind::DECIMAL_NEGATIVE,
                    TokenKind::DECIMAL,
                    TokenKind::ANON,
                    TokenKind::LBrack,
                    TokenKind::STRING_LITERAL_LONG2,
                    TokenKind::LParen,
                    TokenKind::STRING_LITERAL_LONG1,
                    TokenKind::BLANK_NODE_LABEL,
                    TokenKind::STRING_LITERAL1,
                    TokenKind::INTEGER_POSITIVE,
                    TokenKind::DOUBLE_NEGATIVE,
                    TokenKind::PNAME_NS,
                    TokenKind::VAR1,
                    TokenKind::True,
                    TokenKind::False,
                    TokenKind::DOUBLE_POSITIVE,
                    TokenKind::INTEGER_NEGATIVE,
                    TokenKind::INTEGER,
                    TokenKind::NIL,
                    TokenKind::PNAME_LN,
                    TokenKind::DECIMAL_POSITIVE,
                ],
            )
        {
            parse_ConstructTriples(p);
        }
    }
    p.close(marker, TreeKind::ConstructTriples);
}
/// [85] VarOrTerm -> Var | GraphTerm
pub(super) fn parse_VarOrTerm(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::VAR2 | TokenKind::VAR1 => {
            parse_Var(p);
        }
        TokenKind::IRIREF
        | TokenKind::STRING_LITERAL_LONG2
        | TokenKind::STRING_LITERAL_LONG1
        | TokenKind::PNAME_LN
        | TokenKind::DECIMAL
        | TokenKind::DOUBLE
        | TokenKind::True
        | TokenKind::NIL
        | TokenKind::DECIMAL_NEGATIVE
        | TokenKind::INTEGER
        | TokenKind::DOUBLE_NEGATIVE
        | TokenKind::DECIMAL_POSITIVE
        | TokenKind::BLANK_NODE_LABEL
        | TokenKind::ANON
        | TokenKind::STRING_LITERAL2
        | TokenKind::INTEGER_NEGATIVE
        | TokenKind::False
        | TokenKind::PNAME_NS
        | TokenKind::STRING_LITERAL1
        | TokenKind::DOUBLE_POSITIVE
        | TokenKind::INTEGER_POSITIVE => {
            parse_GraphTerm(p);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::VarOrTerm);
}
/// [86] PropertyListNotEmpty -> Verb ObjectList (';' (Verb ObjectList)?)*
pub(super) fn parse_PropertyListNotEmpty(p: &mut Parser) {
    let marker = p.open();
    parse_Verb(p);
    parse_ObjectList(p);
    while [TokenKind::Semicolon].contains(&p.nth(0)) {
        p.expect(TokenKind::Semicolon);
        if p
            .at_any(
                &[
                    TokenKind::VAR2,
                    TokenKind::PNAME_LN,
                    TokenKind::PNAME_NS,
                    TokenKind::a,
                    TokenKind::IRIREF,
                    TokenKind::VAR1,
                ],
            )
        {
            parse_Verb(p);
            parse_ObjectList(p);
        }
    }
    p.close(marker, TreeKind::PropertyListNotEmpty);
}
/// [87] TriplesNode -> Collection | BlankNodePropertyList
pub(super) fn parse_TriplesNode(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::LParen => {
            parse_Collection(p);
        }
        TokenKind::LBrack => {
            parse_BlankNodePropertyList(p);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::TriplesNode);
}
/// [88] PropertyList -> PropertyListNotEmpty?
pub(super) fn parse_PropertyList(p: &mut Parser) {
    let marker = p.open();
    if p
        .at_any(
            &[
                TokenKind::PNAME_LN,
                TokenKind::IRIREF,
                TokenKind::VAR2,
                TokenKind::a,
                TokenKind::PNAME_NS,
                TokenKind::VAR1,
            ],
        )
    {
        parse_PropertyListNotEmpty(p);
    }
    p.close(marker, TreeKind::PropertyList);
}
/// [89] Verb -> VarOrIri | 'a'
pub(super) fn parse_Verb(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::VAR2
        | TokenKind::IRIREF
        | TokenKind::PNAME_NS
        | TokenKind::VAR1
        | TokenKind::PNAME_LN => {
            parse_VarOrIri(p);
        }
        TokenKind::a => {
            p.expect(TokenKind::a);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::Verb);
}
/// [90] ObjectList -> Object (',' Object)*
pub(super) fn parse_ObjectList(p: &mut Parser) {
    let marker = p.open();
    parse_Object(p);
    while [TokenKind::Colon].contains(&p.nth(0)) {
        p.expect(TokenKind::Colon);
        parse_Object(p);
    }
    p.close(marker, TreeKind::ObjectList);
}
/// [91] Object -> GraphNode
pub(super) fn parse_Object(p: &mut Parser) {
    let marker = p.open();
    parse_GraphNode(p);
    p.close(marker, TreeKind::Object);
}
/// [92] GraphNode -> VarOrTerm | TriplesNode
pub(super) fn parse_GraphNode(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::VAR2
        | TokenKind::DOUBLE
        | TokenKind::IRIREF
        | TokenKind::ANON
        | TokenKind::VAR1
        | TokenKind::STRING_LITERAL_LONG2
        | TokenKind::DECIMAL
        | TokenKind::STRING_LITERAL_LONG1
        | TokenKind::PNAME_NS
        | TokenKind::PNAME_LN
        | TokenKind::NIL
        | TokenKind::False
        | TokenKind::INTEGER_POSITIVE
        | TokenKind::DOUBLE_POSITIVE
        | TokenKind::STRING_LITERAL2
        | TokenKind::DECIMAL_POSITIVE
        | TokenKind::DECIMAL_NEGATIVE
        | TokenKind::DOUBLE_NEGATIVE
        | TokenKind::INTEGER
        | TokenKind::BLANK_NODE_LABEL
        | TokenKind::True
        | TokenKind::INTEGER_NEGATIVE
        | TokenKind::STRING_LITERAL1 => {
            parse_VarOrTerm(p);
        }
        TokenKind::LParen | TokenKind::LBrack => {
            parse_TriplesNode(p);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::GraphNode);
}
/// [93] PropertyListPathNotEmpty -> (VerbPath | VerbSimple) ObjectListPath (';' ((VerbPath | VerbSimple) ObjectList)?)*
pub(super) fn parse_PropertyListPathNotEmpty(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::ExclamationMark
        | TokenKind::PNAME_LN
        | TokenKind::IRIREF
        | TokenKind::PNAME_NS
        | TokenKind::LParen
        | TokenKind::a
        | TokenKind::Zirkumflex => {
            parse_VerbPath(p);
        }
        TokenKind::VAR2 | TokenKind::VAR1 => {
            parse_VerbSimple(p);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    parse_ObjectListPath(p);
    while [TokenKind::Semicolon].contains(&p.nth(0)) {
        p.expect(TokenKind::Semicolon);
        if p
            .at_any(
                &[
                    TokenKind::PNAME_NS,
                    TokenKind::Zirkumflex,
                    TokenKind::VAR2,
                    TokenKind::PNAME_LN,
                    TokenKind::ExclamationMark,
                    TokenKind::IRIREF,
                    TokenKind::a,
                    TokenKind::LParen,
                    TokenKind::VAR1,
                ],
            )
        {
            match p.nth(0) {
                TokenKind::ExclamationMark
                | TokenKind::PNAME_LN
                | TokenKind::IRIREF
                | TokenKind::PNAME_NS
                | TokenKind::LParen
                | TokenKind::a
                | TokenKind::Zirkumflex => {
                    parse_VerbPath(p);
                }
                TokenKind::VAR2 | TokenKind::VAR1 => {
                    parse_VerbSimple(p);
                }
                _ => {
                    p.advance_with_error("Expected ....");
                }
            };
            parse_ObjectList(p);
        }
    }
    p.close(marker, TreeKind::PropertyListPathNotEmpty);
}
/// [94] TriplesNodePath -> CollectionPath | BlankNodePropertyListPath
pub(super) fn parse_TriplesNodePath(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::LParen => {
            parse_CollectionPath(p);
        }
        TokenKind::LBrack => {
            parse_BlankNodePropertyListPath(p);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::TriplesNodePath);
}
/// [95] PropertyListPath -> PropertyListPathNotEmpty?
pub(super) fn parse_PropertyListPath(p: &mut Parser) {
    let marker = p.open();
    if p
        .at_any(
            &[
                TokenKind::ExclamationMark,
                TokenKind::VAR2,
                TokenKind::PNAME_NS,
                TokenKind::a,
                TokenKind::PNAME_LN,
                TokenKind::IRIREF,
                TokenKind::LParen,
                TokenKind::VAR1,
                TokenKind::Zirkumflex,
            ],
        )
    {
        parse_PropertyListPathNotEmpty(p);
    }
    p.close(marker, TreeKind::PropertyListPath);
}
/// [96] VerbPath -> Path
pub(super) fn parse_VerbPath(p: &mut Parser) {
    let marker = p.open();
    parse_Path(p);
    p.close(marker, TreeKind::VerbPath);
}
/// [97] VerbSimple -> Var
pub(super) fn parse_VerbSimple(p: &mut Parser) {
    let marker = p.open();
    parse_Var(p);
    p.close(marker, TreeKind::VerbSimple);
}
/// [98] ObjectListPath -> ObjectPath (',' ObjectPath)*
pub(super) fn parse_ObjectListPath(p: &mut Parser) {
    let marker = p.open();
    parse_ObjectPath(p);
    while [TokenKind::Colon].contains(&p.nth(0)) {
        p.expect(TokenKind::Colon);
        parse_ObjectPath(p);
    }
    p.close(marker, TreeKind::ObjectListPath);
}
/// [99] Path -> PathAlternative
pub(super) fn parse_Path(p: &mut Parser) {
    let marker = p.open();
    parse_PathAlternative(p);
    p.close(marker, TreeKind::Path);
}
/// [100] ObjectPath -> GraphNodePath
pub(super) fn parse_ObjectPath(p: &mut Parser) {
    let marker = p.open();
    parse_GraphNodePath(p);
    p.close(marker, TreeKind::ObjectPath);
}
/// [101] GraphNodePath -> VarOrTerm | TriplesNodePath
pub(super) fn parse_GraphNodePath(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::VAR2
        | TokenKind::DOUBLE
        | TokenKind::IRIREF
        | TokenKind::ANON
        | TokenKind::VAR1
        | TokenKind::STRING_LITERAL_LONG2
        | TokenKind::DECIMAL
        | TokenKind::STRING_LITERAL_LONG1
        | TokenKind::PNAME_NS
        | TokenKind::PNAME_LN
        | TokenKind::NIL
        | TokenKind::False
        | TokenKind::INTEGER_POSITIVE
        | TokenKind::DOUBLE_POSITIVE
        | TokenKind::STRING_LITERAL2
        | TokenKind::DECIMAL_POSITIVE
        | TokenKind::DECIMAL_NEGATIVE
        | TokenKind::DOUBLE_NEGATIVE
        | TokenKind::INTEGER
        | TokenKind::BLANK_NODE_LABEL
        | TokenKind::True
        | TokenKind::INTEGER_NEGATIVE
        | TokenKind::STRING_LITERAL1 => {
            parse_VarOrTerm(p);
        }
        TokenKind::LBrack | TokenKind::LParen => {
            parse_TriplesNodePath(p);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::GraphNodePath);
}
/// [102] PathAlternative -> PathSequence ('|' PathSequence)*
pub(super) fn parse_PathAlternative(p: &mut Parser) {
    let marker = p.open();
    parse_PathSequence(p);
    while [TokenKind::Pipe].contains(&p.nth(0)) {
        p.expect(TokenKind::Pipe);
        parse_PathSequence(p);
    }
    p.close(marker, TreeKind::PathAlternative);
}
/// [103] PathSequence -> PathEltOrInverse ('/' PathEltOrInverse)*
pub(super) fn parse_PathSequence(p: &mut Parser) {
    let marker = p.open();
    parse_PathEltOrInverse(p);
    while [TokenKind::Slash].contains(&p.nth(0)) {
        p.expect(TokenKind::Slash);
        parse_PathEltOrInverse(p);
    }
    p.close(marker, TreeKind::PathSequence);
}
/// [104] PathEltOrInverse -> PathElt | '^' PathElt
pub(super) fn parse_PathEltOrInverse(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::LParen
        | TokenKind::ExclamationMark
        | TokenKind::a
        | TokenKind::IRIREF
        | TokenKind::PNAME_LN
        | TokenKind::PNAME_NS => {
            parse_PathElt(p);
        }
        TokenKind::Zirkumflex => {
            p.expect(TokenKind::Zirkumflex);
            parse_PathElt(p);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::PathEltOrInverse);
}
/// [105] PathElt -> PathPrimary PathMod?
pub(super) fn parse_PathElt(p: &mut Parser) {
    let marker = p.open();
    parse_PathPrimary(p);
    if p.at_any(&[TokenKind::Plus, TokenKind::Star, TokenKind::QuestionMark]) {
        parse_PathMod(p);
    }
    p.close(marker, TreeKind::PathElt);
}
/// [106] PathPrimary -> iri | 'a' | '!' PathNegatedPropertySet | '(' Path ')'
pub(super) fn parse_PathPrimary(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::IRIREF | TokenKind::PNAME_LN | TokenKind::PNAME_NS => {
            parse_iri(p);
        }
        TokenKind::a => {
            p.expect(TokenKind::a);
        }
        TokenKind::ExclamationMark => {
            p.expect(TokenKind::ExclamationMark);
            parse_PathNegatedPropertySet(p);
        }
        TokenKind::LParen => {
            p.expect(TokenKind::LParen);
            parse_Path(p);
            p.expect(TokenKind::RParen);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::PathPrimary);
}
/// [107] PathMod -> '?' | '*' | '+'
pub(super) fn parse_PathMod(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::QuestionMark => {
            p.expect(TokenKind::QuestionMark);
        }
        TokenKind::Star => {
            p.expect(TokenKind::Star);
        }
        TokenKind::Plus => {
            p.expect(TokenKind::Plus);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::PathMod);
}
/// [108] PathNegatedPropertySet -> PathOneInPropertySet | '(' (PathOneInPropertySet ('|' PathOneInPropertySet)*)? ')'
pub(super) fn parse_PathNegatedPropertySet(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::IRIREF
        | TokenKind::PNAME_LN
        | TokenKind::PNAME_NS
        | TokenKind::Zirkumflex
        | TokenKind::a => {
            parse_PathOneInPropertySet(p);
        }
        TokenKind::LParen => {
            p.expect(TokenKind::LParen);
            if p
                .at_any(
                    &[
                        TokenKind::PNAME_NS,
                        TokenKind::PNAME_LN,
                        TokenKind::Zirkumflex,
                        TokenKind::IRIREF,
                        TokenKind::a,
                    ],
                )
            {
                parse_PathOneInPropertySet(p);
                while [TokenKind::Pipe].contains(&p.nth(0)) {
                    p.expect(TokenKind::Pipe);
                    parse_PathOneInPropertySet(p);
                }
            }
            p.expect(TokenKind::RParen);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::PathNegatedPropertySet);
}
/// [109] PathOneInPropertySet -> iri | 'a' | '^' (iri | 'a')
pub(super) fn parse_PathOneInPropertySet(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::IRIREF | TokenKind::PNAME_LN | TokenKind::PNAME_NS => {
            parse_iri(p);
        }
        TokenKind::a => {
            p.expect(TokenKind::a);
        }
        TokenKind::Zirkumflex => {
            p.expect(TokenKind::Zirkumflex);
            match p.nth(0) {
                TokenKind::IRIREF | TokenKind::PNAME_LN | TokenKind::PNAME_NS => {
                    parse_iri(p);
                }
                TokenKind::a => {
                    p.expect(TokenKind::a);
                }
                _ => {
                    p.advance_with_error("Expected ....");
                }
            };
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::PathOneInPropertySet);
}
/// [110] Integer -> 'INTEGER'
pub(super) fn parse_Integer(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::INTEGER);
    p.close(marker, TreeKind::Integer);
}
/// [111] Collection -> '(' GraphNode GraphNode* ')'
pub(super) fn parse_Collection(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::LParen);
    parse_GraphNode(p);
    while [
        TokenKind::IRIREF,
        TokenKind::STRING_LITERAL_LONG1,
        TokenKind::PNAME_LN,
        TokenKind::BLANK_NODE_LABEL,
        TokenKind::VAR1,
        TokenKind::DOUBLE,
        TokenKind::NIL,
        TokenKind::False,
        TokenKind::STRING_LITERAL2,
        TokenKind::STRING_LITERAL_LONG2,
        TokenKind::ANON,
        TokenKind::DOUBLE_NEGATIVE,
        TokenKind::DOUBLE_POSITIVE,
        TokenKind::INTEGER_NEGATIVE,
        TokenKind::LBrack,
        TokenKind::STRING_LITERAL1,
        TokenKind::DECIMAL_NEGATIVE,
        TokenKind::True,
        TokenKind::VAR2,
        TokenKind::DECIMAL,
        TokenKind::PNAME_NS,
        TokenKind::LParen,
        TokenKind::DECIMAL_POSITIVE,
        TokenKind::INTEGER,
        TokenKind::INTEGER_POSITIVE,
    ]
        .contains(&p.nth(0))
    {
        parse_GraphNode(p);
    }
    p.expect(TokenKind::RParen);
    p.close(marker, TreeKind::Collection);
}
/// [112] BlankNodePropertyList -> '[' PropertyListNotEmpty ']'
pub(super) fn parse_BlankNodePropertyList(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::LBrack);
    parse_PropertyListNotEmpty(p);
    p.expect(TokenKind::RBrack);
    p.close(marker, TreeKind::BlankNodePropertyList);
}
/// [113] CollectionPath -> '(' GraphNodePath GraphNodePath* ')'
pub(super) fn parse_CollectionPath(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::LParen);
    parse_GraphNodePath(p);
    while [
        TokenKind::True,
        TokenKind::INTEGER_NEGATIVE,
        TokenKind::DECIMAL_NEGATIVE,
        TokenKind::LBrack,
        TokenKind::IRIREF,
        TokenKind::LParen,
        TokenKind::STRING_LITERAL_LONG2,
        TokenKind::DECIMAL_POSITIVE,
        TokenKind::STRING_LITERAL2,
        TokenKind::STRING_LITERAL1,
        TokenKind::NIL,
        TokenKind::VAR2,
        TokenKind::ANON,
        TokenKind::STRING_LITERAL_LONG1,
        TokenKind::INTEGER,
        TokenKind::DECIMAL,
        TokenKind::PNAME_NS,
        TokenKind::DOUBLE,
        TokenKind::VAR1,
        TokenKind::INTEGER_POSITIVE,
        TokenKind::PNAME_LN,
        TokenKind::DOUBLE_POSITIVE,
        TokenKind::BLANK_NODE_LABEL,
        TokenKind::DOUBLE_NEGATIVE,
        TokenKind::False,
    ]
        .contains(&p.nth(0))
    {
        parse_GraphNodePath(p);
    }
    p.expect(TokenKind::RParen);
    p.close(marker, TreeKind::CollectionPath);
}
/// [114] BlankNodePropertyListPath -> '[' PropertyListPathNotEmpty ']'
pub(super) fn parse_BlankNodePropertyListPath(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::LBrack);
    parse_PropertyListPathNotEmpty(p);
    p.expect(TokenKind::RBrack);
    p.close(marker, TreeKind::BlankNodePropertyListPath);
}
/// [115] GraphTerm -> iri | RDFLiteral | NumericLiteral | BooleanLiteral | BlankNode | 'NIL'
pub(super) fn parse_GraphTerm(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::IRIREF | TokenKind::PNAME_LN | TokenKind::PNAME_NS => {
            parse_iri(p);
        }
        TokenKind::STRING_LITERAL_LONG1
        | TokenKind::STRING_LITERAL1
        | TokenKind::STRING_LITERAL2
        | TokenKind::STRING_LITERAL_LONG2 => {
            parse_RDFLiteral(p);
        }
        TokenKind::DOUBLE_POSITIVE
        | TokenKind::DECIMAL_NEGATIVE
        | TokenKind::DOUBLE_NEGATIVE
        | TokenKind::DECIMAL
        | TokenKind::INTEGER
        | TokenKind::DECIMAL_POSITIVE
        | TokenKind::INTEGER_POSITIVE
        | TokenKind::INTEGER_NEGATIVE
        | TokenKind::DOUBLE => {
            parse_NumericLiteral(p);
        }
        TokenKind::True | TokenKind::False => {
            parse_BooleanLiteral(p);
        }
        TokenKind::BLANK_NODE_LABEL | TokenKind::ANON => {
            parse_BlankNode(p);
        }
        TokenKind::NIL => {
            p.expect(TokenKind::NIL);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::GraphTerm);
}
/// [116] BlankNode -> 'BLANK_NODE_LABEL' | 'ANON'
pub(super) fn parse_BlankNode(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::BLANK_NODE_LABEL => {
            p.expect(TokenKind::BLANK_NODE_LABEL);
        }
        TokenKind::ANON => {
            p.expect(TokenKind::ANON);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::BlankNode);
}
/// [117] ConditionalOrExpression -> ConditionalAndExpression ('||' ConditionalAndExpression)*
pub(super) fn parse_ConditionalOrExpression(p: &mut Parser) {
    let marker = p.open();
    parse_ConditionalAndExpression(p);
    while [TokenKind::DoublePipe].contains(&p.nth(0)) {
        p.expect(TokenKind::DoublePipe);
        parse_ConditionalAndExpression(p);
    }
    p.close(marker, TreeKind::ConditionalOrExpression);
}
/// [118] ConditionalAndExpression -> ValueLogical ('&&' ValueLogical)*
pub(super) fn parse_ConditionalAndExpression(p: &mut Parser) {
    let marker = p.open();
    parse_ValueLogical(p);
    while [TokenKind::DoubleAnd].contains(&p.nth(0)) {
        p.expect(TokenKind::DoubleAnd);
        parse_ValueLogical(p);
    }
    p.close(marker, TreeKind::ConditionalAndExpression);
}
/// [119] ValueLogical -> RelationalExpression
pub(super) fn parse_ValueLogical(p: &mut Parser) {
    let marker = p.open();
    parse_RelationalExpression(p);
    p.close(marker, TreeKind::ValueLogical);
}
/// [120] RelationalExpression -> NumericExpression ('=' NumericExpression | '!=' NumericExpression | '<' NumericExpression | '>' NumericExpression | '<=' NumericExpression | '>=' NumericExpression | 'IN' ExpressionList | 'NOT' 'IN' ExpressionList)?
pub(super) fn parse_RelationalExpression(p: &mut Parser) {
    let marker = p.open();
    parse_NumericExpression(p);
    if p
        .at_any(
            &[
                TokenKind::MoreEquals,
                TokenKind::IN,
                TokenKind::More,
                TokenKind::LessEquals,
                TokenKind::Less,
                TokenKind::NOT,
                TokenKind::Equals,
                TokenKind::ExclamationMarkEquals,
            ],
        )
    {
        match p.nth(0) {
            TokenKind::Equals => {
                p.expect(TokenKind::Equals);
                parse_NumericExpression(p);
            }
            TokenKind::ExclamationMarkEquals => {
                p.expect(TokenKind::ExclamationMarkEquals);
                parse_NumericExpression(p);
            }
            TokenKind::Less => {
                p.expect(TokenKind::Less);
                parse_NumericExpression(p);
            }
            TokenKind::More => {
                p.expect(TokenKind::More);
                parse_NumericExpression(p);
            }
            TokenKind::LessEquals => {
                p.expect(TokenKind::LessEquals);
                parse_NumericExpression(p);
            }
            TokenKind::MoreEquals => {
                p.expect(TokenKind::MoreEquals);
                parse_NumericExpression(p);
            }
            TokenKind::IN => {
                p.expect(TokenKind::IN);
                parse_ExpressionList(p);
            }
            TokenKind::NOT => {
                p.expect(TokenKind::NOT);
                p.expect(TokenKind::IN);
                parse_ExpressionList(p);
            }
            _ => {
                p.advance_with_error("Expected ....");
            }
        };
    }
    p.close(marker, TreeKind::RelationalExpression);
}
/// [121] NumericExpression -> AdditiveExpression
pub(super) fn parse_NumericExpression(p: &mut Parser) {
    let marker = p.open();
    parse_AdditiveExpression(p);
    p.close(marker, TreeKind::NumericExpression);
}
/// [122] AdditiveExpression -> MultiplicativeExpression ('+' MultiplicativeExpression | '-' MultiplicativeExpression | (NumericLiteralPositive | NumericLiteralNegative) ('*' UnaryExpression | '/' UnaryExpression)*)*
pub(super) fn parse_AdditiveExpression(p: &mut Parser) {
    let marker = p.open();
    parse_MultiplicativeExpression(p);
    while [
        TokenKind::DOUBLE_NEGATIVE,
        TokenKind::Minus,
        TokenKind::INTEGER_NEGATIVE,
        TokenKind::DECIMAL_NEGATIVE,
        TokenKind::DECIMAL_POSITIVE,
        TokenKind::Plus,
        TokenKind::DOUBLE_POSITIVE,
        TokenKind::INTEGER_POSITIVE,
    ]
        .contains(&p.nth(0))
    {
        match p.nth(0) {
            TokenKind::Plus => {
                p.expect(TokenKind::Plus);
                parse_MultiplicativeExpression(p);
            }
            TokenKind::Minus => {
                p.expect(TokenKind::Minus);
                parse_MultiplicativeExpression(p);
            }
            TokenKind::DECIMAL_POSITIVE
            | TokenKind::INTEGER_POSITIVE
            | TokenKind::DECIMAL_NEGATIVE
            | TokenKind::DOUBLE_NEGATIVE
            | TokenKind::DOUBLE_POSITIVE
            | TokenKind::INTEGER_NEGATIVE => {
                match p.nth(0) {
                    TokenKind::DECIMAL_POSITIVE
                    | TokenKind::DOUBLE_POSITIVE
                    | TokenKind::INTEGER_POSITIVE => {
                        parse_NumericLiteralPositive(p);
                    }
                    TokenKind::INTEGER_NEGATIVE
                    | TokenKind::DECIMAL_NEGATIVE
                    | TokenKind::DOUBLE_NEGATIVE => {
                        parse_NumericLiteralNegative(p);
                    }
                    _ => {
                        p.advance_with_error("Expected ....");
                    }
                };
                while [TokenKind::Slash, TokenKind::Star].contains(&p.nth(0)) {
                    match p.nth(0) {
                        TokenKind::Star => {
                            p.expect(TokenKind::Star);
                            parse_UnaryExpression(p);
                        }
                        TokenKind::Slash => {
                            p.expect(TokenKind::Slash);
                            parse_UnaryExpression(p);
                        }
                        _ => {
                            p.advance_with_error("Expected ....");
                        }
                    };
                }
            }
            _ => {
                p.advance_with_error("Expected ....");
            }
        };
    }
    p.close(marker, TreeKind::AdditiveExpression);
}
/// [123] MultiplicativeExpression -> UnaryExpression ('*' UnaryExpression | '/' UnaryExpression)*
pub(super) fn parse_MultiplicativeExpression(p: &mut Parser) {
    let marker = p.open();
    parse_UnaryExpression(p);
    while [TokenKind::Slash, TokenKind::Star].contains(&p.nth(0)) {
        match p.nth(0) {
            TokenKind::Star => {
                p.expect(TokenKind::Star);
                parse_UnaryExpression(p);
            }
            TokenKind::Slash => {
                p.expect(TokenKind::Slash);
                parse_UnaryExpression(p);
            }
            _ => {
                p.advance_with_error("Expected ....");
            }
        };
    }
    p.close(marker, TreeKind::MultiplicativeExpression);
}
/// [124] NumericLiteralPositive -> 'INTEGER_POSITIVE' | 'DECIMAL_POSITIVE' | 'DOUBLE_POSITIVE'
pub(super) fn parse_NumericLiteralPositive(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::INTEGER_POSITIVE => {
            p.expect(TokenKind::INTEGER_POSITIVE);
        }
        TokenKind::DECIMAL_POSITIVE => {
            p.expect(TokenKind::DECIMAL_POSITIVE);
        }
        TokenKind::DOUBLE_POSITIVE => {
            p.expect(TokenKind::DOUBLE_POSITIVE);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::NumericLiteralPositive);
}
/// [125] NumericLiteralNegative -> 'INTEGER_NEGATIVE' | 'DECIMAL_NEGATIVE' | 'DOUBLE_NEGATIVE'
pub(super) fn parse_NumericLiteralNegative(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::INTEGER_NEGATIVE => {
            p.expect(TokenKind::INTEGER_NEGATIVE);
        }
        TokenKind::DECIMAL_NEGATIVE => {
            p.expect(TokenKind::DECIMAL_NEGATIVE);
        }
        TokenKind::DOUBLE_NEGATIVE => {
            p.expect(TokenKind::DOUBLE_NEGATIVE);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::NumericLiteralNegative);
}
/// [126] UnaryExpression -> '!' PrimaryExpression | '+' PrimaryExpression | '-' PrimaryExpression | PrimaryExpression
pub(super) fn parse_UnaryExpression(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::ExclamationMark => {
            p.expect(TokenKind::ExclamationMark);
            parse_PrimaryExpression(p);
        }
        TokenKind::Plus => {
            p.expect(TokenKind::Plus);
            parse_PrimaryExpression(p);
        }
        TokenKind::Minus => {
            p.expect(TokenKind::Minus);
            parse_PrimaryExpression(p);
        }
        TokenKind::INTEGER_POSITIVE
        | TokenKind::MINUTES
        | TokenKind::DAY
        | TokenKind::MAX
        | TokenKind::RAND
        | TokenKind::MD5
        | TokenKind::CONTAINS
        | TokenKind::MONTH
        | TokenKind::DOUBLE_POSITIVE
        | TokenKind::isIRI
        | TokenKind::CONCAT
        | TokenKind::sameTerm
        | TokenKind::STRING_LITERAL_LONG2
        | TokenKind::True
        | TokenKind::STRDT
        | TokenKind::CEIL
        | TokenKind::GROUP_CONCAT
        | TokenKind::DOUBLE
        | TokenKind::MIN
        | TokenKind::UUID
        | TokenKind::HOURS
        | TokenKind::REGEX
        | TokenKind::SAMPLE
        | TokenKind::EXISTS
        | TokenKind::STRAFTER
        | TokenKind::STRSTARTS
        | TokenKind::UCASE
        | TokenKind::SECONDS
        | TokenKind::AVG
        | TokenKind::STRBEFORE
        | TokenKind::isURI
        | TokenKind::STRLEN
        | TokenKind::INTEGER
        | TokenKind::ABS
        | TokenKind::SHA256
        | TokenKind::STRING_LITERAL_LONG1
        | TokenKind::DECIMAL
        | TokenKind::VAR1
        | TokenKind::TZ
        | TokenKind::LANGMATCHES
        | TokenKind::DOUBLE_NEGATIVE
        | TokenKind::FLOOR
        | TokenKind::False
        | TokenKind::URI
        | TokenKind::SHA384
        | TokenKind::LParen
        | TokenKind::DATATYPE
        | TokenKind::STRING_LITERAL2
        | TokenKind::COUNT
        | TokenKind::REPLACE
        | TokenKind::INTEGER_NEGATIVE
        | TokenKind::VAR2
        | TokenKind::STRLANG
        | TokenKind::STR
        | TokenKind::SUM
        | TokenKind::SUBSTR
        | TokenKind::BNODE
        | TokenKind::TIMEZONE
        | TokenKind::STRENDS
        | TokenKind::SHA1
        | TokenKind::ENCODE_FOR_URI
        | TokenKind::LCASE
        | TokenKind::NOT
        | TokenKind::PNAME_LN
        | TokenKind::STRING_LITERAL1
        | TokenKind::IRI
        | TokenKind::COALESCE
        | TokenKind::ROUND
        | TokenKind::LANG
        | TokenKind::isBLANK
        | TokenKind::PNAME_NS
        | TokenKind::SHA512
        | TokenKind::IRIREF
        | TokenKind::NOW
        | TokenKind::IF
        | TokenKind::DECIMAL_NEGATIVE
        | TokenKind::STRUUID
        | TokenKind::DECIMAL_POSITIVE
        | TokenKind::YEAR
        | TokenKind::BOUND
        | TokenKind::isLITERAL
        | TokenKind::isNUMERIC => {
            parse_PrimaryExpression(p);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::UnaryExpression);
}
/// [127] PrimaryExpression -> BrackettedExpression | BuiltInCall | iriOrFunction | RDFLiteral | NumericLiteral | BooleanLiteral | Var
pub(super) fn parse_PrimaryExpression(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::LParen => {
            parse_BrackettedExpression(p);
        }
        TokenKind::MINUTES
        | TokenKind::SECONDS
        | TokenKind::TIMEZONE
        | TokenKind::MD5
        | TokenKind::STRLANG
        | TokenKind::CONTAINS
        | TokenKind::isIRI
        | TokenKind::AVG
        | TokenKind::REGEX
        | TokenKind::isNUMERIC
        | TokenKind::BNODE
        | TokenKind::TZ
        | TokenKind::REPLACE
        | TokenKind::STRLEN
        | TokenKind::LANG
        | TokenKind::MAX
        | TokenKind::SUBSTR
        | TokenKind::RAND
        | TokenKind::IF
        | TokenKind::IRI
        | TokenKind::UCASE
        | TokenKind::DATATYPE
        | TokenKind::UUID
        | TokenKind::BOUND
        | TokenKind::CONCAT
        | TokenKind::STRBEFORE
        | TokenKind::SAMPLE
        | TokenKind::MIN
        | TokenKind::EXISTS
        | TokenKind::STRAFTER
        | TokenKind::MONTH
        | TokenKind::DAY
        | TokenKind::SHA1
        | TokenKind::STRENDS
        | TokenKind::ABS
        | TokenKind::SHA384
        | TokenKind::STRSTARTS
        | TokenKind::URI
        | TokenKind::isURI
        | TokenKind::sameTerm
        | TokenKind::ROUND
        | TokenKind::YEAR
        | TokenKind::SHA512
        | TokenKind::ENCODE_FOR_URI
        | TokenKind::NOW
        | TokenKind::LANGMATCHES
        | TokenKind::isBLANK
        | TokenKind::LCASE
        | TokenKind::SUM
        | TokenKind::SHA256
        | TokenKind::STRDT
        | TokenKind::COUNT
        | TokenKind::FLOOR
        | TokenKind::isLITERAL
        | TokenKind::STR
        | TokenKind::CEIL
        | TokenKind::NOT
        | TokenKind::GROUP_CONCAT
        | TokenKind::COALESCE
        | TokenKind::HOURS
        | TokenKind::STRUUID => {
            parse_BuiltInCall(p);
        }
        TokenKind::PNAME_LN | TokenKind::PNAME_NS | TokenKind::IRIREF => {
            parse_iriOrFunction(p);
        }
        TokenKind::STRING_LITERAL_LONG1
        | TokenKind::STRING_LITERAL1
        | TokenKind::STRING_LITERAL2
        | TokenKind::STRING_LITERAL_LONG2 => {
            parse_RDFLiteral(p);
        }
        TokenKind::DOUBLE_POSITIVE
        | TokenKind::DECIMAL_NEGATIVE
        | TokenKind::DOUBLE_NEGATIVE
        | TokenKind::DECIMAL
        | TokenKind::INTEGER
        | TokenKind::DECIMAL_POSITIVE
        | TokenKind::INTEGER_POSITIVE
        | TokenKind::INTEGER_NEGATIVE
        | TokenKind::DOUBLE => {
            parse_NumericLiteral(p);
        }
        TokenKind::True | TokenKind::False => {
            parse_BooleanLiteral(p);
        }
        TokenKind::VAR2 | TokenKind::VAR1 => {
            parse_Var(p);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::PrimaryExpression);
}
/// [128] iriOrFunction -> iri ArgList?
pub(super) fn parse_iriOrFunction(p: &mut Parser) {
    let marker = p.open();
    parse_iri(p);
    if p.at_any(&[TokenKind::LParen, TokenKind::NIL]) {
        parse_ArgList(p);
    }
    p.close(marker, TreeKind::iriOrFunction);
}
/// [129] Aggregate -> 'COUNT' '(' 'DISTINCT'? ('*' | Expression) ')' | 'SUM' '(' 'DISTINCT'? Expression ')' | 'MIN' '(' 'DISTINCT'? Expression ')' | 'MAX' '(' 'DISTINCT'? Expression ')' | 'AVG' '(' 'DISTINCT'? Expression ')' | 'SAMPLE' '(' 'DISTINCT'? Expression ')' | 'GROUP_CONCAT' '(' 'DISTINCT'? Expression (';' 'SEPARATOR' '=' String)? ')'
pub(super) fn parse_Aggregate(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::COUNT => {
            p.expect(TokenKind::COUNT);
            p.expect(TokenKind::LParen);
            if p.at_any(&[TokenKind::DISTINCT]) {
                p.expect(TokenKind::DISTINCT);
            }
            match p.nth(0) {
                TokenKind::Star => {
                    p.expect(TokenKind::Star);
                }
                TokenKind::IF
                | TokenKind::IRI
                | TokenKind::STRING_LITERAL_LONG2
                | TokenKind::SECONDS
                | TokenKind::STRDT
                | TokenKind::isLITERAL
                | TokenKind::SUBSTR
                | TokenKind::INTEGER_POSITIVE
                | TokenKind::LParen
                | TokenKind::ENCODE_FOR_URI
                | TokenKind::CONCAT
                | TokenKind::Plus
                | TokenKind::UUID
                | TokenKind::PNAME_LN
                | TokenKind::SHA512
                | TokenKind::SAMPLE
                | TokenKind::STRBEFORE
                | TokenKind::DAY
                | TokenKind::EXISTS
                | TokenKind::TZ
                | TokenKind::TIMEZONE
                | TokenKind::DECIMAL_NEGATIVE
                | TokenKind::STRLANG
                | TokenKind::SHA384
                | TokenKind::STRING_LITERAL1
                | TokenKind::MONTH
                | TokenKind::NOW
                | TokenKind::VAR1
                | TokenKind::IRIREF
                | TokenKind::MINUTES
                | TokenKind::REGEX
                | TokenKind::HOURS
                | TokenKind::DOUBLE
                | TokenKind::SUM
                | TokenKind::False
                | TokenKind::STRAFTER
                | TokenKind::CONTAINS
                | TokenKind::SHA256
                | TokenKind::DECIMAL_POSITIVE
                | TokenKind::STRLEN
                | TokenKind::isBLANK
                | TokenKind::COUNT
                | TokenKind::DATATYPE
                | TokenKind::REPLACE
                | TokenKind::ExclamationMark
                | TokenKind::URI
                | TokenKind::BOUND
                | TokenKind::MIN
                | TokenKind::STR
                | TokenKind::True
                | TokenKind::STRING_LITERAL_LONG1
                | TokenKind::STRENDS
                | TokenKind::Minus
                | TokenKind::DOUBLE_NEGATIVE
                | TokenKind::STRUUID
                | TokenKind::isIRI
                | TokenKind::LANG
                | TokenKind::INTEGER
                | TokenKind::NOT
                | TokenKind::STRING_LITERAL2
                | TokenKind::GROUP_CONCAT
                | TokenKind::SHA1
                | TokenKind::AVG
                | TokenKind::CEIL
                | TokenKind::BNODE
                | TokenKind::sameTerm
                | TokenKind::isNUMERIC
                | TokenKind::ROUND
                | TokenKind::DECIMAL
                | TokenKind::LCASE
                | TokenKind::UCASE
                | TokenKind::COALESCE
                | TokenKind::LANGMATCHES
                | TokenKind::YEAR
                | TokenKind::DOUBLE_POSITIVE
                | TokenKind::STRSTARTS
                | TokenKind::MD5
                | TokenKind::FLOOR
                | TokenKind::ABS
                | TokenKind::VAR2
                | TokenKind::isURI
                | TokenKind::PNAME_NS
                | TokenKind::MAX
                | TokenKind::INTEGER_NEGATIVE
                | TokenKind::RAND => {
                    parse_Expression(p);
                }
                _ => {
                    p.advance_with_error("Expected ....");
                }
            };
            p.expect(TokenKind::RParen);
        }
        TokenKind::SUM => {
            p.expect(TokenKind::SUM);
            p.expect(TokenKind::LParen);
            if p.at_any(&[TokenKind::DISTINCT]) {
                p.expect(TokenKind::DISTINCT);
            }
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::MIN => {
            p.expect(TokenKind::MIN);
            p.expect(TokenKind::LParen);
            if p.at_any(&[TokenKind::DISTINCT]) {
                p.expect(TokenKind::DISTINCT);
            }
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::MAX => {
            p.expect(TokenKind::MAX);
            p.expect(TokenKind::LParen);
            if p.at_any(&[TokenKind::DISTINCT]) {
                p.expect(TokenKind::DISTINCT);
            }
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::AVG => {
            p.expect(TokenKind::AVG);
            p.expect(TokenKind::LParen);
            if p.at_any(&[TokenKind::DISTINCT]) {
                p.expect(TokenKind::DISTINCT);
            }
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::SAMPLE => {
            p.expect(TokenKind::SAMPLE);
            p.expect(TokenKind::LParen);
            if p.at_any(&[TokenKind::DISTINCT]) {
                p.expect(TokenKind::DISTINCT);
            }
            parse_Expression(p);
            p.expect(TokenKind::RParen);
        }
        TokenKind::GROUP_CONCAT => {
            p.expect(TokenKind::GROUP_CONCAT);
            p.expect(TokenKind::LParen);
            if p.at_any(&[TokenKind::DISTINCT]) {
                p.expect(TokenKind::DISTINCT);
            }
            parse_Expression(p);
            if p.at_any(&[TokenKind::Semicolon]) {
                p.expect(TokenKind::Semicolon);
                p.expect(TokenKind::SEPARATOR);
                p.expect(TokenKind::Equals);
                parse_String(p);
            }
            p.expect(TokenKind::RParen);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::Aggregate);
}
/// [130] SubstringExpression -> 'SUBSTR' '(' Expression ',' Expression (',' Expression)? ')'
pub(super) fn parse_SubstringExpression(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::SUBSTR);
    p.expect(TokenKind::LParen);
    parse_Expression(p);
    p.expect(TokenKind::Colon);
    parse_Expression(p);
    if p.at_any(&[TokenKind::Colon]) {
        p.expect(TokenKind::Colon);
        parse_Expression(p);
    }
    p.expect(TokenKind::RParen);
    p.close(marker, TreeKind::SubstringExpression);
}
/// [131] StrReplaceExpression -> 'REPLACE' '(' Expression ',' Expression ',' Expression (',' Expression)? ')'
pub(super) fn parse_StrReplaceExpression(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::REPLACE);
    p.expect(TokenKind::LParen);
    parse_Expression(p);
    p.expect(TokenKind::Colon);
    parse_Expression(p);
    p.expect(TokenKind::Colon);
    parse_Expression(p);
    if p.at_any(&[TokenKind::Colon]) {
        p.expect(TokenKind::Colon);
        parse_Expression(p);
    }
    p.expect(TokenKind::RParen);
    p.close(marker, TreeKind::StrReplaceExpression);
}
/// [132] RegexExpression -> 'REGEX' '(' Expression ',' Expression (',' Expression)? ')'
pub(super) fn parse_RegexExpression(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::REGEX);
    p.expect(TokenKind::LParen);
    parse_Expression(p);
    p.expect(TokenKind::Colon);
    parse_Expression(p);
    if p.at_any(&[TokenKind::Colon]) {
        p.expect(TokenKind::Colon);
        parse_Expression(p);
    }
    p.expect(TokenKind::RParen);
    p.close(marker, TreeKind::RegexExpression);
}
/// [133] ExistsFunc -> 'EXISTS' GroupGraphPattern
pub(super) fn parse_ExistsFunc(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::EXISTS);
    parse_GroupGraphPattern(p);
    p.close(marker, TreeKind::ExistsFunc);
}
/// [134] NotExistsFunc -> 'NOT' 'EXISTS' GroupGraphPattern
pub(super) fn parse_NotExistsFunc(p: &mut Parser) {
    let marker = p.open();
    p.expect(TokenKind::NOT);
    p.expect(TokenKind::EXISTS);
    parse_GroupGraphPattern(p);
    p.close(marker, TreeKind::NotExistsFunc);
}
/// [135] String -> 'STRING_LITERAL1' | 'STRING_LITERAL2' | 'STRING_LITERAL_LONG1' | 'STRING_LITERAL_LONG2'
pub(super) fn parse_String(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::STRING_LITERAL1 => {
            p.expect(TokenKind::STRING_LITERAL1);
        }
        TokenKind::STRING_LITERAL2 => {
            p.expect(TokenKind::STRING_LITERAL2);
        }
        TokenKind::STRING_LITERAL_LONG1 => {
            p.expect(TokenKind::STRING_LITERAL_LONG1);
        }
        TokenKind::STRING_LITERAL_LONG2 => {
            p.expect(TokenKind::STRING_LITERAL_LONG2);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::String);
}
/// [136] NumericLiteralUnsigned -> 'INTEGER' | 'DECIMAL' | 'DOUBLE'
pub(super) fn parse_NumericLiteralUnsigned(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::INTEGER => {
            p.expect(TokenKind::INTEGER);
        }
        TokenKind::DECIMAL => {
            p.expect(TokenKind::DECIMAL);
        }
        TokenKind::DOUBLE => {
            p.expect(TokenKind::DOUBLE);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::NumericLiteralUnsigned);
}
/// [137] PrefixedName -> 'PNAME_LN' | 'PNAME_NS'
pub(super) fn parse_PrefixedName(p: &mut Parser) {
    let marker = p.open();
    match p.nth(0) {
        TokenKind::PNAME_LN => {
            p.expect(TokenKind::PNAME_LN);
        }
        TokenKind::PNAME_NS => {
            p.expect(TokenKind::PNAME_NS);
        }
        _ => {
            p.advance_with_error("Expected ....");
        }
    };
    p.close(marker, TreeKind::PrefixedName);
}
