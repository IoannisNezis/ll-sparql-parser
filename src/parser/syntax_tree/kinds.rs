use logos::Logos;
use serde::Serialize;
use wasm_bindgen::prelude::wasm_bindgen;

#[allow(non_camel_case_types)]
#[derive(Logos, Debug, PartialEq, Clone, Copy, Serialize)]
#[logos(skip r"[ \t\n\f]+")]
pub enum TokenKind {
    Eof,
    ErrorToken,
    #[token("BASE", ignore(case))]
    BASE,
    #[regex(r#"<[^<>\"{}|^`\\\u{00}-\u{20}]*>"#)]
    IRIREF,
    #[token("PREFIX", ignore(case))]
    PREFIX,
    #[regex("[A-Za-z\u{00C0}-\u{00D6}\u{00D8}-\u{00F6}\u{00F8}-\u{02FF}\u{0370}-\u{037D}\u{037F}-\u{1FFF}\u{200C}-\u{200D}\u{2070}-\u{218F}\u{2C00}-\u{2FEF}\u{3001}-\u{D7FF}\u{F900}-\u{FDCF}\u{FDF0}-\u{FFFD}\u{10000}-\u{EFFFF}]([A-Za-z\u{00C0}-\u{00D6}\u{00D8}-\u{00F6}\u{00F8}-\u{02FF}\u{0370}-\u{037D}\u{037F}-\u{1FFF}\u{200C}-\u{200D}\u{2070}-\u{218F}\u{2C00}-\u{2FEF}\u{3001}-\u{D7FF}\u{F900}-\u{FDCF}\u{FDF0}-\u{FFFD}\u{10000}-\u{EFFFF}0-9\u{00B7}\u{0300}-\u{036F}\u{203F}-\u{2040}_.-]*[A-Za-z\u{00C0}-\u{00D6}\u{00D8}-\u{00F6}\u{00F8}-\u{02FF}\u{0370}-\u{037D}\u{037F}-\u{1FFF}\u{200C}-\u{200D}\u{2070}-\u{218F}\u{2C00}-\u{2FEF}\u{3001}-\u{D7FF}\u{F900}-\u{FDCF}\u{FDF0}-\u{FFFD}\u{10000}-\u{EFFFF}0-9\u{00B7}\u{0300}-\u{036F}\u{203F}-\u{2040}_-])?:")]
    PNAME_NS,
    #[token("SELECT", ignore(case))]
    SELECT,
    #[token("DISTINCT", ignore(case))]
    DISTINCT,
    #[token("REDUCED", ignore(case))]
    REDUCED,
    #[token("(")]
    LParen,
    #[token("AS")]
    AS,
    #[token(")")]
    RParen,
    #[token("*")]
    Star,
    #[token("CONSTRUCT", ignore(case))]
    CONSTRUCT,
    #[token("WHERE", ignore(case))]
    WHERE,
    #[token("{")]
    LCurly,
    #[token("}")]
    RCurly,
    #[token("DESCRIBE", ignore(case))]
    DESCRIBE,
    #[token("ASK", ignore(case))]
    ASK,
    #[token("FROM", ignore(case))]
    FROM,
    #[token("NAMED", ignore(case))]
    NAMED,
    #[token("GROUP", ignore(case))]
    GROUP,
    #[token("BY", ignore(case))]
    BY,
    #[token("HAVING", ignore(case))]
    HAVING,
    #[token("ORDER", ignore(case))]
    ORDER,
    #[token("ASC", ignore(case))]
    ASC,
    #[token("DESC", ignore(case))]
    DESC,
    #[token("LIMIT", ignore(case))]
    LIMIT,
    #[token("INTEGER", ignore(case))]
    INTEGER,
    #[token("OFFSET", ignore(case))]
    OFFSET,
    #[token("VALUES", ignore(case))]
    VALUES,
    #[token("Semicolon", ignore(case))]
    Semicolon,
    #[token("LOAD", ignore(case))]
    LOAD,
    #[token("SILENT", ignore(case))]
    SILENT,
    #[token("INTO", ignore(case))]
    INTO,
    #[token("CLEAR", ignore(case))]
    CLEAR,
    #[token("DROP", ignore(case))]
    DROP,
    #[token("CREATE", ignore(case))]
    CREATE,
    #[token("ADD", ignore(case))]
    ADD,
    #[token("TO", ignore(case))]
    TO,
    #[token("MOVE", ignore(case))]
    MOVE,
    #[token("COPY", ignore(case))]
    COPY,
    #[token("INSERT", ignore(case))]
    INSERT,
    #[token("DATA", ignore(case))]
    DATA,
    INSERT_DATA,
    #[regex(r"(?i)DELETE\s+DATA")]
    DELETE_DATA,
    #[token(r"(?i)DELETE\s+WHERE")]
    DELETE_WHERE,
    #[token("DELETE", ignore(case))]
    DELETE,
    #[token("WITH", ignore(case))]
    WITH,
    #[token("USING", ignore(case))]
    USING,
    #[token("DEFAULT", ignore(case))]
    DEFAULT,
    #[token("GRAPH", ignore(case))]
    GRAPH,
    #[token("ALL", ignore(case))]
    ALL,
    #[token("Dot", ignore(case))]
    Dot,
    #[token("OPTIONAL", ignore(case))]
    OPTIONAL,
    #[token("SERVICE", ignore(case))]
    SERVICE,
    #[token("BIND", ignore(case))]
    BIND,
    #[token("NIL", ignore(case))]
    NIL,
    #[token("UNDEF", ignore(case))]
    UNDEF,
    #[token("MINUS", ignore(case))]
    MINUS,
    #[token("UNION", ignore(case))]
    UNION,
    #[token("FILTER", ignore(case))]
    FILTER,
    #[token(":")]
    Colon,
    #[token("a")]
    a,
    #[token("|")]
    Pipe,
    #[token("/")]
    Slash,
    #[token("^")]
    Zirkumflex,
    #[token("?")]
    QuestionMark,
    #[token("+")]
    Plus,
    #[token("!")]
    ExclamationMark,
    #[token("[")]
    LBrack,
    #[token("]")]
    RBrack,
    #[regex("\\?[A-Za-z]+")]
    VAR1,
    #[regex("\\$[A-Za-z]+")]
    VAR2,
    #[token("||")]
    DoublePipe,
    #[token("&&")]
    DoubleAnd,
    #[token("=")]
    Equals,
    #[token("!=")]
    ExclamationMarkEquals,
    #[token("<")]
    Less,
    #[token(">")]
    More,
    #[token("<=")]
    LessEquals,
    #[token(">=")]
    MoreEquals,
    #[token("IN")]
    IN,
    #[token("NOT")]
    NOT,
    #[token("-")]
    Minus,
    #[token("STR")]
    STR,
    #[token("LANG")]
    LANG,
    #[token("LANGMATCHES")]
    LANGMATCHES,
    #[token("DATATYPE")]
    DATATYPE,
    #[token("BOUND")]
    BOUND,
    #[token("IRI")]
    IRI,
    #[token("URI")]
    URI,
    #[token("BNODE")]
    BNODE,
    #[token("RAND")]
    RAND,
    #[token("ABS")]
    ABS,
    #[token("CEIL")]
    CEIL,
    #[token("FLOOR")]
    FLOOR,
    #[token("ROUND")]
    ROUND,
    #[token("CONCAT")]
    CONCAT,
    #[token("STRLEN")]
    STRLEN,
    #[token("UCASE")]
    UCASE,
    #[token("LCASE")]
    LCASE,
    #[token("ENCODE_FOR_URI")]
    ENCODE_FOR_URI,
    #[token("CONTAINS")]
    CONTAINS,
    #[token("STRSTARTS")]
    STRSTARTS,
    #[token("STRENDS")]
    STRENDS,
    #[token("STRBEFORE")]
    STRBEFORE,
    #[token("STRAFTER")]
    STRAFTER,
    #[token("YEAR")]
    YEAR,
    #[token("MONTH")]
    MONTH,
    #[token("DAY")]
    DAY,
    #[token("HOURS")]
    HOURS,
    #[token("MINUTES")]
    MINUTES,
    #[token("SECONDS")]
    SECONDS,
    #[token("TIMEZONE")]
    TIMEZONE,
    #[token("TZ")]
    TZ,
    #[token("NOW")]
    NOW,
    #[token("UUID")]
    UUID,
    #[token("STRUUID")]
    STRUUID,
    #[token("MD5")]
    MD5,
    #[token("SHA1")]
    SHA1,
    #[token("SHA256")]
    SHA256,
    #[token("SHA384")]
    SHA384,
    #[token("SHA512")]
    SHA512,
    #[token("COALESCE")]
    COALESCE,
    #[token("IF")]
    IF,
    #[token("STRLANG")]
    STRLANG,
    #[token("STRDT")]
    STRDT,
    #[token("sameTerm")]
    sameTerm,
    #[token("isIRI")]
    isIRI,
    #[token("isURI")]
    isURI,
    #[token("isBLANK")]
    isBLANK,
    #[token("isLITERAL")]
    isLITERAL,
    #[token("isNUMERIC")]
    isNUMERIC,
    #[token("REGEX")]
    REGEX,
    #[token("SUBSTR")]
    SUBSTR,
    #[token("REPLACE")]
    REPLACE,
    #[token("EXISTS")]
    EXISTS,
    #[token("COUNT")]
    COUNT,
    #[token("SUM")]
    SUM,
    #[token("MIN")]
    MIN,
    #[token("MAX")]
    MAX,
    #[token("AVG")]
    AVG,
    #[token("SAMPLE")]
    SAMPLE,
    #[token("GROUP_CONCAT")]
    GROUP_CONCAT,
    #[token("SEPARATOR")]
    SEPARATOR,
    #[token("LANGTAG")]
    LANGTAG,
    #[token("DoubleZirkumflex")]
    DoubleZirkumflex,
    #[token("DECIMAL")]
    DECIMAL,
    #[token("DOUBLE")]
    DOUBLE,
    #[token("INTEGER_POSITIVE")]
    INTEGER_POSITIVE,
    #[token("DECIMAL_POSITIVE")]
    DECIMAL_POSITIVE,
    #[token("DOUBLE_POSITIVE")]
    DOUBLE_POSITIVE,
    #[token("INTEGER_NEGATIVE")]
    INTEGER_NEGATIVE,
    #[token("DECIMAL_NEGATIVE")]
    DECIMAL_NEGATIVE,
    #[token("DOUBLE_NEGATIVE")]
    DOUBLE_NEGATIVE,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("STRING_LITERAL1")]
    STRING_LITERAL1,
    #[token("STRING_LITERAL2")]
    STRING_LITERAL2,
    #[token("STRING_LITERAL_LONG1")]
    STRING_LITERAL_LONG1,
    #[token("STRING_LITERAL_LONG2")]
    STRING_LITERAL_LONG2,

    // PN_CHARS : [A-Za-z\u{00C0}-\u{00D6}\u{00D8}-\u{00F6}\u{00F8}-\u{02FF}\u{0370}-\u{037D}\u{037F}-\u{1FFF}\u{200C}-\u{200D}\u{2070}-\u{218F}\u{2C00}-\u{2FEF}\u{3001}-\u{D7FF}\u{F900}-\u{FDCF}\u{FDF0}-\u{FFFD}\u{10000}-\u{EFFFF}0-9\u{00B7}\u{0300}-\u{036F}\u{203F}-\u{2040}_-]
    // PN_CHARS_BASE : [A-Za-z\u{00C0}-\u{00D6}\u{00D8}-\u{00F6}\u{00F8}-\u{02FF}\u{0370}-\u{037D}\u{037F}-\u{1FFF}\u{200C}-\u{200D}\u{2070}-\u{218F}\u{2C00}-\u{2FEF}\u{3001}-\u{D7FF}\u{F900}-\u{FDCF}\u{FDF0}-\u{FFFD}\u{10000}-\u{EFFFF}]
    // PN_CHARS_U : [A-Za-z\u{00C0}-\u{00D6}\u{00D8}-\u{00F6}\u{00F8}-\u{02FF}\u{0370}-\u{037D}\u{037F}-\u{1FFF}\u{200C}-\u{200D}\u{2070}-\u{218F}\u{2C00}-\u{2FEF}\u{3001}-\u{D7FF}\u{F900}-\u{FDCF}\u{FDF0}-\u{FFFD}\u{10000}-\u{EFFFF}_]
    #[regex("([A-Za-z\u{00C0}-\u{00D6}\u{00D8}-\u{00F6}\u{00F8}-\u{02FF}\u{0370}-\u{037D}\u{037F}-\u{1FFF}\u{200C}-\u{200D}\u{2070}-\u{218F}\u{2C00}-\u{2FEF}\u{3001}-\u{D7FF}\u{F900}-\u{FDCF}\u{FDF0}-\u{FFFD}\u{10000}-\u{EFFFF}_0-9:]|%[0-9A-Fa-f])([A-Za-z\u{00C0}-\u{00D6}\u{00D8}-\u{00F6}\u{00F8}-\u{02FF}\u{0370}-\u{037D}\u{037F}-\u{1FFF}\u{200C}-\u{200D}\u{2070}-\u{218F}\u{2C00}-\u{2FEF}\u{3001}-\u{D7FF}\u{F900}-\u{FDCF}\u{FDF0}-\u{FFFD}\u{10000}-\u{EFFFF}0-9\u{00B7}\u{0300}-\u{036F}\u{203F}-\u{2040}_:.-]|%[0-9A-Fa-f])*([A-Za-z\u{00C0}-\u{00D6}\u{00D8}-\u{00F6}\u{00F8}-\u{02FF}\u{0370}-\u{037D}\u{037F}-\u{1FFF}\u{200C}-\u{200D}\u{2070}-\u{218F}\u{2C00}-\u{2FEF}\u{3001}-\u{D7FF}\u{F900}-\u{FDCF}\u{FDF0}-\u{FFFD}\u{10000}-\u{EFFFF}0-9\u{00B7}\u{0300}-\u{036F}\u{203F}-\u{2040}_:-]|%[0-9A-Fa-f])", priority=1)]
    PNAME_LN,
    #[token("BLANK_NODE_LABEL")]
    BLANK_NODE_LABEL,
    #[token("ANON")]
    ANON,
}

#[wasm_bindgen]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Serialize)]
pub enum TreeKind {
    ErrorTree,
    QueryUnit,
    Query,
    Prologue,
    SelectQuery,
    ConstructQuery,
    DescribeQuery,
    AskQuery,
    ValuesClause,
    UpdateUnit,
    Update,
    BaseDecl,
    PrefixDecl,
    SelectClause,
    DatasetClause,
    WhereClause,
    SolutionModifier,
    SubSelect,
    Var,
    Expression,
    ConstructTemplate,
    TriplesTemplate,
    VarOrIri,
    DefaultGraphClause,
    NamedGraphClause,
    SourceSelector,
    iri,
    GroupGraphPattern,
    GroupClause,
    HavingClause,
    OrderClause,
    LimitOffsetClauses,
    GroupCondition,
    BuiltInCall,
    FunctionCall,
    HavingCondition,
    Constraint,
    OrderCondition,
    BrackettedExpression,
    LimitClause,
    OffsetClause,
    DataBlock,
    UpdateOne,
    Load,
    Clear,
    Drop,
    Move,
    Copy,
    Create,
    InsertData,
    DeleteData,
    DeleteWhere,
    Modify,
    GraphRef,
    GraphRefAll,
    Add,
    GraphOrDefault,
    QuadData,
    QuadPattern,
    DeleteClause,
    InsertClause,
    UsingClause,
    Quads,
    QuadsNotTriples,
    TriplesSameSubject,
    GroupGraphPatternSub,
    TriplesBlock,
    GraphPatternNotTriples,
    TriplesSameSubjectPath,
    GroupOrUnionGraphPattern,
    OptionalGraphPattern,
    MinusGraphPattern,
    GraphGraphPattern,
    ServiceGraphPattern,
    Filter,
    Bind,
    InlineData,
    InlineDataOneVar,
    InlineDataFull,
    DataBlockValue,
    RDFLiteral,
    NumericLiteral,
    BooleanLiteral,
    ArgList,
    ExpressionList,
    ConstructTriples,
    VarOrTerm,
    PropertyListNotEmpty,
    TriplesNode,
    PropertyList,
    Verb,
    ObjectList,
    Object,
    GraphNode,
    PropertyListPathNotEmpty,
    TriplesNodePath,
    PropertyListPath,
    VerbPath,
    VerbSimple,
    ObjectListPath,
    Path,
    ObjectPath,
    GraphNodePath,
    PathAlternative,
    PathSequence,
    PathEltOrInverse,
    PathElt,
    PathPrimary,
    PathMod,
    PathNegatedPropertySet,
    PathOneInPropertySet,
    Integer,
    Collection,
    BlankNodePropertyList,
    CollectionPath,
    BlankNodePropertyListPath,
    GraphTerm,
    BlankNode,
    ConditionalOrExpression,
    ConditionalAndExpression,
    ValueLogical,
    RelationalExpression,
    NumericExpression,
    AdditiveExpression,
    MultiplicativeExpression,
    NumericLiteralPositive,
    NumericLiteralNegative,
    UnaryExpression,
    PrimaryExpression,
    iriOrFunction,
    Aggregate,
    SubstringExpression,
    StrReplaceExpression,
    RegexExpression,
    ExistsFunc,
    NotExistsFunc,
    String,
    NumericLiteralUnsigned,
    PrefixedName,
}
