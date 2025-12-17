use std::collections::HashMap;

use crate::ast::{Expr, Stmt, StringPart};

pub struct Resolver {
    pub variables: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(i64),
    String(String),
}

#[derive(Debug)]
pub enum ResolveError {
    UndefinedVariable(String),
    InvalidExpression,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
    pub fn resolve_expression(&self, expr: &Expr) -> Result<Value, ResolveError> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::Identifier(name) => self
                .variables
                .get(name)
                .cloned()
                .ok_or(ResolveError::UndefinedVariable(name.clone())),
            Expr::String(parts) => {
                let mut result = String::new();

                for part in parts {
                    match part {
                        StringPart::Text(t) => result.push_str(t),
                        StringPart::Expression(e) => {
                            let v = self.resolve_expression(e)?;
                            match v {
                                Value::String(s) => result.push_str(&s),
                                Value::Number(n) => result.push_str(&n.to_string()),
                            }
                        }
                    }
                }
                Ok(Value::String(result))
            }
        }
    }
    pub fn resolve_statement(&mut self, stmt: &Stmt) -> Result<(), ResolveError> {
        match stmt {
            Stmt::Assignment { name, value } => {
                let v = self.resolve_expression(value)?;
                self.variables.insert(name.clone(), v);
                Ok(())
            }
            Stmt::Request(req) => {
                let _url = self.resolve_expression(&req.url)?;
                Ok(())
            }
        }
    }
}
