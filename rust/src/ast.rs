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

pub trait Node {
    fn token_literal(&self) -> String;
}

#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
}

#[derive(Debug)]
pub struct LetStatement {
    pub name: String,
    pub token: token::Token,
    pub value: Option<Expression>,
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: token::Token,
    pub return_value: Option<Expression>,
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
