use crate::lexer;
use crate::token;
use std::io::{self, Write};

pub fn start() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("{PROMPT}");
        let _ = stdout.flush();
        let mut buffer = String::new();
        let input = stdin.read_line(&mut buffer);
        if input.is_ok() {
            if buffer.trim().is_empty() {
                continue;
            } else {
                let mut lexer = lexer::Lexer::new(&buffer);
                for token in std::iter::from_fn(|| {
                    let t = lexer.next_token();
                    if t.token_type == token::TokenType::Eof {
                        None
                    } else {
                        Some(t)
                    }
                }) {
                    println!("{:?}", token);
                }
            }
        } else {
            println!("> failed to read stdin")
        }
    }
}

const PROMPT: &str = ">> ";
