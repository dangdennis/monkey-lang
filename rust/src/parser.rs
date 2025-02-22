use crate::ast;
use crate::{lexer, token};

struct Parser<'a> {
    l: lexer::Lexer<'a>,
    curr_token: token::Token,
    peek_token: token::Token,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    fn new(l: lexer::Lexer<'a>) -> Parser<'a> {
        let mut p = Parser {
            l,
            curr_token: token::Token::eof(),
            peek_token: token::Token::eof(),
            errors: vec![],
        };

        p.next_token();
        p.next_token();

        p
    }

    fn next_token(&mut self) {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    fn parse_program(&mut self) -> ast::Program {
        let mut p = ast::Program { statements: vec![] };

        while self.curr_token.token_type != token::TokenType::Eof {
            let statement = self.parse_statement();

            if let Some(statement) = statement {
                p.statements.push(statement);
            }

            self.next_token();
        }

        p
    }

    fn parse_statement(&mut self) -> Option<ast::Statement> {
        match self.curr_token.token_type {
            token::TokenType::Let => self.parse_let_statement(),
            token::TokenType::Return => self.parse_return_statement(),
            _ => unimplemented!("missing parser"),
        }
    }

    fn parse_let_statement(&mut self) -> Option<ast::Statement> {
        let token = self.curr_token.clone();

        if !self.expect_peek(&token::TokenType::Ident) {
            return None;
        }

        let name = self.curr_token.literal.clone();

        if !self.expect_peek(&token::TokenType::Assign) {
            return None;
        }

        while !self.curr_token_is(&token::TokenType::Semicolon) {
            self.next_token();
        }

        Some(ast::Statement::Let(ast::LetStatement {
            token,
            name,
            value: None,
        }))
    }

    fn parse_return_statement(&mut self) -> Option<ast::Statement> {
        let token = self.curr_token.clone();

        while !self.curr_token_is(&token::TokenType::Semicolon) {
            self.next_token();
        }

        Some(ast::Statement::Return(ast::ReturnStatement {
            token,
            return_value: None,
        }))
    }

    fn curr_token_is(&self, t: &token::TokenType) -> bool {
        self.curr_token.token_type == *t
    }

    fn peek_token_is(&self, t: &token::TokenType) -> bool {
        self.peek_token.token_type == *t
    }

    fn expect_peek(&mut self, t: &token::TokenType) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            true
        } else {
            self.peek_error(t);
            false
        }
    }

    fn errors(&self) -> &Vec<String> {
        &self.errors
    }

    fn peek_error(&mut self, t: &token::TokenType) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            t, self.peek_token.token_type
        );
        self.errors.push(msg);
    }
}

#[cfg(test)]
mod test {
    use crate::parser::Parser;
    use crate::token::TokenType;
    use crate::{ast, lexer};

    #[test]
    fn test_let_statements() {
        let input = r#"
          let x = 5;
          let y = 10;
          let foobar = 838383;
          "#;

        let l = lexer::Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();

        check_parser_errors(&p);

        assert_eq!(
            program.statements.len(),
            3,
            "program.statements does not contain 3 statements. got={}",
            program.statements.len()
        );

        let expected_identifiers = vec!["x", "y", "foobar"];

        for (i, expected_ident) in expected_identifiers.iter().enumerate() {
            assert_let_statement(program.statements.get(i).unwrap(), expected_ident);
        }
    }

    #[test]
    fn test_return_statements() {
        let input = r#"
          return 5;
          return 10;
          return 993322;
          "#;

        let l = lexer::Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();

        check_parser_errors(&p);

        assert_eq!(
            program.statements.len(),
            3,
            "program.statements does not contain 3 statements. got={}",
            program.statements.len()
        );

        for stmt in program.statements.iter() {
            if let ast::Statement::Return(return_stmt) = stmt {
                assert_eq!(return_stmt.token.token_type, TokenType::Return);
            } else {
                panic!("expected return statement");
            }
        }
    }

    fn check_parser_errors(p: &Parser) {
        let errors = p.errors();
        if errors.len() == 0 {
            return;
        }

        for error in errors {
            eprintln!("parser error: {}", error);
        }

        assert_eq!(errors.len(), 0, "parser has {} errors", errors.len());
    }

    fn assert_let_statement(stmt: &ast::Statement, expected_ident: &str) {
        match stmt {
            ast::Statement::Let(let_statement) => {
                assert_eq!(
                    let_statement.name, expected_ident,
                    "let statement name not={}, expected={}",
                    let_statement.name, expected_ident
                );
                assert_eq!(let_statement.token.token_type, TokenType::Let);
            }
            _ => panic!("expected let statement"),
        }
    }
}
