use std::{iter::Peekable, str::Chars};

use crate::token::{lookup_ident, Token, TokenType};

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    current: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars().peekable();
        let current = chars.next();
        Self { chars, current }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        match self.current {
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
        }
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
            literal: number,
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
            literal: identifier,
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

#[cfg(test)]
mod test {
    use crate::lexer::Lexer;
    use crate::token::TokenType;

    #[test]
    fn test_next_token_simple() {
        let input = "=+(){},;*/-+!";

        #[derive(Debug)]
        struct Test {
            expected_type: TokenType,
            expected_literal: String,
        }

        let tests = [
            Test {
                expected_type: TokenType::Assign,
                expected_literal: "=".to_string(),
            },
            Test {
                expected_type: TokenType::Plus,
                expected_literal: "+".to_string(),
            },
            Test {
                expected_type: TokenType::LParen,
                expected_literal: "(".to_string(),
            },
            Test {
                expected_type: TokenType::RParen,
                expected_literal: ")".to_string(),
            },
            Test {
                expected_type: TokenType::LBrace,
                expected_literal: "{".to_string(),
            },
            Test {
                expected_type: TokenType::RBrace,
                expected_literal: "}".to_string(),
            },
            Test {
                expected_type: TokenType::Comma,
                expected_literal: ",".to_string(),
            },
            Test {
                expected_type: TokenType::Semicolon,
                expected_literal: ";".to_string(),
            },
            Test {
                expected_type: TokenType::Asterisk,
                expected_literal: "*".to_string(),
            },
            Test {
                expected_type: TokenType::Slash,
                expected_literal: "/".to_string(),
            },
            Test {
                expected_type: TokenType::Minus,
                expected_literal: "-".to_string(),
            },
            Test {
                expected_type: TokenType::Plus,
                expected_literal: "+".to_string(),
            },
            Test {
                expected_type: TokenType::Bang,
                expected_literal: "!".to_string(),
            },
        ];

        let mut lexer = Lexer::new(input);

        for test in tests.iter() {
            let token = lexer.next_token();
            assert_eq!(token.token_type, test.expected_type);
            assert_eq!(token.literal, test.expected_literal);
        }
    }

    #[test]
    fn test_next_token_syntax() {
        let input = r#"let five = 5;
      let ten = 10;

      let add = fn(x, y) {
        x + y;
      };

      let result = add(five, ten);
      !-/*5;
      5 < 10 > 5;

      if (5 < 10) {
      return true;
      } else {
      return false;
      }

      10 == 10;
      10 != 9;
           "#;

        #[derive(Debug)]
        struct Test {
            expected_type: TokenType,
            expected_literal: String,
        }

        let tests = [
            Test {
                expected_type: TokenType::Let,
                expected_literal: "let".to_string(),
            },
            Test {
                expected_type: TokenType::Ident,
                expected_literal: "five".to_string(),
            },
            Test {
                expected_type: TokenType::Assign,
                expected_literal: "=".to_string(),
            },
            Test {
                expected_type: TokenType::Int,
                expected_literal: "5".to_string(),
            },
            Test {
                expected_type: TokenType::Semicolon,
                expected_literal: ";".to_string(),
            },
            Test {
                expected_type: TokenType::Let,
                expected_literal: "let".to_string(),
            },
            Test {
                expected_type: TokenType::Ident,
                expected_literal: "ten".to_string(),
            },
            Test {
                expected_type: TokenType::Assign,
                expected_literal: "=".to_string(),
            },
            Test {
                expected_type: TokenType::Int,
                expected_literal: "10".to_string(),
            },
            Test {
                expected_type: TokenType::Semicolon,
                expected_literal: ";".to_string(),
            },
            Test {
                expected_type: TokenType::Let,
                expected_literal: "let".to_string(),
            },
            Test {
                expected_type: TokenType::Ident,
                expected_literal: "add".to_string(),
            },
            Test {
                expected_type: TokenType::Assign,
                expected_literal: "=".to_string(),
            },
            Test {
                expected_type: TokenType::Function,
                expected_literal: "fn".to_string(),
            },
            Test {
                expected_type: TokenType::LParen,
                expected_literal: "(".to_string(),
            },
            Test {
                expected_type: TokenType::Ident,
                expected_literal: "x".to_string(),
            },
            Test {
                expected_type: TokenType::Comma,
                expected_literal: ",".to_string(),
            },
            Test {
                expected_type: TokenType::Ident,
                expected_literal: "y".to_string(),
            },
            Test {
                expected_type: TokenType::RParen,
                expected_literal: ")".to_string(),
            },
            Test {
                expected_type: TokenType::LBrace,
                expected_literal: "{".to_string(),
            },
            Test {
                expected_type: TokenType::Ident,
                expected_literal: "x".to_string(),
            },
            Test {
                expected_type: TokenType::Plus,
                expected_literal: "+".to_string(),
            },
            Test {
                expected_type: TokenType::Ident,
                expected_literal: "y".to_string(),
            },
            Test {
                expected_type: TokenType::Semicolon,
                expected_literal: ";".to_string(),
            },
            Test {
                expected_type: TokenType::RBrace,
                expected_literal: "}".to_string(),
            },
            Test {
                expected_type: TokenType::Semicolon,
                expected_literal: ";".to_string(),
            },
            Test {
                expected_type: TokenType::Let,
                expected_literal: "let".to_string(),
            },
            Test {
                expected_type: TokenType::Ident,
                expected_literal: "result".to_string(),
            },
            Test {
                expected_type: TokenType::Assign,
                expected_literal: "=".to_string(),
            },
            Test {
                expected_type: TokenType::Ident,
                expected_literal: "add".to_string(),
            },
            Test {
                expected_type: TokenType::LParen,
                expected_literal: "(".to_string(),
            },
            Test {
                expected_type: TokenType::Ident,
                expected_literal: "five".to_string(),
            },
            Test {
                expected_type: TokenType::Comma,
                expected_literal: ",".to_string(),
            },
            Test {
                expected_type: TokenType::Ident,
                expected_literal: "ten".to_string(),
            },
            Test {
                expected_type: TokenType::RParen,
                expected_literal: ")".to_string(),
            },
            Test {
                expected_type: TokenType::Semicolon,
                expected_literal: ";".to_string(),
            },
            Test {
                expected_type: TokenType::Bang,
                expected_literal: "!".to_string(),
            },
            Test {
                expected_type: TokenType::Minus,
                expected_literal: "-".to_string(),
            },
            Test {
                expected_type: TokenType::Slash,
                expected_literal: "/".to_string(),
            },
            Test {
                expected_type: TokenType::Asterisk,
                expected_literal: "*".to_string(),
            },
            Test {
                expected_type: TokenType::Int,
                expected_literal: "5".to_string(),
            },
            Test {
                expected_type: TokenType::Semicolon,
                expected_literal: ";".to_string(),
            },
            Test {
                expected_type: TokenType::Int,
                expected_literal: "5".to_string(),
            },
            Test {
                expected_type: TokenType::Lt,
                expected_literal: "<".to_string(),
            },
            Test {
                expected_type: TokenType::Int,
                expected_literal: "10".to_string(),
            },
            Test {
                expected_type: TokenType::Gt,
                expected_literal: ">".to_string(),
            },
            Test {
                expected_type: TokenType::Int,
                expected_literal: "5".to_string(),
            },
            Test {
                expected_type: TokenType::Semicolon,
                expected_literal: ";".to_string(),
            },
            Test {
                expected_type: TokenType::If,
                expected_literal: "if".to_string(),
            },
            Test {
                expected_type: TokenType::LParen,
                expected_literal: "(".to_string(),
            },
            Test {
                expected_type: TokenType::Int,
                expected_literal: "5".to_string(),
            },
            Test {
                expected_type: TokenType::Lt,
                expected_literal: "<".to_string(),
            },
            Test {
                expected_type: TokenType::Int,
                expected_literal: "10".to_string(),
            },
            Test {
                expected_type: TokenType::RParen,
                expected_literal: ")".to_string(),
            },
            Test {
                expected_type: TokenType::LBrace,
                expected_literal: "{".to_string(),
            },
            Test {
                expected_type: TokenType::Return,
                expected_literal: "return".to_string(),
            },
            Test {
                expected_type: TokenType::True,
                expected_literal: "true".to_string(),
            },
            Test {
                expected_type: TokenType::Semicolon,
                expected_literal: ";".to_string(),
            },
            Test {
                expected_type: TokenType::RBrace,
                expected_literal: "}".to_string(),
            },
            Test {
                expected_type: TokenType::Else,
                expected_literal: "else".to_string(),
            },
            Test {
                expected_type: TokenType::LBrace,
                expected_literal: "{".to_string(),
            },
            Test {
                expected_type: TokenType::Return,
                expected_literal: "return".to_string(),
            },
            Test {
                expected_type: TokenType::False,
                expected_literal: "false".to_string(),
            },
            Test {
                expected_type: TokenType::Semicolon,
                expected_literal: ";".to_string(),
            },
            Test {
                expected_type: TokenType::RBrace,
                expected_literal: "}".to_string(),
            },
            Test {
                expected_type: TokenType::Int,
                expected_literal: "10".to_string(),
            },
            Test {
                expected_type: TokenType::Eq,
                expected_literal: "==".to_string(),
            },
            Test {
                expected_type: TokenType::Int,
                expected_literal: "10".to_string(),
            },
            Test {
                expected_type: TokenType::Semicolon,
                expected_literal: ";".to_string(),
            },
            Test {
                expected_type: TokenType::Int,
                expected_literal: "10".to_string(),
            },
            Test {
                expected_type: TokenType::NotEq,
                expected_literal: "!=".to_string(),
            },
            Test {
                expected_type: TokenType::Int,
                expected_literal: "9".to_string(),
            },
            Test {
                expected_type: TokenType::Semicolon,
                expected_literal: ";".to_string(),
            },
            Test {
                expected_type: TokenType::Eof,
                expected_literal: String::new(),
            },
        ];

        let mut lexer = Lexer::new(input);
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

    #[test]
    fn test_next_token_comprehensive() {
        let input = r#"let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
	return true;
} else {
	return false;
}

10 == 10;
10 != 9;
"foobar"
"foo bar"
[1, 2];
{"foo": "bar"}
"#;

        #[derive(Debug)]
        struct Test {
            expected_type: TokenType,
            expected_literal: String,
        }

        let tests = [
            Test { expected_type: TokenType::Let, expected_literal: "let".to_string() },
            Test { expected_type: TokenType::Ident, expected_literal: "five".to_string() },
            Test { expected_type: TokenType::Assign, expected_literal: "=".to_string() },
            Test { expected_type: TokenType::Int, expected_literal: "5".to_string() },
            Test { expected_type: TokenType::Semicolon, expected_literal: ";".to_string() },
            Test { expected_type: TokenType::Let, expected_literal: "let".to_string() },
            Test { expected_type: TokenType::Ident, expected_literal: "ten".to_string() },
            Test { expected_type: TokenType::Assign, expected_literal: "=".to_string() },
            Test { expected_type: TokenType::Int, expected_literal: "10".to_string() },
            Test { expected_type: TokenType::Semicolon, expected_literal: ";".to_string() },
            Test { expected_type: TokenType::Let, expected_literal: "let".to_string() },
            Test { expected_type: TokenType::Ident, expected_literal: "add".to_string() },
            Test { expected_type: TokenType::Assign, expected_literal: "=".to_string() },
            Test { expected_type: TokenType::Function, expected_literal: "fn".to_string() },
            Test { expected_type: TokenType::LParen, expected_literal: "(".to_string() },
            Test { expected_type: TokenType::Ident, expected_literal: "x".to_string() },
            Test { expected_type: TokenType::Comma, expected_literal: ",".to_string() },
            Test { expected_type: TokenType::Ident, expected_literal: "y".to_string() },
            Test { expected_type: TokenType::RParen, expected_literal: ")".to_string() },
            Test { expected_type: TokenType::LBrace, expected_literal: "{".to_string() },
            Test { expected_type: TokenType::Ident, expected_literal: "x".to_string() },
            Test { expected_type: TokenType::Plus, expected_literal: "+".to_string() },
            Test { expected_type: TokenType::Ident, expected_literal: "y".to_string() },
            Test { expected_type: TokenType::Semicolon, expected_literal: ";".to_string() },
            Test { expected_type: TokenType::RBrace, expected_literal: "}".to_string() },
            Test { expected_type: TokenType::Semicolon, expected_literal: ";".to_string() },
            Test { expected_type: TokenType::Let, expected_literal: "let".to_string() },
            Test { expected_type: TokenType::Ident, expected_literal: "result".to_string() },
            Test { expected_type: TokenType::Assign, expected_literal: "=".to_string() },
            Test { expected_type: TokenType::Ident, expected_literal: "add".to_string() },
            Test { expected_type: TokenType::LParen, expected_literal: "(".to_string() },
            Test { expected_type: TokenType::Ident, expected_literal: "five".to_string() },
            Test { expected_type: TokenType::Comma, expected_literal: ",".to_string() },
            Test { expected_type: TokenType::Ident, expected_literal: "ten".to_string() },
            Test { expected_type: TokenType::RParen, expected_literal: ")".to_string() },
            Test { expected_type: TokenType::Semicolon, expected_literal: ";".to_string() },
            Test { expected_type: TokenType::Bang, expected_literal: "!".to_string() },
            Test { expected_type: TokenType::Minus, expected_literal: "-".to_string() },
            Test { expected_type: TokenType::Slash, expected_literal: "/".to_string() },
            Test { expected_type: TokenType::Asterisk, expected_literal: "*".to_string() },
            Test { expected_type: TokenType::Int, expected_literal: "5".to_string() },
            Test { expected_type: TokenType::Semicolon, expected_literal: ";".to_string() },
            Test { expected_type: TokenType::Int, expected_literal: "5".to_string() },
            Test { expected_type: TokenType::Lt, expected_literal: "<".to_string() },
            Test { expected_type: TokenType::Int, expected_literal: "10".to_string() },
            Test { expected_type: TokenType::Gt, expected_literal: ">".to_string() },
            Test { expected_type: TokenType::Int, expected_literal: "5".to_string() },
            Test { expected_type: TokenType::Semicolon, expected_literal: ";".to_string() },
            Test { expected_type: TokenType::If, expected_literal: "if".to_string() },
            Test { expected_type: TokenType::LParen, expected_literal: "(".to_string() },
            Test { expected_type: TokenType::Int, expected_literal: "5".to_string() },
            Test { expected_type: TokenType::Lt, expected_literal: "<".to_string() },
            Test { expected_type: TokenType::Int, expected_literal: "10".to_string() },
            Test { expected_type: TokenType::RParen, expected_literal: ")".to_string() },
            Test { expected_type: TokenType::LBrace, expected_literal: "{".to_string() },
            Test { expected_type: TokenType::Return, expected_literal: "return".to_string() },
            Test { expected_type: TokenType::True, expected_literal: "true".to_string() },
            Test { expected_type: TokenType::Semicolon, expected_literal: ";".to_string() },
            Test { expected_type: TokenType::RBrace, expected_literal: "}".to_string() },
            Test { expected_type: TokenType::Else, expected_literal: "else".to_string() },
            Test { expected_type: TokenType::LBrace, expected_literal: "{".to_string() },
            Test { expected_type: TokenType::Return, expected_literal: "return".to_string() },
            Test { expected_type: TokenType::False, expected_literal: "false".to_string() },
            Test { expected_type: TokenType::Semicolon, expected_literal: ";".to_string() },
            Test { expected_type: TokenType::RBrace, expected_literal: "}".to_string() },
            Test { expected_type: TokenType::Int, expected_literal: "10".to_string() },
            Test { expected_type: TokenType::Eq, expected_literal: "==".to_string() },
            Test { expected_type: TokenType::Int, expected_literal: "10".to_string() },
            Test { expected_type: TokenType::Semicolon, expected_literal: ";".to_string() },
            Test { expected_type: TokenType::Int, expected_literal: "10".to_string() },
            Test { expected_type: TokenType::NotEq, expected_literal: "!=".to_string() },
            Test { expected_type: TokenType::Int, expected_literal: "9".to_string() },
            Test { expected_type: TokenType::Semicolon, expected_literal: ";".to_string() },
            Test { expected_type: TokenType::String, expected_literal: "foobar".to_string() },
            Test { expected_type: TokenType::String, expected_literal: "foo bar".to_string() },
            Test { expected_type: TokenType::LBracket, expected_literal: "[".to_string() },
            Test { expected_type: TokenType::Int, expected_literal: "1".to_string() },
            Test { expected_type: TokenType::Comma, expected_literal: ",".to_string() },
            Test { expected_type: TokenType::Int, expected_literal: "2".to_string() },
            Test { expected_type: TokenType::RBracket, expected_literal: "]".to_string() },
            Test { expected_type: TokenType::Semicolon, expected_literal: ";".to_string() },
            Test { expected_type: TokenType::LBrace, expected_literal: "{".to_string() },
            Test { expected_type: TokenType::String, expected_literal: "foo".to_string() },
            Test { expected_type: TokenType::Colon, expected_literal: ":".to_string() },
            Test { expected_type: TokenType::String, expected_literal: "bar".to_string() },
            Test { expected_type: TokenType::RBrace, expected_literal: "}".to_string() },
            Test { expected_type: TokenType::Eof, expected_literal: "".to_string() },
        ];

        let mut lexer = Lexer::new(input);

        for (i, test) in tests.iter().enumerate() {
            let token = lexer.next_token();

            assert_eq!(
                token.token_type, test.expected_type,
                "tests[{}] - token type wrong. expected={:?}, got={:?}",
                i, test.expected_type, token.token_type
            );

            assert_eq!(
                token.literal, test.expected_literal,
                "tests[{}] - literal wrong. expected={:?}, got={:?}",
                i, test.expected_literal, token.literal
            );
        }
    }
}
