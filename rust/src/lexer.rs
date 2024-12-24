use crate::token::{self, Token, TokenType};

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

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch.clone() {
            None => Token {
                token_type: TokenType::EOF,
                literal: None,
            },
            Some(ch) => match ch.as_str() {
                "=" => Token {
                    token_type: TokenType::ASSIGN,
                    literal: Some(String::from("=")),
                },
                ";" => Token {
                    token_type: TokenType::SEMICOLON,
                    literal: Some(String::from(";")),
                },
                "(" => Token {
                    token_type: TokenType::LPAREN,
                    literal: Some(String::from("(")),
                },
                ")" => Token {
                    token_type: TokenType::RPAREN,
                    literal: Some(String::from(")")),
                },
                "," => Token {
                    token_type: TokenType::COMMA,
                    literal: Some(String::from(",")),
                },
                "+" => Token {
                    token_type: TokenType::PLUS,
                    literal: Some(String::from("+")),
                },
                "{" => Token {
                    token_type: TokenType::LBRACE,
                    literal: Some(String::from("{")),
                },
                "}" => Token {
                    token_type: TokenType::RBRACE,
                    literal: Some(String::from("}")),
                },
                _ => {
                    if is_letter(ch.chars().nth(0).unwrap()) {
                        let literal = self.read_identifier();
                        return Token {
                            token_type: token::lookup_ident(&literal),
                            literal: Some(literal),
                        };
                    } else if is_digit(ch.chars().nth(0).unwrap()) {
                        let literal = self.read_number();
                        return Token {
                            token_type: TokenType::INT,
                            literal: Some(literal),
                        };
                    } else {
                        Token {
                            token_type: TokenType::ILLEGAL,
                            literal: None,
                        }
                    }
                }
            },
        };

        self.read_char();
        token
    }

    pub fn read_char(&mut self) {
        if self.read_position as usize >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = Some(
                self.input
                    .chars()
                    .nth(self.read_position as usize)
                    .unwrap()
                    .to_string(),
            );
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn read_number(&mut self) -> String {
        let position = self.position;
        while let Some(ch) = self.ch.as_ref() {
            if is_digit(ch.chars().nth(0).unwrap()) {
                self.read_char();
            } else {
                break;
            }
        }

        let chars: Vec<char> = self.input.chars().collect();
        let ident: String = chars[position as usize..self.position as usize]
            .iter()
            .collect();

        ident
    }

    pub fn read_identifier(&mut self) -> String {
        let position = self.position;
        while let Some(ch) = self.ch.as_ref() {
            if is_letter(ch.chars().nth(0).unwrap()) {
                self.read_char();
            } else {
                break;
            }
        }

        let chars: Vec<char> = self.input.chars().collect();
        let ident = chars[position as usize..self.position as usize]
            .iter()
            .collect();

        ident
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(ch) = &self.ch {
            match ch.as_str() {
                " " | "\t" | "\n" | "\r" => {
                    self.read_char();
                }
                _ => break,
            }
        }
    }
}

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z'
        || 'A' <= ch && ch <= 'Z'
        || ch == '_'
        || ch == '?'
        || ch == '!'
        || ch == '?'
        || ch == '!'
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, token::TokenType};

    #[test]
    fn test_next_token_simple() {
        let input = "=+(){},;";

        #[derive(Debug)]
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
            let ten = 10;

            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);
        "#;

        #[derive(Debug)]
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

        let mut lexer = Lexer::new(input.to_string());
        for test in tests.iter() {
            let token = lexer.next_token();
            assert_eq!(
                token.literal, test.expected_literal,
                "Expected literal {:?}, got {:?}",
                test.expected_literal, token.literal
            );
            assert_eq!(
                token.token_type, test.expected_type,
                "Expected token type {:?}, got {:?}",
                test.expected_type, token.token_type
            );
        }
    }
}
