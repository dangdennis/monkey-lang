use crate::token::{Token, TokenType};

pub struct Lexer {
    pub input: String,
    pub position: u32,
    pub read_position: u32,
    pub ch: Option<String>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: None, // or a placeholder character like '\0'
        };

        lexer.read_char();

        lexer
    }

    pub fn read_char(&mut self) {
        if self.read_position as usize >= self.input.len() {
            self.ch = None;
        } else {
            let ch = self.input.chars().nth(self.read_position as usize);
            if let Some(ch) = ch {
                self.ch = Some(ch.to_string());
            }
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let (token_type, literal) = match self.ch.clone() {
            None => (TokenType::EOF, None),
            Some(ch) => match ch.as_str() {
                "=" => (TokenType::ASSIGN, Some(String::from("="))),
                ";" => (TokenType::SEMICOLON, Some(String::from(";"))),
                "(" => (TokenType::LPAREN, Some(String::from("("))),
                ")" => (TokenType::RPAREN, Some(String::from(")"))),
                "," => (TokenType::COMMA, Some(String::from(","))),
                "+" => (TokenType::PLUS, Some(String::from("+"))),
                "{" => (TokenType::LBRACE, Some(String::from("{"))),
                "}" => (TokenType::RBRACE, Some(String::from("}"))),
                _ => {
                    if is_letter(ch.chars().nth(0).unwrap()) {
                        let literal = self.read_identifier();
                        (TokenType::IDENT, Some(literal))
                    } else {
                        (TokenType::ILLEGAL, None)
                    }
                }
            },
        };

        self.read_char();

        Token {
            token_type,
            literal,
        }
    }

    pub fn read_identifier(&mut self) -> String {
        let position = self.position.clone();
        if let Some(ch) = self.ch.clone() {
            while is_letter(ch.chars().nth(0).unwrap()) {
                self.read_char();
            }
        }

        let chars: Vec<char> = self.input.chars().collect();
        let ident = chars[position as usize..self.read_position as usize]
            .iter()
            .collect();

        ident
    }
}

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_' || ch == '?' || ch == '!'
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, token::TokenType};

    #[test]
    fn test_next_token_simple() {
        let input = "=+(){},;";

        struct Test {
            expected_type: TokenType,
            expected_literal: Option<String>,
        }

        let tests = [
            Test {
                expected_type: TokenType::ASSIGN,
                expected_literal: Some("=".to_string()),
            },
            Test {
                expected_type: TokenType::PLUS,
                expected_literal: Some("+".to_string()),
            },
            Test {
                expected_type: TokenType::LPAREN,
                expected_literal: Some("(".to_string()),
            },
            Test {
                expected_type: TokenType::RPAREN,
                expected_literal: Some(")".to_string()),
            },
            Test {
                expected_type: TokenType::LBRACE,
                expected_literal: Some("{".to_string()),
            },
            Test {
                expected_type: TokenType::RBRACE,
                expected_literal: Some("}".to_string()),
            },
            Test {
                expected_type: TokenType::COMMA,
                expected_literal: Some(",".to_string()),
            },
            Test {
                expected_type: TokenType::SEMICOLON,
                expected_literal: Some(";".to_string()),
            },
        ];

        let mut lexer = Lexer::new(input.to_string());

        for test in tests.iter() {
            let token = lexer.next_token();
            assert_eq!(token.token_type, test.expected_type);
            assert_eq!(token.literal, test.expected_literal);
        }
    }

    #[test]
    fn test_next_token_syntax() {
        let input = r#"
            let five = 5;
            let ten = 10”

            “let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);”
        "#;

        struct Test {
            expected_type: TokenType,
            expected_literal: Option<String>,
        }

        let tests = [
            Test {
                expected_type: TokenType::LET,
                expected_literal: Some("let".to_string()),
            },
            Test {
                expected_type: TokenType::IDENT,
                expected_literal: Some("five".to_string()),
            },
            Test {
                expected_type: TokenType::ASSIGN,
                expected_literal: Some("=".to_string()),
            },
            Test {
                expected_type: TokenType::INT,
                expected_literal: Some("5".to_string()),
            },
            Test {
                expected_type: TokenType::SEMICOLON,
                expected_literal: Some(";".to_string()),
            },
            Test {
                expected_type: TokenType::LET,
                expected_literal: Some("let".to_string()),
            },
            Test {
                expected_type: TokenType::IDENT,
                expected_literal: Some("ten".to_string()),
            },
            Test {
                expected_type: TokenType::ASSIGN,
                expected_literal: Some("=".to_string()),
            },
            Test {
                expected_type: TokenType::INT,
                expected_literal: Some("10".to_string()),
            },
            Test {
                expected_type: TokenType::SEMICOLON,
                expected_literal: Some(";".to_string()),
            },
            Test {
                expected_type: TokenType::LET,
                expected_literal: Some("let".to_string()),
            },
            Test {
                expected_type: TokenType::IDENT,
                expected_literal: Some("add".to_string()),
            },
            Test {
                expected_type: TokenType::ASSIGN,
                expected_literal: Some("=".to_string()),
            },
            Test {
                expected_type: TokenType::FUNCTION,
                expected_literal: Some("fn".to_string()),
            },
            Test {
                expected_type: TokenType::LPAREN,
                expected_literal: Some("(".to_string()),
            },
            Test {
                expected_type: TokenType::IDENT,
                expected_literal: Some("x".to_string()),
            },
            Test {
                expected_type: TokenType::COMMA,
                expected_literal: Some(",".to_string()),
            },
            Test {
                expected_type: TokenType::IDENT,
                expected_literal: Some("y".to_string()),
            },
            Test {
                expected_type: TokenType::RPAREN,
                expected_literal: Some(")".to_string()),
            },
            Test {
                expected_type: TokenType::LBRACE,
                expected_literal: Some("{".to_string()),
            },
            Test {
                expected_type: TokenType::IDENT,
                expected_literal: Some("x".to_string()),
            },
            Test {
                expected_type: TokenType::PLUS,
                expected_literal: Some("+".to_string()),
            },
            Test {
                expected_type: TokenType::IDENT,
                expected_literal: Some("y".to_string()),
            },
            Test {
                expected_type: TokenType::SEMICOLON,
                expected_literal: Some(";".to_string()),
            },
            Test {
                expected_type: TokenType::RBRACE,
                expected_literal: Some("}".to_string()),
            },
            Test {
                expected_type: TokenType::SEMICOLON,
                expected_literal: Some(";".to_string()),
            },
            Test {
                expected_type: TokenType::LET,
                expected_literal: Some("let".to_string()),
            },
            Test {
                expected_type: TokenType::IDENT,
                expected_literal: Some("result".to_string()),
            },
            Test {
                expected_type: TokenType::ASSIGN,
                expected_literal: Some("=".to_string()),
            },
            Test {
                expected_type: TokenType::IDENT,
                expected_literal: Some("add".to_string()),
            },
            Test {
                expected_type: TokenType::LPAREN,
                expected_literal: Some("(".to_string()),
            },
            Test {
                expected_type: TokenType::IDENT,
                expected_literal: Some("five".to_string()),
            },
            Test {
                expected_type: TokenType::COMMA,
                expected_literal: Some(",".to_string()),
            },
            Test {
                expected_type: TokenType::IDENT,
                expected_literal: Some("ten".to_string()),
            },
            Test {
                expected_type: TokenType::RPAREN,
                expected_literal: Some(")".to_string()),
            },
            Test {
                expected_type: TokenType::SEMICOLON,
                expected_literal: Some(";".to_string()),
            },
            Test {
                expected_type: TokenType::EOF,
                expected_literal: None,
            },
        ];
    }
}
