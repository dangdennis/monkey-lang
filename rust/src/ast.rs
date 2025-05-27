use std::fmt;
use std::collections::HashMap;

use crate::token;

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn token_literal(&self) -> String {
        if let Some(first_stmt) = self.statements.get(0) {
            first_stmt.token_literal()
        } else {
            String::new()
        }
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

#[derive(Debug, Clone)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
    Block(BlockStatement),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Let(let_stmt) => write!(f, "{}", let_stmt),
            Statement::Return(ret_stmt) => write!(f, "{}", ret_stmt),
            Statement::Expression(expr_stmt) => write!(f, "{}", expr_stmt),
            Statement::Block(block_stmt) => write!(f, "{}", block_stmt),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub token: token::Token,
    pub name: Identifier,
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
                .map_or("".to_string(), |expr| expr.to_string())
        )
    }
}

#[derive(Debug, Clone)]
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
                .map_or("".to_string(), |expr| expr.to_string())
        )
    }
}

#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub token: token::Token,
    pub expression: Option<Expression>,
}

impl fmt::Display for ExpressionStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref expr) = self.expression {
            write!(f, "{}", expr)
        } else {
            write!(f, "")
        }
    }
}

#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub token: token::Token,
    pub statements: Vec<Statement>,
}

impl fmt::Display for BlockStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        for stmt in &self.statements {
            out.push_str(&stmt.to_string());
        }
        write!(f, "{}", out)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub token: token::Token,
    pub value: String,
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Display for IntegerLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl PartialEq<str> for Identifier {
    fn eq(&self, other: &str) -> bool {
        self.value == other
    }
}

impl PartialEq<&str> for Identifier {
    fn eq(&self, other: &&str) -> bool {
        self.value == *other
    }
}

impl PartialEq<i64> for IntegerLiteral {
    fn eq(&self, other: &i64) -> bool {
        self.value == *other
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    Boolean(Boolean),
    PrefixExpression {
        token: token::Token,
        operator: String,
        right: Box<Expression>,
    },
    InfixExpression {
        token: token::Token,
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    },
    IfExpression(IfExpression),
    FunctionLiteral(FunctionLiteral),
    CallExpression(CallExpression),
    StringLiteral(StringLiteral),
    ArrayLiteral(ArrayLiteral),
    IndexExpression(IndexExpression),
    HashLiteral(HashLiteral),
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntegerLiteral {
    pub token: token::Token,
    pub value: i64,
}

#[derive(Debug, Clone)]
pub struct Boolean {
    pub token: token::Token,
    pub value: bool,
}

#[derive(Debug, Clone)]
pub struct IfExpression {
    pub token: token::Token,
    pub condition: Box<Expression>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

#[derive(Debug, Clone)]
pub struct FunctionLiteral {
    pub token: token::Token,
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}

#[derive(Debug, Clone)]
pub struct CallExpression {
    pub token: token::Token,
    pub function: Box<Expression>,
    pub arguments: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub token: token::Token,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct ArrayLiteral {
    pub token: token::Token,
    pub elements: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub struct IndexExpression {
    pub token: token::Token,
    pub left: Box<Expression>,
    pub index: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct HashLiteral {
    pub token: token::Token,
    pub pairs: HashMap<Expression, Expression>,
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Identifier(identifier) => write!(f, "{}", identifier.value),
            Expression::IntegerLiteral(int_lit) => write!(f, "{}", int_lit.token.literal),
            Expression::Boolean(boolean) => write!(f, "{}", boolean.token.literal),
            Expression::PrefixExpression { operator, right, .. } => {
                write!(f, "({}{})", operator, right)
            }
            Expression::InfixExpression {
                left,
                right,
                operator,
                ..
            } => write!(f, "({} {} {})", left, operator, right),
            Expression::IfExpression(if_expr) => {
                let mut result = format!("if{} {}", if_expr.condition, if_expr.consequence);
                if let Some(ref alt) = if_expr.alternative {
                    result.push_str(&format!("else {}", alt));
                }
                write!(f, "{}", result)
            }
            Expression::FunctionLiteral(fn_lit) => {
                let params: Vec<String> = fn_lit.parameters.iter().map(|p| p.to_string()).collect();
                write!(f, "{}({}) {}", fn_lit.token.literal, params.join(", "), fn_lit.body)
            }
            Expression::CallExpression(call_expr) => {
                let args: Vec<String> = call_expr.arguments.iter().map(|a| a.to_string()).collect();
                write!(f, "{}({})", call_expr.function, args.join(", "))
            }
            Expression::StringLiteral(str_lit) => write!(f, "{}", str_lit.token.literal),
            Expression::ArrayLiteral(arr_lit) => {
                let elements: Vec<String> = arr_lit.elements.iter().map(|e| e.to_string()).collect();
                write!(f, "[{}]", elements.join(", "))
            }
            Expression::IndexExpression(idx_expr) => {
                write!(f, "({}[{}])", idx_expr.left, idx_expr.index)
            }
            Expression::HashLiteral(hash_lit) => {
                let pairs: Vec<String> = hash_lit.pairs.iter().map(|(k, v)| format!("{}:{}", k, v)).collect();
                write!(f, "{{{}}}", pairs.join(", "))
            }
        }
    }
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Let(let_stmt) => let_stmt.token.literal.clone(),
            Statement::Return(ret_stmt) => ret_stmt.token.literal.clone(),
            Statement::Expression(expr_stmt) => expr_stmt.token.literal.clone(),
            Statement::Block(block_stmt) => block_stmt.token.literal.clone(),
        }
    }
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(ident) => ident.token.literal.clone(),
            Expression::IntegerLiteral(int_lit) => int_lit.token.literal.clone(),
            Expression::Boolean(boolean) => boolean.token.literal.clone(),
            Expression::PrefixExpression { token, .. } => token.literal.clone(),
            Expression::InfixExpression { token, .. } => token.literal.clone(),
            Expression::IfExpression(if_expr) => if_expr.token.literal.clone(),
            Expression::FunctionLiteral(fn_lit) => fn_lit.token.literal.clone(),
            Expression::CallExpression(call_expr) => call_expr.token.literal.clone(),
            Expression::StringLiteral(str_lit) => str_lit.token.literal.clone(),
            Expression::ArrayLiteral(arr_lit) => arr_lit.token.literal.clone(),
            Expression::IndexExpression(idx_expr) => idx_expr.token.literal.clone(),
            Expression::HashLiteral(hash_lit) => hash_lit.token.literal.clone(),
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

// We need to implement Hash and Eq for Expression to use it as HashMap key
impl std::hash::Hash for Expression {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Expression::Identifier(ident) => {
                "identifier".hash(state);
                ident.value.hash(state);
            }
            Expression::IntegerLiteral(int_lit) => {
                "integer".hash(state);
                int_lit.value.hash(state);
            }
            Expression::Boolean(boolean) => {
                "boolean".hash(state);
                boolean.value.hash(state);
            }
            Expression::StringLiteral(str_lit) => {
                "string".hash(state);
                str_lit.value.hash(state);
            }
            _ => {
                // For complex expressions, hash their string representation
                self.to_string().hash(state);
            }
        }
    }
}

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Expression::Identifier(a), Expression::Identifier(b)) => a.value == b.value,
            (Expression::IntegerLiteral(a), Expression::IntegerLiteral(b)) => a.value == b.value,
            (Expression::Boolean(a), Expression::Boolean(b)) => a.value == b.value,
            (Expression::StringLiteral(a), Expression::StringLiteral(b)) => a.value == b.value,
            _ => self.to_string() == other.to_string(),
        }
    }
}

impl Eq for Expression {}

#[cfg(test)]
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
                name: Identifier {
                    token: token::Token {
                        token_type: token::TokenType::Ident,
                        literal: "myVar".to_string(),
                    },
                    value: "myVar".to_string(),
                },
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