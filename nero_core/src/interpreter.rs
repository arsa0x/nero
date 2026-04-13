use std::collections::HashMap;

use crate::{
    ast::{Expression, Statement},
    token::MathOperator,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Number(i32),
    Null,
}

#[derive(Debug, Default)]
pub struct Env {
    pub vars: HashMap<String, Value>,
}

impl Env {
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.vars.get(name)
    }

    pub fn set(&mut self, name: String, value: Value) {
        self.vars.insert(name, value);
    }
}

pub struct Interpreter {
    pub env: Env,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Env::default(),
        }
    }

    pub fn eval_expr(&mut self, expr: &Expression) -> Result<Value, String> {
        match expr {
            Expression::Number(n) => Ok(Value::Number(*n)),
            Expression::String(s) => Ok(Value::String(s.clone())),
            Expression::Identifier(name) => self
                .env
                .get(name)
                .cloned()
                .ok_or(format!("Undefined variable: {}", name)),
            Expression::Binary { left, op, right } => {
                let l = self.eval_expr(left)?;
                let r = self.eval_expr(right)?;
                self.eval_binary(l, op, r)
            }
            Expression::Unary { op, expr } => {
                let val = self.eval_expr(expr)?;
                match (op, val) {
                    (MathOperator::Sub, Value::Number(n)) => Ok(Value::Number(-n)),
                    _ => Err("Invalid unary operator".into()),
                }
            }
        }
    }

    pub fn eval_stmt(&mut self, stmt: &Statement) -> Result<(), String> {
        match stmt {
            Statement::Assignment { name, value } => {
                let val = self.eval_expr(value)?;
                self.env.set(name.clone(), val);
                Ok(())
            }
            Statement::Request { .. } => Ok(()),
        }
    }

    fn eval_binary(&self, l: Value, op: &MathOperator, r: Value) -> Result<Value, String> {
        match (l, r) {
            (Value::Number(a), Value::Number(b)) => match op {
                MathOperator::Add => Ok(Value::Number(a + b)),
                MathOperator::Sub => Ok(Value::Number(a - b)),
                MathOperator::Mul => Ok(Value::Number(a * b)),
                MathOperator::Div => Ok(Value::Number(a / b)),
            },
            (Value::String(s), Value::Number(n)) => match op {
                MathOperator::Add => Ok(Value::String(format!("{}{}", s, n))),
                _ => Err("Invalid string operation".into()),
            },
            _ => Err("Type mismatch in binary operation".into()),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        interpreter::{Interpreter, Value},
        lexer::Lexer,
        parser::Parser,
    };

    #[test]
    fn eval_arithmetic() {
        let case_1 = Parser::new(Lexer::new("5 + 2 * 3").tokenize().unwrap())
            .parse_expression(0)
            .unwrap();

        let case_2 = Parser::new(Lexer::new("-5 + 5").tokenize().unwrap())
            .parse_expression(0)
            .unwrap();

        let case_3 = Parser::new(Lexer::new("10 * 3 / (5 + 5)").tokenize().unwrap())
            .parse_expression(0)
            .unwrap();

        let mut interp = Interpreter::new();

        assert_eq!(interp.eval_expr(&case_1).unwrap(), Value::Number(11));
        assert_eq!(interp.eval_expr(&case_2).unwrap(), Value::Number(0));
        assert_eq!(interp.eval_expr(&case_3).unwrap(), Value::Number(3));
    }

    #[test]
    fn eval_variable() {
        let mut interp = Interpreter::new();

        let stmts = Parser::new(
            Lexer::new(
                r#"
                a = 10
                b = a + 5
            "#,
            )
            .tokenize()
            .unwrap(),
        )
        .parse()
        .unwrap();

        for stmt in &stmts {
            interp.eval_stmt(stmt).unwrap();
        }

        assert_eq!(interp.env.get("b"), Some(&Value::Number(15)));
    }
}
