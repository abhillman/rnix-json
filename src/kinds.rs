use rnix::SyntaxKind;

#[derive(PartialEq)]
pub enum KindType {
    TOKEN,
    NODE,
    LAST,
}

pub fn kind_type(kind: SyntaxKind) -> KindType {
    match kind {
        SyntaxKind::TOKEN_COMMENT
        | SyntaxKind::TOKEN_ERROR
        | SyntaxKind::TOKEN_WHITESPACE
        | SyntaxKind::TOKEN_ASSERT
        | SyntaxKind::TOKEN_ELSE
        | SyntaxKind::TOKEN_IF
        | SyntaxKind::TOKEN_IN
        | SyntaxKind::TOKEN_INHERIT
        | SyntaxKind::TOKEN_LET
        | SyntaxKind::TOKEN_OR
        | SyntaxKind::TOKEN_REC
        | SyntaxKind::TOKEN_THEN
        | SyntaxKind::TOKEN_WITH
        | SyntaxKind::TOKEN_L_BRACE
        | SyntaxKind::TOKEN_R_BRACE
        | SyntaxKind::TOKEN_L_BRACK
        | SyntaxKind::TOKEN_R_BRACK
        | SyntaxKind::TOKEN_ASSIGN
        | SyntaxKind::TOKEN_AT
        | SyntaxKind::TOKEN_COLON
        | SyntaxKind::TOKEN_COMMA
        | SyntaxKind::TOKEN_DOT
        | SyntaxKind::TOKEN_ELLIPSIS
        | SyntaxKind::TOKEN_QUESTION
        | SyntaxKind::TOKEN_SEMICOLON
        | SyntaxKind::TOKEN_L_PAREN
        | SyntaxKind::TOKEN_R_PAREN
        | SyntaxKind::TOKEN_CONCAT
        | SyntaxKind::TOKEN_INVERT
        | SyntaxKind::TOKEN_UPDATE
        | SyntaxKind::TOKEN_ADD
        | SyntaxKind::TOKEN_SUB
        | SyntaxKind::TOKEN_MUL
        | SyntaxKind::TOKEN_DIV
        | SyntaxKind::TOKEN_AND_AND
        | SyntaxKind::TOKEN_EQUAL
        | SyntaxKind::TOKEN_IMPLICATION
        | SyntaxKind::TOKEN_LESS
        | SyntaxKind::TOKEN_LESS_OR_EQ
        | SyntaxKind::TOKEN_MORE
        | SyntaxKind::TOKEN_MORE_OR_EQ
        | SyntaxKind::TOKEN_NOT_EQUAL
        | SyntaxKind::TOKEN_OR_OR
        | SyntaxKind::TOKEN_FLOAT
        | SyntaxKind::TOKEN_IDENT
        | SyntaxKind::TOKEN_INTEGER
        | SyntaxKind::TOKEN_INTERPOL_END
        | SyntaxKind::TOKEN_INTERPOL_START
        | SyntaxKind::TOKEN_PATH
        | SyntaxKind::TOKEN_URI
        | SyntaxKind::TOKEN_STRING_CONTENT
        | SyntaxKind::TOKEN_STRING_END
        | SyntaxKind::TOKEN_STRING_START => KindType::TOKEN,
        SyntaxKind::NODE_APPLY
        | SyntaxKind::NODE_ASSERT
        | SyntaxKind::NODE_ATTRPATH
        | SyntaxKind::NODE_DYNAMIC
        | SyntaxKind::NODE_ERROR
        | SyntaxKind::NODE_IDENT
        | SyntaxKind::NODE_IF_ELSE
        | SyntaxKind::NODE_SELECT
        | SyntaxKind::NODE_INHERIT
        | SyntaxKind::NODE_INHERIT_FROM
        | SyntaxKind::NODE_STRING
        | SyntaxKind::NODE_INTERPOL
        | SyntaxKind::NODE_LAMBDA
        | SyntaxKind::NODE_IDENT_PARAM
        | SyntaxKind::NODE_LEGACY_LET
        | SyntaxKind::NODE_LET_IN
        | SyntaxKind::NODE_LIST
        | SyntaxKind::NODE_BIN_OP
        | SyntaxKind::NODE_PAREN
        | SyntaxKind::NODE_PATTERN
        | SyntaxKind::NODE_PAT_BIND
        | SyntaxKind::NODE_PAT_ENTRY
        | SyntaxKind::NODE_ROOT
        | SyntaxKind::NODE_ATTR_SET
        | SyntaxKind::NODE_ATTRPATH_VALUE
        | SyntaxKind::NODE_UNARY_OP
        | SyntaxKind::NODE_LITERAL
        | SyntaxKind::NODE_WITH
        | SyntaxKind::NODE_PATH
        | SyntaxKind::NODE_HAS_ATTR => KindType::NODE,
        SyntaxKind::__LAST => KindType::LAST,
    }
}

pub(crate) fn from_str(kind_str: &str) -> SyntaxKind {
    match kind_str {
        "TOKEN_COMMENT" => SyntaxKind::TOKEN_COMMENT,
        "TOKEN_ERROR" => SyntaxKind::TOKEN_ERROR,
        "TOKEN_WHITESPACE" => SyntaxKind::TOKEN_WHITESPACE,
        "TOKEN_ASSERT" => SyntaxKind::TOKEN_ASSERT,
        "TOKEN_ELSE" => SyntaxKind::TOKEN_ELSE,
        "TOKEN_IF" => SyntaxKind::TOKEN_IF,
        "TOKEN_IN" => SyntaxKind::TOKEN_IN,
        "TOKEN_INHERIT" => SyntaxKind::TOKEN_INHERIT,
        "TOKEN_LET" => SyntaxKind::TOKEN_LET,
        "TOKEN_OR" => SyntaxKind::TOKEN_OR,
        "TOKEN_REC" => SyntaxKind::TOKEN_REC,
        "TOKEN_THEN" => SyntaxKind::TOKEN_THEN,
        "TOKEN_WITH" => SyntaxKind::TOKEN_WITH,
        "TOKEN_L_BRACE" => SyntaxKind::TOKEN_L_BRACE,
        "TOKEN_R_BRACE" => SyntaxKind::TOKEN_R_BRACE,
        "TOKEN_L_BRACK" => SyntaxKind::TOKEN_L_BRACK,
        "TOKEN_R_BRACK" => SyntaxKind::TOKEN_R_BRACK,
        "TOKEN_ASSIGN" => SyntaxKind::TOKEN_ASSIGN,
        "TOKEN_AT" => SyntaxKind::TOKEN_AT,
        "TOKEN_COLON" => SyntaxKind::TOKEN_COLON,
        "TOKEN_COMMA" => SyntaxKind::TOKEN_COMMA,
        "TOKEN_DOT" => SyntaxKind::TOKEN_DOT,
        "TOKEN_ELLIPSIS" => SyntaxKind::TOKEN_ELLIPSIS,
        "TOKEN_QUESTION" => SyntaxKind::TOKEN_QUESTION,
        "TOKEN_SEMICOLON" => SyntaxKind::TOKEN_SEMICOLON,
        "TOKEN_L_PAREN" => SyntaxKind::TOKEN_L_PAREN,
        "TOKEN_R_PAREN" => SyntaxKind::TOKEN_R_PAREN,
        "TOKEN_CONCAT" => SyntaxKind::TOKEN_CONCAT,
        "TOKEN_INVERT" => SyntaxKind::TOKEN_INVERT,
        "TOKEN_UPDATE" => SyntaxKind::TOKEN_UPDATE,
        "TOKEN_ADD" => SyntaxKind::TOKEN_ADD,
        "TOKEN_SUB" => SyntaxKind::TOKEN_SUB,
        "TOKEN_MUL" => SyntaxKind::TOKEN_MUL,
        "TOKEN_DIV" => SyntaxKind::TOKEN_DIV,
        "TOKEN_AND_AND" => SyntaxKind::TOKEN_AND_AND,
        "TOKEN_EQUAL" => SyntaxKind::TOKEN_EQUAL,
        "TOKEN_IMPLICATION" => SyntaxKind::TOKEN_IMPLICATION,
        "TOKEN_LESS" => SyntaxKind::TOKEN_LESS,
        "TOKEN_LESS_OR_EQ" => SyntaxKind::TOKEN_LESS_OR_EQ,
        "TOKEN_MORE" => SyntaxKind::TOKEN_MORE,
        "TOKEN_MORE_OR_EQ" => SyntaxKind::TOKEN_MORE_OR_EQ,
        "TOKEN_NOT_EQUAL" => SyntaxKind::TOKEN_NOT_EQUAL,
        "TOKEN_OR_OR" => SyntaxKind::TOKEN_OR_OR,
        "TOKEN_FLOAT" => SyntaxKind::TOKEN_FLOAT,
        "TOKEN_IDENT" => SyntaxKind::TOKEN_IDENT,
        "TOKEN_INTEGER" => SyntaxKind::TOKEN_INTEGER,
        "TOKEN_INTERPOL_END" => SyntaxKind::TOKEN_INTERPOL_END,
        "TOKEN_INTERPOL_START" => SyntaxKind::TOKEN_INTERPOL_START,
        "TOKEN_PATH" => SyntaxKind::TOKEN_PATH,
        "TOKEN_URI" => SyntaxKind::TOKEN_URI,
        "TOKEN_STRING_CONTENT" => SyntaxKind::TOKEN_STRING_CONTENT,
        "TOKEN_STRING_END" => SyntaxKind::TOKEN_STRING_END,
        "TOKEN_STRING_START" => SyntaxKind::TOKEN_STRING_START,
        "NODE_APPLY" => SyntaxKind::NODE_APPLY,
        "NODE_ASSERT" => SyntaxKind::NODE_ASSERT,
        "NODE_ATTRPATH" => SyntaxKind::NODE_ATTRPATH,
        "NODE_DYNAMIC" => SyntaxKind::NODE_DYNAMIC,
        "NODE_ERROR" => SyntaxKind::NODE_ERROR,
        "NODE_IDENT" => SyntaxKind::NODE_IDENT,
        "NODE_IF_ELSE" => SyntaxKind::NODE_IF_ELSE,
        "NODE_SELECT" => SyntaxKind::NODE_SELECT,
        "NODE_INHERIT" => SyntaxKind::NODE_INHERIT,
        "NODE_INHERIT_FROM" => SyntaxKind::NODE_INHERIT_FROM,
        "NODE_STRING" => SyntaxKind::NODE_STRING,
        "NODE_INTERPOL" => SyntaxKind::NODE_INTERPOL,
        "NODE_LAMBDA" => SyntaxKind::NODE_LAMBDA,
        "NODE_IDENT_PARAM" => SyntaxKind::NODE_IDENT_PARAM,
        "NODE_LEGACY_LET" => SyntaxKind::NODE_LEGACY_LET,
        "NODE_LET_IN" => SyntaxKind::NODE_LET_IN,
        "NODE_LIST" => SyntaxKind::NODE_LIST,
        "NODE_BIN_OP" => SyntaxKind::NODE_BIN_OP,
        "NODE_PAREN" => SyntaxKind::NODE_PAREN,
        "NODE_PATTERN" => SyntaxKind::NODE_PATTERN,
        "NODE_PAT_BIND" => SyntaxKind::NODE_PAT_BIND,
        "NODE_PAT_ENTRY" => SyntaxKind::NODE_PAT_ENTRY,
        "NODE_ROOT" => SyntaxKind::NODE_ROOT,
        "NODE_ATTR_SET" => SyntaxKind::NODE_ATTR_SET,
        "NODE_ATTRPATH_VALUE" => SyntaxKind::NODE_ATTRPATH_VALUE,
        "NODE_UNARY_OP" => SyntaxKind::NODE_UNARY_OP,
        "NODE_LITERAL" => SyntaxKind::NODE_LITERAL,
        "NODE_WITH" => SyntaxKind::NODE_WITH,
        "NODE_PATH" => SyntaxKind::NODE_PATH,
        "NODE_HAS_ATTR" => SyntaxKind::NODE_HAS_ATTR,
        "__LAST" => SyntaxKind::__LAST,
        &_ => panic!("developer error"),
    }
}
