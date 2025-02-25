use std::fmt;

use crate::token;

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    fn token_literal(&self) -> Option<String> {
        self.statements.get(0).map(|stmt| stmt.token_literal())
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();

        for stmt in &self.statements {
            out.push_str(&stmt.to_string());
        }

        write!(f, "{}", out)
    }
}

pub trait Node {
    fn token_literal(&self) -> String;
}

#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Let(let_stmt) => write!(f, "{}", let_stmt),
            Statement::Return(ret_stmt) => write!(f, "{}", ret_stmt),
            Statement::Expression(expr_stmt) => write!(f, "{:#?}", expr_stmt),
        }
    }
}

#[derive(Debug)]
pub struct LetStatement {
    pub name: String,
    pub token: token::Token,
    pub value: Option<Expression>,
}

impl fmt::Display for LetStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} = {};",
            self.token_literal(),
            self.name,
            self.value
                .as_ref()
                .map_or("".to_string(), |expr| expr.token_literal())
        )
    }
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: token::Token,
    pub return_value: Option<Expression>,
}

impl fmt::Display for ReturnStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {};",
            self.token.literal,
            self.return_value
                .as_ref()
                .map_or("".to_string(), |expr| expr.token_literal())
        )
    }
}

// ExpressionStatement is a statement that consists of a single expression.
#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: token::Token, // first token in the expression
    pub expression: Expression,
}

impl fmt::Display for ExpressionStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.expression.token_literal())
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub token: token::Token, // The Ident token
    pub value: String,
}

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(i64),
    PrefixExpression {
        operator: String,
        right: Box<Expression>,
    },
    InfixExpression {
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    },
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Let(let_stmt) => let_stmt.token.literal.clone(),
            Self::Return(ret_stmt) => ret_stmt.token.literal.clone(),
            Self::Expression(expr_stmt) => expr_stmt.token.literal.clone(),
        }
    }
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(ident) => ident.token.literal.clone(),
            Expression::IntegerLiteral(val) => val.to_string(),
            Expression::PrefixExpression { operator, .. } => operator.clone(),
            Expression::InfixExpression { operator, .. } => operator.clone(),
        }
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_string() {
        let program = Program {
            statements: vec![Statement::Let(LetStatement {
                token: token::Token {
                    token_type: token::TokenType::Let,
                    literal: "let".to_string(),
                },
                name: "myVar".to_string(),
                value: Some(Expression::Identifier(Identifier {
                    token: token::Token {
                        token_type: token::TokenType::Ident,
                        literal: "anotherVar".to_string(),
                    },
                    value: "anotherVar".to_string(),
                })),
            })],
        };

        assert_eq!(program.to_string(), "let myVar = anotherVar;");
    }
}
