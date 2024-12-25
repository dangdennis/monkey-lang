#[derive(Debug, Clone, PartialEq)]
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
    pub literal: Option<String>,
}

impl Token {
    pub fn new(token_type: TokenType, literal: impl Into<String>) -> Token {
        Token {
            token_type,
            literal: Some(literal.into()),
        }
    }

    pub fn eof() -> Self {
        Self {
            token_type: TokenType::Eof,
            literal: None,
        }
    }

    pub fn illegal() -> Self {
        Self {
            token_type: TokenType::Illegal,
            literal: None,
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
