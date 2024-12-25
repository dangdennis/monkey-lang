use crate::token::{lookup_ident, Token, TokenType};

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
                token_type: TokenType::Eof,
                literal: None,
            },
            Some(ch) => match ch.as_str() {
                "=" => match self.peek_char() {
                    Some(peeked_ch) => {
                        if peeked_ch == "=" {
                            self.read_char();
                            Token {
                                token_type: TokenType::Eq,
                                literal: Some("==".to_string()),
                            }
                        } else {
                            Token {
                                token_type: TokenType::Assign,
                                literal: Some(String::from("=")),
                            }
                        }
                    }
                    None => Token {
                        token_type: TokenType::Assign,
                        literal: Some(String::from("=")),
                    },
                },
                ";" => Token {
                    token_type: TokenType::Semicolon,
                    literal: Some(String::from(";")),
                },
                "(" => Token {
                    token_type: TokenType::LParen,
                    literal: Some(String::from("(")),
                },
                ")" => Token {
                    token_type: TokenType::RParen,
                    literal: Some(String::from(")")),
                },
                "{" => Token {
                    token_type: TokenType::LBrace,
                    literal: Some(String::from("{")),
                },
                "}" => Token {
                    token_type: TokenType::RBrace,
                    literal: Some(String::from("}")),
                },
                "/" => Token {
                    token_type: TokenType::Slash,
                    literal: Some(String::from("/")),
                },
                "," => Token {
                    token_type: TokenType::Comma,
                    literal: Some(String::from(",")),
                },
                "+" => Token {
                    token_type: TokenType::Plus,
                    literal: Some(String::from("+")),
                },
                "-" => Token {
                    token_type: TokenType::Minus,
                    literal: Some(String::from("-")),
                },
                "*" => Token {
                    token_type: TokenType::Asterisk,
                    literal: Some(String::from("*")),
                },
                "<" => Token {
                    token_type: TokenType::Lt,
                    literal: Some(String::from("<")),
                },
                ">" => Token {
                    token_type: TokenType::Gt,
                    literal: Some(String::from(">")),
                },
                "!" => match self.peek_char() {
                    Some(peeked_ch) => {
                        if peeked_ch == "=" {
                            self.read_char();
                            Token {
                                token_type: TokenType::NotEq,
                                literal: Some("!=".to_string()),
                            }
                        } else {
                            Token {
                                token_type: TokenType::Bang,
                                literal: Some(String::from("!")),
                            }
                        }
                    }
                    None => Token {
                        token_type: TokenType::Bang,
                        literal: Some(String::from("!")),
                    },
                },
                _ => {
                    if is_letter(ch.chars().nth(0).unwrap()) {
                        let literal = self.read_identifier();
                        let token = Token {
                            token_type: lookup_ident(&literal),
                            literal: Some(literal),
                        };
                        println!("token1 {token:?}");
                        return token;
                    } else if is_digit(ch.chars().nth(0).unwrap()) {
                        let literal = self.read_number();
                        let token = Token {
                            token_type: TokenType::Int,
                            literal: Some(literal),
                        };
                        println!("token2 {token:?}");
                        return token;
                    } else {
                        Token {
                            token_type: TokenType::Illegal,
                            literal: None,
                        }
                    }
                }
            },
        };

        println!("token3 {token:?}");

        self.read_char();

        token
    }

    fn read_char(&mut self) {
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

    fn read_number(&mut self) -> String {
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

    fn read_identifier(&mut self) -> String {
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

    fn skip_whitespace(&mut self) {
        while let Some(ch) = &self.ch {
            match ch.as_str() {
                " " | "\t" | "\n" | "\r" => {
                    self.read_char();
                }
                _ => break,
            }
        }
    }

    fn peek_char(&mut self) -> Option<String> {
        if self.read_position as usize >= self.input.len() {
            None
        } else {
            self.input
                .chars()
                .nth(self.read_position as usize)
                .map(|ch| ch.to_string())
        }
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_ascii_lowercase() || ch.is_ascii_uppercase() || ch == '_' || ch == '?' || ch == '!'
}

fn is_digit(ch: char) -> bool {
    ch.is_ascii_digit()
}
