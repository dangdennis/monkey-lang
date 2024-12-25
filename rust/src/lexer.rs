use std::{iter::Peekable, str::Chars};

use crate::token::{lookup_ident, Token, TokenType};

pub struct Lexer<'a> {
    pub input: &'a str,
    chars: Peekable<Chars<'a>>,
    current: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars().peekable();
        let current = chars.next();
        Self {
            input,
            chars,
            current,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.current {
            None => Token::eof(),
            Some(ch) => match ch {
                '=' => self.handle_equals(),
                '!' => self.handle_bang(),
                ch if is_letter(ch) => self.read_identifier(),
                ch if is_digit(ch) => self.read_number(),
                ch => {
                    let token = Token::new(
                        match ch {
                            ';' => TokenType::Semicolon,
                            '(' => TokenType::LParen,
                            ')' => TokenType::RParen,
                            '{' => TokenType::LBrace,
                            '}' => TokenType::RBrace,
                            '+' => TokenType::Plus,
                            '-' => TokenType::Minus,
                            '*' => TokenType::Asterisk,
                            '/' => TokenType::Slash,
                            ',' => TokenType::Comma,
                            '<' => TokenType::Lt,
                            '>' => TokenType::Gt,
                            _ => return Token::illegal(),
                        },
                        ch,
                    );

                    self.advance();

                    token
                }
            },
        };

        token
    }

    fn advance(&mut self) {
        self.current = self.chars.next();
    }

    fn advance_steps(&mut self, step: usize) {
        for _ in 0..step {
            self.current = self.chars.next();
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn read_number(&mut self) -> Token {
        let mut number = String::new();
        while let Some(ch) = self.current {
            if !is_digit(ch) {
                break;
            }
            number.push(ch);
            self.advance();
        }

        Token {
            token_type: TokenType::Int,
            literal: Some(number),
        }
    }

    fn read_identifier(&mut self) -> Token {
        let mut identifier = String::new();
        while let Some(ch) = self.current {
            if !is_letter(ch) {
                break;
            }
            identifier.push(ch);
            self.advance();
        }

        Token {
            token_type: lookup_ident(&identifier),
            literal: Some(identifier),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current {
            if !ch.is_whitespace() {
                break;
            }
            self.advance();
        }
    }

    fn handle_equals(&mut self) -> Token {
        let peeked = self.peek();
        if let Some('=') = peeked {
            self.advance_steps(2);
            Token::new(TokenType::Eq, "==".to_string())
        } else {
            self.advance();
            Token::new(TokenType::Assign, "=".to_string())
        }
    }

    fn handle_bang(&mut self) -> Token {
        if let Some('=') = self.peek() {
            self.advance_steps(2);
            Token::new(TokenType::NotEq, "!=".to_string())
        } else {
            self.advance();
            Token::new(TokenType::Bang, "!".to_string())
        }
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_ascii_lowercase() || ch.is_ascii_uppercase() || ch == '_' || ch == '?' || ch == '!'
}

fn is_digit(ch: char) -> bool {
    ch.is_ascii_digit()
}
