#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    // Identifiers + literals
    IDENT,
    INT,

    // Operators
    ASSIGN,
    PLUS,

    // Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION,
    LET,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: Option<String>,
}

static KEYWORDS: &[(&str, TokenType)] = &[("fn", TokenType::FUNCTION), ("let", TokenType::LET)];

pub fn lookup_ident(ident: &str) -> TokenType {
    KEYWORDS
        .iter()
        .find(|item| item.0 == ident)
        .map_or(TokenType::IDENT, |ident| ident.1.clone())
}
