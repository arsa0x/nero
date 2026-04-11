use crate::token::{MathOperator, RequestMethod};

#[derive(Debug, PartialEq)]
pub enum Statement {
    Assignment {
        name: String,
        value: Expression,
    },
    Request {
        method: RequestMethod,
        name: String,
        args: Vec<Expression>,
        options: Vec<RequestOption>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Number(i32),
    String(String),
    Identifier(String),
    Binary {
        left: Box<Expression>,
        op: MathOperator,
        right: Box<Expression>,
    },
    Unary {
        op: MathOperator,
        expr: Box<Expression>,
    },
}

#[derive(Debug, PartialEq)]
pub enum RequestOption {
    Headers(Vec<(String, Expression)>),
    Body(Vec<(String, Expression)>),
    Query(Vec<(String, Expression)>),
    Timeout(Expression),
    Retry(Expression),
    Sleep(Expression),
}
