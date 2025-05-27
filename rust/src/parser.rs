use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::ast;
use crate::{lexer, token};

// Parser tracing module
pub mod tracing {
    use super::*;

    static TRACE_LEVEL: AtomicUsize = AtomicUsize::new(0);
    const TRACE_IDENT_PLACEHOLDER: &str = "\t";

    fn ident_level() -> String {
        let level = TRACE_LEVEL.load(Ordering::Relaxed);
        if level > 0 {
            TRACE_IDENT_PLACEHOLDER.repeat(level - 1)
        } else {
            String::new()
        }
    }

    fn trace_print(fs: &str) {
        println!("{}{}", ident_level(), fs);
    }

    fn inc_ident() {
        TRACE_LEVEL.fetch_add(1, Ordering::Relaxed);
    }

    fn dec_ident() {
        TRACE_LEVEL.fetch_sub(1, Ordering::Relaxed);
    }

    pub fn trace(msg: &str) -> String {
        inc_ident();
        trace_print(&format!("BEGIN {}", msg));
        msg.to_string()
    }

    pub fn untrace(msg: &str) {
        trace_print(&format!("END {}", msg));
        dec_ident();
    }
}

#[derive(Clone)]
enum PrefixParseAction {
    Identifier,
    IntegerLiteral,
    StringLiteral,
    Boolean,
    PrefixExpression,
    GroupedExpression,
    IfExpression,
    FunctionLiteral,
    ArrayLiteral,
    HashLiteral,
}

#[derive(Clone)]
enum InfixParseAction {
    InfixExpression,
    CallExpression,
    IndexExpression,
}

pub struct Parser<'a> {
    l: lexer::Lexer<'a>,
    errors: Vec<String>,
    curr_token: token::Token,
    peek_token: token::Token,
    prefix_parse_actions: HashMap<token::TokenType, PrefixParseAction>,
    infix_parse_actions: HashMap<token::TokenType, InfixParseAction>,
}

impl<'a> Parser<'a> {
    pub fn new(l: lexer::Lexer<'a>) -> Parser<'a> {
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

    pub fn parse_program(&mut self) -> ast::Program {
        let _trace = tracing::trace("parseProgram");
        let mut program = ast::Program { statements: vec![] };

        while self.curr_token.token_type != token::TokenType::Eof {
            if let Some(statement) = self.parse_statement() {
                program.statements.push(statement);
            }
            self.next_token();
        }

        tracing::untrace("parseProgram");
        program
    }

    fn parse_statement(&mut self) -> Option<ast::Statement> {
        let _trace = tracing::trace("parseStatement");
        let result = match self.curr_token.token_type {
            token::TokenType::Let => self.parse_let_statement(),
            token::TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        };
        tracing::untrace("parseStatement");
        result
    }

    fn parse_let_statement(&mut self) -> Option<ast::Statement> {
        let _trace = tracing::trace("parseLetStatement");
        let token = self.curr_token.clone();

        if !self.expect_peek(&token::TokenType::Ident) {
            tracing::untrace("parseLetStatement");
            return None;
        }

        let name = ast::Identifier {
            token: self.curr_token.clone(),
            value: self.curr_token.literal.clone(),
        };

        if !self.expect_peek(&token::TokenType::Assign) {
            tracing::untrace("parseLetStatement");
            return None;
        }

        self.next_token();

        let value = self.parse_expression(Precedence::Lowest);

        if self.peek_token_is(&token::TokenType::Semicolon) {
            self.next_token();
        }

        let result = Some(ast::Statement::Let(ast::LetStatement {
            token,
            name,
            value,
        }));

        tracing::untrace("parseLetStatement");
        result
    }

    fn parse_return_statement(&mut self) -> Option<ast::Statement> {
        let _trace = tracing::trace("parseReturnStatement");
        let token = self.curr_token.clone();

        self.next_token();

        let return_value = self.parse_expression(Precedence::Lowest);

        if self.peek_token_is(&token::TokenType::Semicolon) {
            self.next_token();
        }

        let result = Some(ast::Statement::Return(ast::ReturnStatement {
            token,
            return_value,
        }));

        tracing::untrace("parseReturnStatement");
        result
    }

    fn parse_expression_statement(&mut self) -> Option<ast::Statement> {
        let _trace = tracing::trace("parseExpressionStatement");
        let stmt = ast::ExpressionStatement {
            expression: self.parse_expression(Precedence::Lowest),
            token: self.curr_token.clone(),
        };

        if self.peek_token_is(&token::TokenType::Semicolon) {
            self.next_token();
        }

        let result = Some(ast::Statement::Expression(stmt));
        tracing::untrace("parseExpressionStatement");
        result
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<ast::Expression> {
        let _trace = tracing::trace("parseExpression");
        let token_type = self.curr_token.token_type.clone();
        let parse_action = self.prefix_parse_actions.get(&token_type).cloned();

        let mut left = if let Some(action) = parse_action {
            match action {
                PrefixParseAction::Identifier => self.parse_identifier(),
                PrefixParseAction::IntegerLiteral => self.parse_integer_literal(),
                PrefixParseAction::StringLiteral => self.parse_string_literal(),
                PrefixParseAction::Boolean => self.parse_boolean(),
                PrefixParseAction::PrefixExpression => self.parse_prefix_expression(),
                PrefixParseAction::GroupedExpression => self.parse_grouped_expression(),
                PrefixParseAction::IfExpression => self.parse_if_expression(),
                PrefixParseAction::FunctionLiteral => self.parse_function_literal(),
                PrefixParseAction::ArrayLiteral => self.parse_array_literal(),
                PrefixParseAction::HashLiteral => self.parse_hash_literal(),
            }
        } else {
            self.no_prefix_parse_fn_error(&token_type);
            tracing::untrace("parseExpression");
            return None;
        };

        while !self.peek_token_is(&token::TokenType::Semicolon)
            && precedence < self.peek_precedence()
        {
            let peek_token_type = self.peek_token.token_type.clone();
            let infix_action = self.infix_parse_actions.get(&peek_token_type).cloned();

            if let Some(action) = infix_action {
                self.next_token();
                left = match action {
                    InfixParseAction::InfixExpression => Some(self.parse_infix_expression(left?)),
                    InfixParseAction::CallExpression => self.parse_call_expression(left?),
                    InfixParseAction::IndexExpression => self.parse_index_expression(left?),
                };
            } else {
                tracing::untrace("parseExpression");
                return left;
            }
        }

        tracing::untrace("parseExpression");
        left
    }

    fn parse_identifier(&mut self) -> Option<ast::Expression> {
        let _trace = tracing::trace("parseIdentifier");
        let result = Some(ast::Expression::Identifier(ast::Identifier {
            token: self.curr_token.clone(),
            value: self.curr_token.literal.clone(),
        }));
        tracing::untrace("parseIdentifier");
        result
    }

    fn parse_integer_literal(&mut self) -> Option<ast::Expression> {
        let _trace = tracing::trace("parseIntegerLiteral");
        let value = match self.curr_token.literal.parse::<i64>() {
            Ok(v) => v,
            Err(_) => {
                let msg = format!("could not parse {} as integer", self.curr_token.literal);
                self.errors.push(msg);
                tracing::untrace("parseIntegerLiteral");
                return None;
            }
        };

        let result = Some(ast::Expression::IntegerLiteral(ast::IntegerLiteral {
            token: self.curr_token.clone(),
            value,
        }));
        tracing::untrace("parseIntegerLiteral");
        result
    }

    fn parse_string_literal(&mut self) -> Option<ast::Expression> {
        let _trace = tracing::trace("parseStringLiteral");
        let result = Some(ast::Expression::StringLiteral(ast::StringLiteral {
            token: self.curr_token.clone(),
            value: self.curr_token.literal.clone(),
        }));
        tracing::untrace("parseStringLiteral");
        result
    }

    fn parse_boolean(&mut self) -> Option<ast::Expression> {
        let _trace = tracing::trace("parseBoolean");
        let result = Some(ast::Expression::Boolean(ast::Boolean {
            token: self.curr_token.clone(),
            value: self.curr_token_is(&token::TokenType::True),
        }));
        tracing::untrace("parseBoolean");
        result
    }

    fn parse_prefix_expression(&mut self) -> Option<ast::Expression> {
        let _trace = tracing::trace("parsePrefixExpression");
        let token = self.curr_token.clone();

        self.next_token();

        let right = self.parse_expression(Precedence::Prefix)?;

        let result = Some(ast::Expression::PrefixExpression {
            token: token.clone(),
            operator: token.literal,
            right: Box::new(right),
        });

        tracing::untrace("parsePrefixExpression");
        result
    }

    fn parse_infix_expression(&mut self, left: ast::Expression) -> ast::Expression {
        let _trace = tracing::trace("parseInfixExpression");
        let token = self.curr_token.clone();
        let precedence = self.curr_precedence();

        self.next_token();

        let right = self
            .parse_expression(precedence)
            .unwrap_or(ast::Expression::IntegerLiteral(ast::IntegerLiteral {
                token: crate::token::Token::new(crate::token::TokenType::Int, "0"),
                value: 0,
            }));

        let result = ast::Expression::InfixExpression {
            token: token.clone(),
            left: Box::new(left),
            operator: token.literal,
            right: Box::new(right),
        };

        tracing::untrace("parseInfixExpression");
        result
    }

    fn parse_grouped_expression(&mut self) -> Option<ast::Expression> {
        let _trace = tracing::trace("parseGroupedExpression");
        self.next_token();

        let exp = self.parse_expression(Precedence::Lowest);

        if !self.expect_peek(&token::TokenType::RParen) {
            tracing::untrace("parseGroupedExpression");
            return None;
        }

        tracing::untrace("parseGroupedExpression");
        exp
    }

    fn parse_if_expression(&mut self) -> Option<ast::Expression> {
        let _trace = tracing::trace("parseIfExpression");
        let token = self.curr_token.clone();

        if !self.expect_peek(&token::TokenType::LParen) {
            tracing::untrace("parseIfExpression");
            return None;
        }

        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest)?;

        if !self.expect_peek(&token::TokenType::RParen) {
            tracing::untrace("parseIfExpression");
            return None;
        }

        if !self.expect_peek(&token::TokenType::LBrace) {
            tracing::untrace("parseIfExpression");
            return None;
        }

        let consequence = self.parse_block_statement();

        let alternative = if self.peek_token_is(&token::TokenType::Else) {
            self.next_token();

            if !self.expect_peek(&token::TokenType::LBrace) {
                tracing::untrace("parseIfExpression");
                return None;
            }

            Some(self.parse_block_statement())
        } else {
            None
        };

        let result = Some(ast::Expression::IfExpression(ast::IfExpression {
            token,
            condition: Box::new(condition),
            consequence,
            alternative,
        }));

        tracing::untrace("parseIfExpression");
        result
    }

    fn parse_block_statement(&mut self) -> ast::BlockStatement {
        let _trace = tracing::trace("parseBlockStatement");
        let mut block = ast::BlockStatement {
            token: self.curr_token.clone(),
            statements: vec![],
        };

        self.next_token();

        while !self.curr_token_is(&token::TokenType::RBrace)
            && !self.curr_token_is(&token::TokenType::Eof)
        {
            if let Some(stmt) = self.parse_statement() {
                block.statements.push(stmt);
            }
            self.next_token();
        }

        tracing::untrace("parseBlockStatement");
        block
    }

    fn parse_function_literal(&mut self) -> Option<ast::Expression> {
        let _trace = tracing::trace("parseFunctionLiteral");
        let token = self.curr_token.clone();

        if !self.expect_peek(&token::TokenType::LParen) {
            tracing::untrace("parseFunctionLiteral");
            return None;
        }

        let parameters = self.parse_function_parameters()?;

        if !self.expect_peek(&token::TokenType::LBrace) {
            tracing::untrace("parseFunctionLiteral");
            return None;
        }

        let body = self.parse_block_statement();

        let result = Some(ast::Expression::FunctionLiteral(ast::FunctionLiteral {
            token,
            parameters,
            body,
        }));

        tracing::untrace("parseFunctionLiteral");
        result
    }

    fn parse_function_parameters(&mut self) -> Option<Vec<ast::Identifier>> {
        let _trace = tracing::trace("parseFunctionParameters");
        let mut identifiers = vec![];

        if self.peek_token_is(&token::TokenType::RParen) {
            self.next_token();
            tracing::untrace("parseFunctionParameters");
            return Some(identifiers);
        }

        self.next_token();

        let ident = ast::Identifier {
            token: self.curr_token.clone(),
            value: self.curr_token.literal.clone(),
        };
        identifiers.push(ident);

        while self.peek_token_is(&token::TokenType::Comma) {
            self.next_token();
            self.next_token();
            let ident = ast::Identifier {
                token: self.curr_token.clone(),
                value: self.curr_token.literal.clone(),
            };
            identifiers.push(ident);
        }

        if !self.expect_peek(&token::TokenType::RParen) {
            tracing::untrace("parseFunctionParameters");
            return None;
        }

        tracing::untrace("parseFunctionParameters");
        Some(identifiers)
    }

    fn parse_call_expression(&mut self, function: ast::Expression) -> Option<ast::Expression> {
        let _trace = tracing::trace("parseCallExpression");
        let token = self.curr_token.clone();
        let arguments = self.parse_expression_list(&token::TokenType::RParen)?;

        let result = Some(ast::Expression::CallExpression(ast::CallExpression {
            token,
            function: Box::new(function),
            arguments,
        }));

        tracing::untrace("parseCallExpression");
        result
    }

    fn parse_expression_list(&mut self, end: &token::TokenType) -> Option<Vec<ast::Expression>> {
        let _trace = tracing::trace("parseExpressionList");
        let mut list = vec![];

        if self.peek_token_is(end) {
            self.next_token();
            tracing::untrace("parseExpressionList");
            return Some(list);
        }

        self.next_token();
        if let Some(expr) = self.parse_expression(Precedence::Lowest) {
            list.push(expr);
        }

        while self.peek_token_is(&token::TokenType::Comma) {
            self.next_token();
            self.next_token();
            if let Some(expr) = self.parse_expression(Precedence::Lowest) {
                list.push(expr);
            }
        }

        if !self.expect_peek(end) {
            tracing::untrace("parseExpressionList");
            return None;
        }

        tracing::untrace("parseExpressionList");
        Some(list)
    }

    fn parse_array_literal(&mut self) -> Option<ast::Expression> {
        let _trace = tracing::trace("parseArrayLiteral");
        let token = self.curr_token.clone();
        let elements = self.parse_expression_list(&token::TokenType::RBracket)?;

        let result = Some(ast::Expression::ArrayLiteral(ast::ArrayLiteral {
            token,
            elements,
        }));

        tracing::untrace("parseArrayLiteral");
        result
    }

    fn parse_index_expression(&mut self, left: ast::Expression) -> Option<ast::Expression> {
        let _trace = tracing::trace("parseIndexExpression");
        let token = self.curr_token.clone();

        self.next_token();
        let index = self.parse_expression(Precedence::Lowest)?;

        if !self.expect_peek(&token::TokenType::RBracket) {
            tracing::untrace("parseIndexExpression");
            return None;
        }

        let result = Some(ast::Expression::IndexExpression(ast::IndexExpression {
            token,
            left: Box::new(left),
            index: Box::new(index),
        }));

        tracing::untrace("parseIndexExpression");
        result
    }

    fn parse_hash_literal(&mut self) -> Option<ast::Expression> {
        let _trace = tracing::trace("parseHashLiteral");
        let mut hash = ast::HashLiteral {
            token: self.curr_token.clone(),
            pairs: HashMap::new(),
        };

        while !self.peek_token_is(&token::TokenType::RBrace) {
            self.next_token();
            let key = self.parse_expression(Precedence::Lowest)?;

            if !self.expect_peek(&token::TokenType::Colon) {
                tracing::untrace("parseHashLiteral");
                return None;
            }

            self.next_token();
            let value = self.parse_expression(Precedence::Lowest)?;

            hash.pairs.insert(key, value);

            if !self.peek_token_is(&token::TokenType::RBrace)
                && !self.expect_peek(&token::TokenType::Comma)
            {
                tracing::untrace("parseHashLiteral");
                return None;
            }
        }

        if !self.expect_peek(&token::TokenType::RBrace) {
            tracing::untrace("parseHashLiteral");
            return None;
        }

        let result = Some(ast::Expression::HashLiteral(hash));
        tracing::untrace("parseHashLiteral");
        result
    }

    fn register_expression_parser_actions(&mut self) {
        // Register prefix parsers
        self.prefix_parse_actions
            .insert(token::TokenType::Ident, PrefixParseAction::Identifier);
        self.prefix_parse_actions
            .insert(token::TokenType::Int, PrefixParseAction::IntegerLiteral);
        self.prefix_parse_actions
            .insert(token::TokenType::String, PrefixParseAction::StringLiteral);
        self.prefix_parse_actions
            .insert(token::TokenType::True, PrefixParseAction::Boolean);
        self.prefix_parse_actions
            .insert(token::TokenType::False, PrefixParseAction::Boolean);
        self.prefix_parse_actions
            .insert(token::TokenType::Bang, PrefixParseAction::PrefixExpression);
        self.prefix_parse_actions
            .insert(token::TokenType::Minus, PrefixParseAction::PrefixExpression);
        self.prefix_parse_actions.insert(
            token::TokenType::LParen,
            PrefixParseAction::GroupedExpression,
        );
        self.prefix_parse_actions
            .insert(token::TokenType::If, PrefixParseAction::IfExpression);
        self.prefix_parse_actions.insert(
            token::TokenType::Function,
            PrefixParseAction::FunctionLiteral,
        );
        self.prefix_parse_actions
            .insert(token::TokenType::LBracket, PrefixParseAction::ArrayLiteral);
        self.prefix_parse_actions
            .insert(token::TokenType::LBrace, PrefixParseAction::HashLiteral);

        // Register infix parsers
        self.infix_parse_actions
            .insert(token::TokenType::Plus, InfixParseAction::InfixExpression);
        self.infix_parse_actions
            .insert(token::TokenType::Minus, InfixParseAction::InfixExpression);
        self.infix_parse_actions
            .insert(token::TokenType::Slash, InfixParseAction::InfixExpression);
        self.infix_parse_actions.insert(
            token::TokenType::Asterisk,
            InfixParseAction::InfixExpression,
        );
        self.infix_parse_actions
            .insert(token::TokenType::Eq, InfixParseAction::InfixExpression);
        self.infix_parse_actions
            .insert(token::TokenType::NotEq, InfixParseAction::InfixExpression);
        self.infix_parse_actions
            .insert(token::TokenType::Lt, InfixParseAction::InfixExpression);
        self.infix_parse_actions
            .insert(token::TokenType::Gt, InfixParseAction::InfixExpression);

        self.infix_parse_actions
            .insert(token::TokenType::LParen, InfixParseAction::CallExpression);
        self.infix_parse_actions.insert(
            token::TokenType::LBracket,
            InfixParseAction::IndexExpression,
        );
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

    pub fn errors(&self) -> &Vec<String> {
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
    Index,
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
            token::TokenType::LParen => Precedence::Call,
            token::TokenType::LBracket => Precedence::Index,
            _ => Precedence::Lowest,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn check_parser_errors(p: &Parser) {
        let errors = p.errors();
        if errors.is_empty() {
            return;
        }

        eprintln!("parser has {} errors", errors.len());
        for error in errors {
            eprintln!("parser error: {}", error);
        }

        panic!("parser errors found");
    }

    fn assert_let_statement(stmt: &ast::Statement, expected_ident: &str) {
        if let ast::Statement::Let(let_stmt) = stmt {
            assert_eq!(let_stmt.token.literal, "let");
            assert_eq!(let_stmt.name.value, expected_ident);
            assert_eq!(let_stmt.name.token.literal, expected_ident);
        } else {
            panic!("statement is not a let statement, got={:?}", stmt);
        }
    }

    fn assert_integer_literal(exp: &ast::Expression, value: i64) {
        if let ast::Expression::IntegerLiteral(int_lit) = exp {
            assert_eq!(int_lit.value, value);
            assert_eq!(int_lit.token.literal, value.to_string());
        } else {
            panic!("expression is not IntegerLiteral, got={:?}", exp);
        }
    }

    fn assert_identifier(exp: &ast::Expression, value: &str) {
        if let ast::Expression::Identifier(ident) = exp {
            assert_eq!(ident.value, value);
            assert_eq!(ident.token.literal, value);
        } else {
            panic!("expression is not Identifier, got={:?}", exp);
        }
    }

    fn assert_boolean_literal(exp: &ast::Expression, value: bool) {
        if let ast::Expression::Boolean(boolean) = exp {
            assert_eq!(boolean.value, value);
            assert_eq!(boolean.token.literal, value.to_string());
        } else {
            panic!("expression is not Boolean, got={:?}", exp);
        }
    }

    fn assert_literal_expression(exp: &ast::Expression, expected: &LiteralValue) {
        match expected {
            LiteralValue::Int(v) => assert_integer_literal(exp, *v),
            LiteralValue::String(v) => assert_identifier(exp, v),
            LiteralValue::Bool(v) => assert_boolean_literal(exp, *v),
        }
    }

    fn assert_infix_expression(
        exp: &ast::Expression,
        left: &LiteralValue,
        operator: &str,
        right: &LiteralValue,
    ) {
        if let ast::Expression::InfixExpression {
            left: left_exp,
            operator: op,
            right: right_exp,
            ..
        } = exp
        {
            assert_literal_expression(left_exp, left);
            assert_eq!(op, operator);
            assert_literal_expression(right_exp, right);
        } else {
            panic!("expression is not InfixExpression, got={:?}", exp);
        }
    }

    #[derive(Debug)]
    enum LiteralValue {
        Int(i64),
        String(String),
        Bool(bool),
    }

    #[test]
    fn test_let_statements() {
        struct TestCase {
            input: &'static str,
            expected_identifier: &'static str,
            expected_value: LiteralValue,
        }

        let tests = vec![
            TestCase {
                input: "let x = 5;",
                expected_identifier: "x",
                expected_value: LiteralValue::Int(5),
            },
            TestCase {
                input: "let y = true;",
                expected_identifier: "y",
                expected_value: LiteralValue::Bool(true),
            },
            TestCase {
                input: "let foobar = y;",
                expected_identifier: "foobar",
                expected_value: LiteralValue::String("y".to_string()),
            },
        ];

        for test in tests {
            let l = Lexer::new(test.input);
            let mut p = Parser::new(l);
            let program = p.parse_program();
            check_parser_errors(&p);

            assert_eq!(
                program.statements.len(),
                1,
                "program.statements does not contain 1 statement. got={}",
                program.statements.len()
            );

            let stmt = &program.statements[0];
            assert_let_statement(stmt, test.expected_identifier);

            if let ast::Statement::Let(let_stmt) = stmt {
                if let Some(ref val) = let_stmt.value {
                    assert_literal_expression(val, &test.expected_value);
                }
            }
        }
    }

    #[test]
    fn test_return_statements() {
        struct TestCase {
            input: &'static str,
            expected_value: LiteralValue,
        }

        let tests = vec![
            TestCase {
                input: "return 5;",
                expected_value: LiteralValue::Int(5),
            },
            TestCase {
                input: "return true;",
                expected_value: LiteralValue::Bool(true),
            },
            TestCase {
                input: "return foobar;",
                expected_value: LiteralValue::String("foobar".to_string()),
            },
        ];

        for test in tests {
            let l = Lexer::new(test.input);
            let mut p = Parser::new(l);
            let program = p.parse_program();
            check_parser_errors(&p);

            assert_eq!(
                program.statements.len(),
                1,
                "program.statements does not contain 1 statement. got={}",
                program.statements.len()
            );

            let stmt = &program.statements[0];
            if let ast::Statement::Return(return_stmt) = stmt {
                if let Some(ref val) = return_stmt.return_value {
                    assert_literal_expression(val, &test.expected_value);
                }
            } else {
                panic!("statement is not return statement, got={:?}", stmt);
            }
        }
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);

        assert_eq!(
            program.statements.len(),
            1,
            "program has not enough statements. got={}",
            program.statements.len()
        );

        if let ast::Statement::Expression(expr_stmt) = &program.statements[0] {
            if let Some(ast::Expression::Identifier(ident)) = &expr_stmt.expression {
                assert_eq!(ident.value, "foobar");
                assert_eq!(ident.token.literal, "foobar");
            } else {
                panic!("expression not Identifier, got={:?}", expr_stmt.expression);
            }
        } else {
            panic!("program.statements[0] is not ExpressionStatement");
        }
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);

        assert_eq!(
            program.statements.len(),
            1,
            "program has not enough statements. got={}",
            program.statements.len()
        );

        if let ast::Statement::Expression(expr_stmt) = &program.statements[0] {
            if let Some(ref expr) = expr_stmt.expression {
                assert_integer_literal(expr, 5);
            } else {
                panic!("expression is None");
            }
        } else {
            panic!("program.statements[0] is not ExpressionStatement");
        }
    }

    #[test]
    fn test_parsing_prefix_expressions() {
        struct PrefixTest {
            input: &'static str,
            operator: &'static str,
            value: LiteralValue,
        }

        let prefix_tests = vec![
            PrefixTest {
                input: "!5;",
                operator: "!",
                value: LiteralValue::Int(5),
            },
            PrefixTest {
                input: "-15;",
                operator: "-",
                value: LiteralValue::Int(15),
            },
            PrefixTest {
                input: "!foobar;",
                operator: "!",
                value: LiteralValue::String("foobar".to_string()),
            },
            PrefixTest {
                input: "!true;",
                operator: "!",
                value: LiteralValue::Bool(true),
            },
            PrefixTest {
                input: "!false;",
                operator: "!",
                value: LiteralValue::Bool(false),
            },
        ];

        for test in prefix_tests {
            let l = Lexer::new(test.input);
            let mut p = Parser::new(l);
            let program = p.parse_program();
            check_parser_errors(&p);

            assert_eq!(
                program.statements.len(),
                1,
                "program.statements does not contain 1 statement. got={}",
                program.statements.len()
            );

            if let ast::Statement::Expression(expr_stmt) = &program.statements[0] {
                if let Some(ast::Expression::PrefixExpression {
                    operator, right, ..
                }) = &expr_stmt.expression
                {
                    assert_eq!(operator, test.operator);
                    assert_literal_expression(right, &test.value);
                } else {
                    panic!(
                        "expression is not PrefixExpression, got={:?}",
                        expr_stmt.expression
                    );
                }
            } else {
                panic!("program.statements[0] is not ExpressionStatement");
            }
        }
    }

    #[test]
    fn test_parsing_infix_expressions() {
        struct InfixTest {
            input: &'static str,
            left_value: LiteralValue,
            operator: &'static str,
            right_value: LiteralValue,
        }

        let infix_tests = vec![
            InfixTest {
                input: "5 + 5;",
                left_value: LiteralValue::Int(5),
                operator: "+",
                right_value: LiteralValue::Int(5),
            },
            InfixTest {
                input: "5 - 5;",
                left_value: LiteralValue::Int(5),
                operator: "-",
                right_value: LiteralValue::Int(5),
            },
            InfixTest {
                input: "5 * 5;",
                left_value: LiteralValue::Int(5),
                operator: "*",
                right_value: LiteralValue::Int(5),
            },
            InfixTest {
                input: "5 / 5;",
                left_value: LiteralValue::Int(5),
                operator: "/",
                right_value: LiteralValue::Int(5),
            },
            InfixTest {
                input: "5 > 5;",
                left_value: LiteralValue::Int(5),
                operator: ">",
                right_value: LiteralValue::Int(5),
            },
            InfixTest {
                input: "5 < 5;",
                left_value: LiteralValue::Int(5),
                operator: "<",
                right_value: LiteralValue::Int(5),
            },
            InfixTest {
                input: "5 == 5;",
                left_value: LiteralValue::Int(5),
                operator: "==",
                right_value: LiteralValue::Int(5),
            },
            InfixTest {
                input: "5 != 5;",
                left_value: LiteralValue::Int(5),
                operator: "!=",
                right_value: LiteralValue::Int(5),
            },
            InfixTest {
                input: "foobar + barfoo;",
                left_value: LiteralValue::String("foobar".to_string()),
                operator: "+",
                right_value: LiteralValue::String("barfoo".to_string()),
            },
            InfixTest {
                input: "foobar - barfoo;",
                left_value: LiteralValue::String("foobar".to_string()),
                operator: "-",
                right_value: LiteralValue::String("barfoo".to_string()),
            },
            InfixTest {
                input: "foobar * barfoo;",
                left_value: LiteralValue::String("foobar".to_string()),
                operator: "*",
                right_value: LiteralValue::String("barfoo".to_string()),
            },
            InfixTest {
                input: "foobar / barfoo;",
                left_value: LiteralValue::String("foobar".to_string()),
                operator: "/",
                right_value: LiteralValue::String("barfoo".to_string()),
            },
            InfixTest {
                input: "foobar > barfoo;",
                left_value: LiteralValue::String("foobar".to_string()),
                operator: ">",
                right_value: LiteralValue::String("barfoo".to_string()),
            },
            InfixTest {
                input: "foobar < barfoo;",
                left_value: LiteralValue::String("foobar".to_string()),
                operator: "<",
                right_value: LiteralValue::String("barfoo".to_string()),
            },
            InfixTest {
                input: "foobar == barfoo;",
                left_value: LiteralValue::String("foobar".to_string()),
                operator: "==",
                right_value: LiteralValue::String("barfoo".to_string()),
            },
            InfixTest {
                input: "foobar != barfoo;",
                left_value: LiteralValue::String("foobar".to_string()),
                operator: "!=",
                right_value: LiteralValue::String("barfoo".to_string()),
            },
            InfixTest {
                input: "true == true",
                left_value: LiteralValue::Bool(true),
                operator: "==",
                right_value: LiteralValue::Bool(true),
            },
            InfixTest {
                input: "true != false",
                left_value: LiteralValue::Bool(true),
                operator: "!=",
                right_value: LiteralValue::Bool(false),
            },
            InfixTest {
                input: "false == false",
                left_value: LiteralValue::Bool(false),
                operator: "==",
                right_value: LiteralValue::Bool(false),
            },
        ];

        for test in infix_tests {
            let l = Lexer::new(test.input);
            let mut p = Parser::new(l);
            let program = p.parse_program();
            check_parser_errors(&p);

            assert_eq!(
                program.statements.len(),
                1,
                "program.statements does not contain 1 statement. got={}",
                program.statements.len()
            );

            if let ast::Statement::Expression(expr_stmt) = &program.statements[0] {
                if let Some(ref expr) = expr_stmt.expression {
                    assert_infix_expression(
                        expr,
                        &test.left_value,
                        test.operator,
                        &test.right_value,
                    );
                } else {
                    panic!("expression is None");
                }
            } else {
                panic!("program.statements[0] is not ExpressionStatement");
            }
        }
    }

    #[test]
    fn test_operator_precedence_parsing() {
        struct Test {
            input: &'static str,
            expected: &'static str,
        }

        let tests = vec![
            Test {
                input: "-a * b",
                expected: "((-a) * b)",
            },
            Test {
                input: "!-a",
                expected: "(!(-a))",
            },
            Test {
                input: "a + b + c",
                expected: "((a + b) + c)",
            },
            Test {
                input: "a + b - c",
                expected: "((a + b) - c)",
            },
            Test {
                input: "a * b * c",
                expected: "((a * b) * c)",
            },
            Test {
                input: "a * b / c",
                expected: "((a * b) / c)",
            },
            Test {
                input: "a + b / c",
                expected: "(a + (b / c))",
            },
            Test {
                input: "a + b * c + d / e - f",
                expected: "(((a + (b * c)) + (d / e)) - f)",
            },
            Test {
                input: "3 + 4; -5 * 5",
                expected: "(3 + 4)((-5) * 5)",
            },
            Test {
                input: "5 > 4 == 3 < 4",
                expected: "((5 > 4) == (3 < 4))",
            },
            Test {
                input: "5 < 4 != 3 > 4",
                expected: "((5 < 4) != (3 > 4))",
            },
            Test {
                input: "3 + 4 * 5 == 3 * 1 + 4 * 5",
                expected: "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            },
            Test {
                input: "true",
                expected: "true",
            },
            Test {
                input: "false",
                expected: "false",
            },
            Test {
                input: "3 > 5 == false",
                expected: "((3 > 5) == false)",
            },
            Test {
                input: "3 < 5 == true",
                expected: "((3 < 5) == true)",
            },
            Test {
                input: "1 + (2 + 3) + 4",
                expected: "((1 + (2 + 3)) + 4)",
            },
            Test {
                input: "(5 + 5) * 2",
                expected: "((5 + 5) * 2)",
            },
            Test {
                input: "2 / (5 + 5)",
                expected: "(2 / (5 + 5))",
            },
            Test {
                input: "(5 + 5) * 2 * (5 + 5)",
                expected: "(((5 + 5) * 2) * (5 + 5))",
            },
            Test {
                input: "-(5 + 5)",
                expected: "(-(5 + 5))",
            },
            Test {
                input: "!(true == true)",
                expected: "(!(true == true))",
            },
            Test {
                input: "a + add(b * c) + d",
                expected: "((a + add((b * c))) + d)",
            },
            Test {
                input: "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
                expected: "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
            },
            Test {
                input: "add(a + b + c * d / f + g)",
                expected: "add((((a + b) + ((c * d) / f)) + g))",
            },
            Test {
                input: "a * [1, 2, 3, 4][b * c] * d",
                expected: "((a * ([1, 2, 3, 4][(b * c)])) * d)",
            },
            Test {
                input: "add(a * b[2], b[1], 2 * [1, 2][1])",
                expected: "add((a * (b[2])), (b[1]), (2 * ([1, 2][1])))",
            },
        ];

        for test in tests {
            let l = Lexer::new(test.input);
            let mut p = Parser::new(l);
            let program = p.parse_program();
            check_parser_errors(&p);

            let actual = program.to_string();
            assert_eq!(actual, test.expected);
        }
    }

    #[test]
    fn test_boolean_expression() {
        let tests = vec![("true;", true), ("false;", false)];

        for (input, expected_boolean) in tests {
            let l = Lexer::new(input);
            let mut p = Parser::new(l);
            let program = p.parse_program();
            check_parser_errors(&p);

            assert_eq!(
                program.statements.len(),
                1,
                "program has not enough statements. got={}",
                program.statements.len()
            );

            if let ast::Statement::Expression(expr_stmt) = &program.statements[0] {
                if let Some(ast::Expression::Boolean(boolean)) = &expr_stmt.expression {
                    assert_eq!(boolean.value, expected_boolean);
                } else {
                    panic!("expression not Boolean, got={:?}", expr_stmt.expression);
                }
            } else {
                panic!("program.statements[0] is not ExpressionStatement");
            }
        }
    }

    #[test]
    fn test_if_expression() {
        let input = "if (x < y) { x }";
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);

        assert_eq!(
            program.statements.len(),
            1,
            "program.statements does not contain 1 statement. got={}",
            program.statements.len()
        );

        if let ast::Statement::Expression(expr_stmt) = &program.statements[0] {
            if let Some(ast::Expression::IfExpression(if_exp)) = &expr_stmt.expression {
                assert_infix_expression(
                    &if_exp.condition,
                    &LiteralValue::String("x".to_string()),
                    "<",
                    &LiteralValue::String("y".to_string()),
                );

                assert_eq!(
                    if_exp.consequence.statements.len(),
                    1,
                    "consequence is not 1 statement. got={}",
                    if_exp.consequence.statements.len()
                );

                if let ast::Statement::Expression(consequence) = &if_exp.consequence.statements[0] {
                    if let Some(ref expr) = consequence.expression {
                        assert_identifier(expr, "x");
                    } else {
                        panic!("consequence expression is None");
                    }
                } else {
                    panic!("consequence.statements[0] is not ExpressionStatement");
                }

                assert!(
                    if_exp.alternative.is_none(),
                    "if_exp.alternative was not None"
                );
            } else {
                panic!(
                    "stmt.expression is not IfExpression, got={:?}",
                    expr_stmt.expression
                );
            }
        } else {
            panic!("program.statements[0] is not ExpressionStatement");
        }
    }

    #[test]
    fn test_if_else_expression() {
        let input = "if (x < y) { x } else { y }";
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);

        assert_eq!(
            program.statements.len(),
            1,
            "program.statements does not contain 1 statement. got={}",
            program.statements.len()
        );

        if let ast::Statement::Expression(expr_stmt) = &program.statements[0] {
            if let Some(ast::Expression::IfExpression(if_exp)) = &expr_stmt.expression {
                assert_infix_expression(
                    &if_exp.condition,
                    &LiteralValue::String("x".to_string()),
                    "<",
                    &LiteralValue::String("y".to_string()),
                );

                assert_eq!(
                    if_exp.consequence.statements.len(),
                    1,
                    "consequence is not 1 statement. got={}",
                    if_exp.consequence.statements.len()
                );

                if let ast::Statement::Expression(consequence) = &if_exp.consequence.statements[0] {
                    if let Some(ref expr) = consequence.expression {
                        assert_identifier(expr, "x");
                    } else {
                        panic!("consequence expression is None");
                    }
                } else {
                    panic!("consequence.statements[0] is not ExpressionStatement");
                }

                if let Some(ref alternative) = if_exp.alternative {
                    assert_eq!(
                        alternative.statements.len(),
                        1,
                        "alternative is not 1 statement. got={}",
                        alternative.statements.len()
                    );

                    if let ast::Statement::Expression(alt_stmt) = &alternative.statements[0] {
                        if let Some(ref expr) = alt_stmt.expression {
                            assert_identifier(expr, "y");
                        } else {
                            panic!("alternative expression is None");
                        }
                    } else {
                        panic!("alternative.statements[0] is not ExpressionStatement");
                    }
                } else {
                    panic!("if_exp.alternative was None");
                }
            } else {
                panic!(
                    "stmt.expression is not IfExpression, got={:?}",
                    expr_stmt.expression
                );
            }
        } else {
            panic!("program.statements[0] is not ExpressionStatement");
        }
    }

    #[test]
    fn test_function_literal_parsing() {
        let input = "fn(x, y) { x + y; }";
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);

        assert_eq!(
            program.statements.len(),
            1,
            "program.statements does not contain 1 statement. got={}",
            program.statements.len()
        );

        if let ast::Statement::Expression(expr_stmt) = &program.statements[0] {
            if let Some(ast::Expression::FunctionLiteral(function)) = &expr_stmt.expression {
                assert_eq!(
                    function.parameters.len(),
                    2,
                    "function literal parameters wrong. want 2, got={}",
                    function.parameters.len()
                );

                assert_literal_expression(
                    &ast::Expression::Identifier(function.parameters[0].clone()),
                    &LiteralValue::String("x".to_string()),
                );

                assert_literal_expression(
                    &ast::Expression::Identifier(function.parameters[1].clone()),
                    &LiteralValue::String("y".to_string()),
                );

                assert_eq!(
                    function.body.statements.len(),
                    1,
                    "function.body.statements has not 1 statement. got={}",
                    function.body.statements.len()
                );

                if let ast::Statement::Expression(body_stmt) = &function.body.statements[0] {
                    if let Some(ref expr) = body_stmt.expression {
                        assert_infix_expression(
                            expr,
                            &LiteralValue::String("x".to_string()),
                            "+",
                            &LiteralValue::String("y".to_string()),
                        );
                    } else {
                        panic!("body statement expression is None");
                    }
                } else {
                    panic!("function body statement is not ExpressionStatement");
                }
            } else {
                panic!(
                    "stmt.expression is not FunctionLiteral, got={:?}",
                    expr_stmt.expression
                );
            }
        } else {
            panic!("program.statements[0] is not ExpressionStatement");
        }
    }

    #[test]
    fn test_function_parameter_parsing() {
        struct Test {
            input: &'static str,
            expected_params: Vec<&'static str>,
        }

        let tests = vec![
            Test {
                input: "fn() {};",
                expected_params: vec![],
            },
            Test {
                input: "fn(x) {};",
                expected_params: vec!["x"],
            },
            Test {
                input: "fn(x, y, z) {};",
                expected_params: vec!["x", "y", "z"],
            },
        ];

        for test in tests {
            let l = Lexer::new(test.input);
            let mut p = Parser::new(l);
            let program = p.parse_program();
            check_parser_errors(&p);

            if let ast::Statement::Expression(expr_stmt) = &program.statements[0] {
                if let Some(ast::Expression::FunctionLiteral(function)) = &expr_stmt.expression {
                    assert_eq!(
                        function.parameters.len(),
                        test.expected_params.len(),
                        "length parameters wrong. want {}, got={}",
                        test.expected_params.len(),
                        function.parameters.len()
                    );

                    for (i, ident) in test.expected_params.iter().enumerate() {
                        assert_literal_expression(
                            &ast::Expression::Identifier(function.parameters[i].clone()),
                            &LiteralValue::String(ident.to_string()),
                        );
                    }
                } else {
                    panic!("stmt.expression is not FunctionLiteral");
                }
            }
        }
    }

    #[test]
    fn test_call_expression_parsing() {
        let input = "add(1, 2 * 3, 4 + 5);";
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);

        assert_eq!(
            program.statements.len(),
            1,
            "program.statements does not contain 1 statement. got={}",
            program.statements.len()
        );

        if let ast::Statement::Expression(expr_stmt) = &program.statements[0] {
            if let Some(ast::Expression::CallExpression(call_exp)) = &expr_stmt.expression {
                assert_identifier(&call_exp.function, "add");

                assert_eq!(
                    call_exp.arguments.len(),
                    3,
                    "wrong length of arguments. got={}",
                    call_exp.arguments.len()
                );

                assert_literal_expression(&call_exp.arguments[0], &LiteralValue::Int(1));
                assert_infix_expression(
                    &call_exp.arguments[1],
                    &LiteralValue::Int(2),
                    "*",
                    &LiteralValue::Int(3),
                );
                assert_infix_expression(
                    &call_exp.arguments[2],
                    &LiteralValue::Int(4),
                    "+",
                    &LiteralValue::Int(5),
                );
            } else {
                panic!(
                    "stmt.expression is not CallExpression, got={:?}",
                    expr_stmt.expression
                );
            }
        } else {
            panic!("program.statements[0] is not ExpressionStatement");
        }
    }

    #[test]
    fn test_string_literal_expression() {
        let input = r#""hello world";"#;
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);

        if let ast::Statement::Expression(expr_stmt) = &program.statements[0] {
            if let Some(ast::Expression::StringLiteral(literal)) = &expr_stmt.expression {
                assert_eq!(literal.value, "hello world");
            } else {
                panic!("exp not StringLiteral, got={:?}", expr_stmt.expression);
            }
        } else {
            panic!("program.statements[0] is not ExpressionStatement");
        }
    }

    #[test]
    fn test_parsing_array_literals() {
        let input = "[1, 2 * 2, 3 + 3]";
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);

        if let ast::Statement::Expression(expr_stmt) = &program.statements[0] {
            if let Some(ast::Expression::ArrayLiteral(array)) = &expr_stmt.expression {
                assert_eq!(
                    array.elements.len(),
                    3,
                    "len(array.elements) not 3. got={}",
                    array.elements.len()
                );

                assert_integer_literal(&array.elements[0], 1);
                assert_infix_expression(
                    &array.elements[1],
                    &LiteralValue::Int(2),
                    "*",
                    &LiteralValue::Int(2),
                );
                assert_infix_expression(
                    &array.elements[2],
                    &LiteralValue::Int(3),
                    "+",
                    &LiteralValue::Int(3),
                );
            } else {
                panic!("exp not ArrayLiteral, got={:?}", expr_stmt.expression);
            }
        } else {
            panic!("program.statements[0] is not ExpressionStatement");
        }
    }

    #[test]
    fn test_parsing_index_expressions() {
        let input = "myArray[1 + 1]";
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);

        if let ast::Statement::Expression(expr_stmt) = &program.statements[0] {
            if let Some(ast::Expression::IndexExpression(index_exp)) = &expr_stmt.expression {
                assert_identifier(&index_exp.left, "myArray");
                assert_infix_expression(
                    &index_exp.index,
                    &LiteralValue::Int(1),
                    "+",
                    &LiteralValue::Int(1),
                );
            } else {
                panic!("exp not IndexExpression, got={:?}", expr_stmt.expression);
            }
        } else {
            panic!("program.statements[0] is not ExpressionStatement");
        }
    }

    #[test]
    fn test_parsing_hash_literals_string_keys() {
        let input = r#"{"one": 1, "two": 2, "three": 3}"#;
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);

        if let ast::Statement::Expression(expr_stmt) = &program.statements[0] {
            if let Some(ast::Expression::HashLiteral(hash)) = &expr_stmt.expression {
                assert_eq!(
                    hash.pairs.len(),
                    3,
                    "hash.pairs has wrong length. got={}",
                    hash.pairs.len()
                );

                let expected = vec![("one", 1), ("two", 2), ("three", 3)];

                for (expected_key, expected_value) in expected {
                    let key = ast::Expression::StringLiteral(ast::StringLiteral {
                        token: crate::token::Token::new(
                            crate::token::TokenType::String,
                            expected_key,
                        ),
                        value: expected_key.to_string(),
                    });

                    if let Some(value) = hash.pairs.get(&key) {
                        assert_integer_literal(value, expected_value);
                    } else {
                        panic!("No pair for given key in Pairs");
                    }
                }
            } else {
                panic!("exp is not HashLiteral, got={:?}", expr_stmt.expression);
            }
        } else {
            panic!("program.statements[0] is not ExpressionStatement");
        }
    }

    #[test]
    fn test_parsing_empty_hash_literal() {
        let input = "{}";
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);

        if let ast::Statement::Expression(expr_stmt) = &program.statements[0] {
            if let Some(ast::Expression::HashLiteral(hash)) = &expr_stmt.expression {
                assert_eq!(
                    hash.pairs.len(),
                    0,
                    "hash.pairs has wrong length. got={}",
                    hash.pairs.len()
                );
            } else {
                panic!("exp is not HashLiteral, got={:?}", expr_stmt.expression);
            }
        } else {
            panic!("program.statements[0] is not ExpressionStatement");
        }
    }

    #[test]
    fn test_parsing_hash_literals_with_expressions() {
        let input = r#"{"one": 0 + 1, "two": 10 - 8, "three": 15 / 5}"#;
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);

        if let ast::Statement::Expression(expr_stmt) = &program.statements[0] {
            if let Some(ast::Expression::HashLiteral(hash)) = &expr_stmt.expression {
                assert_eq!(
                    hash.pairs.len(),
                    3,
                    "hash.pairs has wrong length. got={}",
                    hash.pairs.len()
                );

                let tests = vec![
                    ("one", LiteralValue::Int(0), "+", LiteralValue::Int(1)),
                    ("two", LiteralValue::Int(10), "-", LiteralValue::Int(8)),
                    ("three", LiteralValue::Int(15), "/", LiteralValue::Int(5)),
                ];

                for (expected_key, left, operator, right) in tests {
                    let key = ast::Expression::StringLiteral(ast::StringLiteral {
                        token: crate::token::Token::new(
                            crate::token::TokenType::String,
                            expected_key,
                        ),
                        value: expected_key.to_string(),
                    });

                    if let Some(value) = hash.pairs.get(&key) {
                        assert_infix_expression(value, &left, operator, &right);
                    } else {
                        panic!("No pair for given key in Pairs");
                    }
                }
            } else {
                panic!("exp is not HashLiteral, got={:?}", expr_stmt.expression);
            }
        } else {
            panic!("program.statements[0] is not ExpressionStatement");
        }
    }
}
