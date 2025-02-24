use logos::{Logos, Span};
#[allow(non_camel_case_types)]
#[derive(Logos, Debug, PartialEq, Clone, Copy)]
#[logos(skip r"[ \t\n\f]+")]
pub enum TokenKind {
    Eof,
    ErrorToken,
    #[token("BASE", ignore(case))]
    BASE,
    #[token("IRIREF", ignore(case))]
    IRIREF,
    #[token("PREFIX", ignore(case))]
    PREFIX,
    #[token("PNAME_NS", ignore(case))]
    PNAME_NS,
    #[token("SELECT", ignore(case))]
    SELECT,
    #[token("DISTINCT", ignore(case))]
    DISTINCT,
    #[token("REDUCED", ignore(case))]
    REDUCED,
    #[token("LParen", ignore(case))]
    LParen,
    #[token("AS", ignore(case))]
    AS,
    #[token("RParen", ignore(case))]
    RParen,
    #[token("Star", ignore(case))]
    Star,
    #[token("CONSTRUCT", ignore(case))]
    CONSTRUCT,
    #[token("WHERE", ignore(case))]
    WHERE,
    #[token("LCurly", ignore(case))]
    LCurly,
    #[token("RCurly", ignore(case))]
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
    #[token("INSERT_DATA", ignore(case))]
    INSERT_DATA,
    #[token("DELETE_DATA", ignore(case))]
    DELETE_DATA,
    #[token("DELETE_WHERE", ignore(case))]
    DELETE_WHERE,
    #[token("WITH", ignore(case))]
    WITH,
    #[token("DELETE", ignore(case))]
    DELETE,
    #[token("INSERT", ignore(case))]
    INSERT,
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
    #[token("Colon", ignore(case))]
    Colon,
    #[token("a", ignore(case))]
    a,
    #[token("Pipe", ignore(case))]
    Pipe,
    #[token("Slash", ignore(case))]
    Slash,
    #[token("Zirkumflex", ignore(case))]
    Zirkumflex,
    #[token("QuestionMark", ignore(case))]
    QuestionMark,
    #[token("Plus", ignore(case))]
    Plus,
    #[token("ExclamationMark", ignore(case))]
    ExclamationMark,
    #[token("LBrack", ignore(case))]
    LBrack,
    #[token("RBrack", ignore(case))]
    RBrack,
    #[token("VAR1", ignore(case))]
    VAR1,
    #[token("VAR2", ignore(case))]
    VAR2,
    #[token("DoublePipe", ignore(case))]
    DoublePipe,
    #[token("DoubleAnd", ignore(case))]
    DoubleAnd,
    #[token("Equals", ignore(case))]
    Equals,
    #[token("ExclamationMarkEquals", ignore(case))]
    ExclamationMarkEquals,
    #[token("Less", ignore(case))]
    Less,
    #[token("More", ignore(case))]
    More,
    #[token("LessEquals", ignore(case))]
    LessEquals,
    #[token("MoreEquals", ignore(case))]
    MoreEquals,
    #[token("IN", ignore(case))]
    IN,
    #[token("NOT", ignore(case))]
    NOT,
    #[token("Minus", ignore(case))]
    Minus,
    #[token("STR", ignore(case))]
    STR,
    #[token("LANG", ignore(case))]
    LANG,
    #[token("LANGMATCHES", ignore(case))]
    LANGMATCHES,
    #[token("DATATYPE", ignore(case))]
    DATATYPE,
    #[token("BOUND", ignore(case))]
    BOUND,
    #[token("IRI", ignore(case))]
    IRI,
    #[token("URI", ignore(case))]
    URI,
    #[token("BNODE", ignore(case))]
    BNODE,
    #[token("RAND", ignore(case))]
    RAND,
    #[token("ABS", ignore(case))]
    ABS,
    #[token("CEIL", ignore(case))]
    CEIL,
    #[token("FLOOR", ignore(case))]
    FLOOR,
    #[token("ROUND", ignore(case))]
    ROUND,
    #[token("CONCAT", ignore(case))]
    CONCAT,
    #[token("STRLEN", ignore(case))]
    STRLEN,
    #[token("UCASE", ignore(case))]
    UCASE,
    #[token("LCASE", ignore(case))]
    LCASE,
    #[token("ENCODE_FOR_URI", ignore(case))]
    ENCODE_FOR_URI,
    #[token("CONTAINS", ignore(case))]
    CONTAINS,
    #[token("STRSTARTS", ignore(case))]
    STRSTARTS,
    #[token("STRENDS", ignore(case))]
    STRENDS,
    #[token("STRBEFORE", ignore(case))]
    STRBEFORE,
    #[token("STRAFTER", ignore(case))]
    STRAFTER,
    #[token("YEAR", ignore(case))]
    YEAR,
    #[token("MONTH", ignore(case))]
    MONTH,
    #[token("DAY", ignore(case))]
    DAY,
    #[token("HOURS", ignore(case))]
    HOURS,
    #[token("MINUTES", ignore(case))]
    MINUTES,
    #[token("SECONDS", ignore(case))]
    SECONDS,
    #[token("TIMEZONE", ignore(case))]
    TIMEZONE,
    #[token("TZ", ignore(case))]
    TZ,
    #[token("NOW", ignore(case))]
    NOW,
    #[token("UUID", ignore(case))]
    UUID,
    #[token("STRUUID", ignore(case))]
    STRUUID,
    #[token("MD5", ignore(case))]
    MD5,
    #[token("SHA1", ignore(case))]
    SHA1,
    #[token("SHA256", ignore(case))]
    SHA256,
    #[token("SHA384", ignore(case))]
    SHA384,
    #[token("SHA512", ignore(case))]
    SHA512,
    #[token("COALESCE", ignore(case))]
    COALESCE,
    #[token("IF", ignore(case))]
    IF,
    #[token("STRLANG", ignore(case))]
    STRLANG,
    #[token("STRDT", ignore(case))]
    STRDT,
    #[token("sameTerm", ignore(case))]
    sameTerm,
    #[token("isIRI", ignore(case))]
    isIRI,
    #[token("isURI", ignore(case))]
    isURI,
    #[token("isBLANK", ignore(case))]
    isBLANK,
    #[token("isLITERAL", ignore(case))]
    isLITERAL,
    #[token("isNUMERIC", ignore(case))]
    isNUMERIC,
    #[token("REGEX", ignore(case))]
    REGEX,
    #[token("SUBSTR", ignore(case))]
    SUBSTR,
    #[token("REPLACE", ignore(case))]
    REPLACE,
    #[token("EXISTS", ignore(case))]
    EXISTS,
    #[token("COUNT", ignore(case))]
    COUNT,
    #[token("SUM", ignore(case))]
    SUM,
    #[token("MIN", ignore(case))]
    MIN,
    #[token("MAX", ignore(case))]
    MAX,
    #[token("AVG", ignore(case))]
    AVG,
    #[token("SAMPLE", ignore(case))]
    SAMPLE,
    #[token("GROUP_CONCAT", ignore(case))]
    GROUP_CONCAT,
    #[token("SEPARATOR", ignore(case))]
    SEPARATOR,
    #[token("LANGTAG", ignore(case))]
    LANGTAG,
    #[token("DoubleZirkumflex", ignore(case))]
    DoubleZirkumflex,
    #[token("DECIMAL", ignore(case))]
    DECIMAL,
    #[token("DOUBLE", ignore(case))]
    DOUBLE,
    #[token("INTEGER_POSITIVE", ignore(case))]
    INTEGER_POSITIVE,
    #[token("DECIMAL_POSITIVE", ignore(case))]
    DECIMAL_POSITIVE,
    #[token("DOUBLE_POSITIVE", ignore(case))]
    DOUBLE_POSITIVE,
    #[token("INTEGER_NEGATIVE", ignore(case))]
    INTEGER_NEGATIVE,
    #[token("DECIMAL_NEGATIVE", ignore(case))]
    DECIMAL_NEGATIVE,
    #[token("DOUBLE_NEGATIVE", ignore(case))]
    DOUBLE_NEGATIVE,
    #[token("True", ignore(case))]
    True,
    #[token("False", ignore(case))]
    False,
    #[token("STRING_LITERAL1", ignore(case))]
    STRING_LITERAL1,
    #[token("STRING_LITERAL2", ignore(case))]
    STRING_LITERAL2,
    #[token("STRING_LITERAL_LONG1", ignore(case))]
    STRING_LITERAL_LONG1,
    #[token("STRING_LITERAL_LONG2", ignore(case))]
    STRING_LITERAL_LONG2,
    #[token("PNAME_LN", ignore(case))]
    PNAME_LN,
    #[token("BLANK_NODE_LABEL", ignore(case))]
    BLANK_NODE_LABEL,
    #[token("ANON", ignore(case))]
    ANON,
}
#[allow(non_camel_case_types)]
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
    Add,
    Move,
    Copy,
    Create,
    InsertData,
    DeleteData,
    DeleteWhere,
    Modify,
    GraphRef,
    GraphRefAll,
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
