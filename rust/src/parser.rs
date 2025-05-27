use std::collections::HashMap;

use crate::ast;
use crate::{lexer, token};

#[derive(Clone)]
enum PrefixParseAction {
    Identifier,
    IntegerLiteral,
    PrefixExpression,
}

#[derive(Clone)]
enum InfixParseAction {
    InfixExpression,
}

struct Parser<'a> {
    l: lexer::Lexer<'a>,
    errors: Vec<String>,

    curr_token: token::Token,
    peek_token: token::Token,

    prefix_parse_actions: HashMap<token::TokenType, PrefixParseAction>,
    infix_parse_actions: HashMap<token::TokenType, InfixParseAction>,
}

impl<'a> Parser<'a> {
    fn new(l: lexer::Lexer<'a>) -> Parser<'a> {
        let prefix_parse_actions = HashMap::new();
        let infix_parse_actions = HashMap::new();

        let mut p = Parser {
            l,
            curr_token: token::Token::eof(),
            peek_token: token::Token::eof(),
            errors: vec![],
            prefix_parse_actions,
            infix_parse_actions,
        };

        p.next_token();
        p.next_token();

        p.register_expression_parser_actions();

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
            _ => self.parse_expression_statement(),
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

    fn parse_expression_statement(&mut self) -> Option<ast::Statement> {
        let stmt = ast::ExpressionStatement {
            expression: self.parse_expression(Precedence::Lowest),
            token: self.curr_token.clone(),
        };

        if self.peek_token_is(&token::TokenType::Semicolon) {
            self.next_token();
        }

        Some(ast::Statement::Expression(stmt))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<ast::Expression> {
        let parse_action = self
            .prefix_parse_actions
            .get(&self.curr_token.token_type.clone());

        let mut left = if let Some(action) = parse_action {
            match action {
                PrefixParseAction::Identifier => self.parse_identifier(),
                PrefixParseAction::IntegerLiteral => self.parse_integer_literal(),
                PrefixParseAction::PrefixExpression => self.parse_prefix_expression(),
            }
        } else {
            self.no_prefix_parse_fn_error(&self.curr_token.token_type.clone());
            return None;
        };

        while !self.peek_token_is(&token::TokenType::Semicolon)
            && precedence < self.peek_precedence()
        {
            let infix_action = self.infix_parse_actions.get(&self.peek_token.token_type);

            if let Some(InfixParseAction::InfixExpression) = infix_action {
                self.next_token();
                left = Some(self.parse_infix_expression(left?));
            } else {
                return left;
            }
        }

        left
    }

    fn parse_identifier(&mut self) -> Option<ast::Expression> {
        Some(ast::Expression::Identifier(ast::Identifier {
            token: self.curr_token.clone(),
            value: self.curr_token.literal.clone(),
        }))
    }

    fn parse_integer_literal(&mut self) -> Option<ast::Expression> {
        let value = self.curr_token.literal.parse::<i64>().ok()?;
        Some(ast::Expression::IntegerLiteral(value))
    }

    fn parse_prefix_expression(&mut self) -> Option<ast::Expression> {
        let token = self.curr_token.clone();

        self.next_token();

        Some(ast::Expression::PrefixExpression {
            operator: token.literal,
            right: Box::new(self.parse_expression(Precedence::Prefix)?),
        })
    }

    fn parse_infix_expression(&mut self, left: ast::Expression) -> ast::Expression {
        let token = self.curr_token.clone();
        let precedence = self.curr_precedence();

        self.next_token();

        ast::Expression::InfixExpression {
            left: Box::new(left),
            operator: token.literal,
            right: Box::new(
                self.parse_expression(precedence)
                    .unwrap_or(ast::Expression::IntegerLiteral(0)),
            ),
        }
    }

    fn register_expression_parser_actions(&mut self) {
        self.prefix_parse_actions
            .insert(token::TokenType::Ident, PrefixParseAction::Identifier);
        self.prefix_parse_actions
            .insert(token::TokenType::Int, PrefixParseAction::IntegerLiteral);
        self.prefix_parse_actions
            .insert(token::TokenType::Minus, PrefixParseAction::PrefixExpression);
        self.prefix_parse_actions
            .insert(token::TokenType::Plus, PrefixParseAction::PrefixExpression);
        self.prefix_parse_actions
            .insert(token::TokenType::Bang, PrefixParseAction::PrefixExpression);

        // Register infix operators
        self.infix_parse_actions
            .insert(token::TokenType::Plus, InfixParseAction::InfixExpression);
        self.infix_parse_actions
            .insert(token::TokenType::Minus, InfixParseAction::InfixExpression);
        self.infix_parse_actions
            .insert(token::TokenType::Slash, InfixParseAction::InfixExpression);
        self.infix_parse_actions
            .insert(token::TokenType::Asterisk, InfixParseAction::InfixExpression);
        self.infix_parse_actions
            .insert(token::TokenType::Eq, InfixParseAction::InfixExpression);
        self.infix_parse_actions
            .insert(token::TokenType::NotEq, InfixParseAction::InfixExpression);
        self.infix_parse_actions
            .insert(token::TokenType::Lt, InfixParseAction::InfixExpression);
        self.infix_parse_actions
            .insert(token::TokenType::Gt, InfixParseAction::InfixExpression);
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

    fn no_prefix_parse_fn_error(&mut self, t: &token::TokenType) {
        let msg = format!("no prefix parse function for {:?} found", t);
        self.errors.push(msg);
    }

    fn peek_precedence(&self) -> Precedence {
        (&self.peek_token.token_type).into()
    }

    fn curr_precedence(&self) -> Precedence {
        (&self.curr_token.token_type).into()
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::Lexer;
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
        if let ast::Statement::Let(let_statement) = stmt {
            assert_eq!(
                let_statement.name, expected_ident,
                "let statement name not={}, expected={}",
                let_statement.name, expected_ident
            );
            assert_eq!(let_statement.token.token_type, TokenType::Let);
        } else {
            panic!("expected let statement")
        }
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar";
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);

        assert_eq!(
            program.statements.len(),
            1,
            "program missing statements. got={}",
            program.statements.len()
        );

        if let ast::Statement::Expression(ast::ExpressionStatement {
            expression: Some(ast::Expression::Identifier(ident)),
            ..
        }) = program.statements.get(0).unwrap()
        {
            assert_eq!(ident.value, "foobar");
            assert_eq!(ident.token.literal, "foobar");
        } else {
            panic!("expected expression statement");
        }
    }

    #[test]
    fn test_integer_literal() {
        let input = "5;";

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);

        assert_eq!(
            program.statements.len(),
            1,
            "program has {} statements. expected 1",
            program.statements.len()
        );

        if let ast::Statement::Expression(ast::ExpressionStatement {
            expression: Some(ast::Expression::IntegerLiteral(val)),
            ..
        }) = program.statements.get(0).unwrap()
        {
            assert_eq!(*val, 5, "expected value of 5. got={}", val);
        } else {
            panic!("expected expression statement");
        }
    }

    #[test]
    fn test_parsing_prefix_expressions() {
        let prefix_tests = vec![("!5;", "!", 5), ("-15;", "-", 15)];

        for (input, operator, integer) in prefix_tests {
            let l = Lexer::new(input);
            let mut p = Parser::new(l);
            let program = p.parse_program();
            check_parser_errors(&p);

            assert_eq!(
                program.statements.len(),
                1,
                "program has {} statements. expected 1",
                program.statements.len()
            );

            if let ast::Statement::Expression(ast::ExpressionStatement {
                expression:
                    Some(ast::Expression::PrefixExpression {
                        operator: op,
                        right,
                    }),
                ..
            }) = program.statements.get(0).unwrap()
            {
                assert_eq!(op, operator);
                assert_integer_literal(*right.clone(), integer);
            } else {
                panic!("expected expression statement");
            }
        }
    }

    fn assert_integer_literal(il: ast::Expression, value: i64) -> bool {
        if let ast::Expression::IntegerLiteral(val) = il {
            val == value
        } else {
            false
        }
    }

    #[test]
    fn test_parsing_infix_expressions() {
        let infix_tests = vec![
            ("5 + 5;", 5, "+", 5),
            ("5 - 5;", 5, "-", 5),
            ("5 * 5;", 5, "*", 5),
            ("5 / 5;", 5, "/", 5),
            ("5 > 5;", 5, ">", 5),
            ("5 < 5;", 5, "<", 5),
            ("5 == 5;", 5, "==", 5),
            ("5 != 5;", 5, "!=", 5),
        ];

        for (input, left, operator, right) in infix_tests {
            let l = Lexer::new(input);
            let mut p = Parser::new(l);
            let program = p.parse_program();
            check_parser_errors(&p);

            assert_eq!(
                program.statements.len(),
                1,
                "program has {} statements. expected 1",
                program.statements.len()
            );

            if let Some(ast::Statement::Expression(ast::ExpressionStatement {
                expression:
                    Some(ast::Expression::InfixExpression {
                        left: got_left,
                        operator: got_op,
                        right: got_right,
                    }),
                ..
            })) = program.statements.get(0)
            {
                assert_integer_literal(*got_left.clone(), left);
                assert_eq!(got_op, operator);
                assert_integer_literal(*got_right.clone(), right);
            } else {
                panic!("expected expression statement");
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

impl From<&token::TokenType> for Precedence {
    fn from(token_type: &token::TokenType) -> Self {
        match token_type {
            token::TokenType::Eq => Precedence::Equals,
            token::TokenType::NotEq => Precedence::Equals,
            token::TokenType::Lt => Precedence::LessGreater,
            token::TokenType::Gt => Precedence::LessGreater,
            token::TokenType::Plus => Precedence::Sum,
            token::TokenType::Minus => Precedence::Sum,
            token::TokenType::Slash => Precedence::Product,
            token::TokenType::Asterisk => Precedence::Product,
            _ => Precedence::Lowest,
        }
    }
}
