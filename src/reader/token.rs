use lexer;


pub type Token = lexer::Token<TokenKind>;
pub type TokenMeta = lexer::TokenMeta;


#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TokenKind {
    WHITESPACE,
    COMMENT,

    IDENTIFIER,

    L_PAREN,
    R_PAREN,

    NIL,
    BOOL,
    NUMBER,
    STRING,
    CHAR,
}
