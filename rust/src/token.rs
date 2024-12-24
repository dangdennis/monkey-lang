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

pub fn lookup_ident(ident: &str) -> TokenType {
    println!("ident: {}", ident);
    match ident {
        "fn" => TokenType::FUNCTION,
        "let" => TokenType::LET,
        _ => TokenType::IDENT,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_ident() {
        assert_eq!(lookup_ident("fn"), TokenType::FUNCTION);
        assert_eq!(lookup_ident("let"), TokenType::LET);
        assert_eq!(lookup_ident("foobar"), TokenType::IDENT);
    }
}
