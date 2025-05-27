#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    Illegal,
    Eof,

    // Identifiers + literals
    Ident,
    Int,

    // Operators
    Assign,
    Plus,
    Bang,
    Minus,
    Slash,
    Asterisk,
    Lt,
    Gt,
    Eq,
    NotEq,

    // Delimiters
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
    If,
    Return,
    True,
    False,
    Else,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: impl Into<String>) -> Token {
        Token {
            token_type,
            literal: literal.into(),
        }
    }

    pub fn eof() -> Self {
        Self {
            token_type: TokenType::Eof,
            literal: String::new(),
        }
    }

    pub fn illegal() -> Self {
        Self {
            token_type: TokenType::Illegal,
            literal: String::new(),
        }
    }
}

pub fn lookup_ident(ident: &str) -> TokenType {
    match ident {
        "fn" => TokenType::Function,
        "let" => TokenType::Let,
        "true" => TokenType::True,
        "false" => TokenType::False,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "return" => TokenType::Return,
        _ => TokenType::Ident,
    }
}

#[cfg(test)]
mod test {
    use crate::token::{lookup_ident, TokenType};

    #[test]
    fn test_lookup_ident() {
        assert_eq!(lookup_ident("fn"), TokenType::Function);
        assert_eq!(lookup_ident("let"), TokenType::Let);
        assert_eq!(lookup_ident("true"), TokenType::True);
        assert_eq!(lookup_ident("false"), TokenType::False);
        assert_eq!(lookup_ident("if"), TokenType::If);
        assert_eq!(lookup_ident("else"), TokenType::Else);
        assert_eq!(lookup_ident("return"), TokenType::Return);
        assert_eq!(lookup_ident("foobar"), TokenType::Ident);
    }
}
