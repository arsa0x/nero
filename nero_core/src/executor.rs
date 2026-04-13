use std::time::Instant;

use crate::{
    ast::{Expression, RequestOption, Statement},
    error::ExecError,
    interpreter::{Interpreter, Value},
    token::RequestMethod,
};

use reqwest::Client;

pub struct Executor {
    interpreter: Interpreter,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            interpreter: Interpreter::new(),
        }
    }
    fn eval(&mut self, expr: &Expression) -> Result<Value, ExecError> {
        self.interpreter
            .eval_expr(expr)
            .map_err(|_| ExecError::InvalidExpression)
    }

    pub async fn execute(&mut self, stmt: &Statement) -> Result<(), ExecError> {
        let Statement::Request {
            method,
            name,
            args,
            options,
        } = stmt
        else {
            return Ok(());
        };

        let client = Client::new();
        let url_expr = args.get(0).ok_or(ExecError::MissingUrl)?;
        let mut url = match self.eval(url_expr)? {
            Value::Number(n) => n.to_string(),
            Value::String(s) => s,
            _ => return Err(ExecError::InvalidExpression),
        };

        let mut query = Vec::new();
        let mut headers = Vec::new();
        let mut body = serde_json::Map::new();

        for opt in options {
            match opt {
                RequestOption::Query(q) => {
                    for (k, v) in q {
                        let val = self.eval(v)?;
                        let val = match val {
                            Value::Number(n) => n.to_string(),
                            Value::String(s) => s,
                            _ => return Err(ExecError::InvalidExpression),
                        };
                        query.push((k.clone(), val));
                    }
                }
                RequestOption::Headers(h) => {
                    for (k, v) in h {
                        let val = self.eval(v)?;
                        let val = match val {
                            Value::Number(n) => n.to_string(),
                            Value::String(s) => s,
                            _ => return Err(ExecError::InvalidExpression),
                        };
                        headers.push((k.clone(), val));
                    }
                }
                RequestOption::Body(b) => {
                    for (k, v) in b {
                        let val = self.eval(v)?;
                        match val {
                            Value::String(s) => {
                                body.insert(k.clone(), serde_json::Value::String(s));
                            }
                            Value::Number(n) => {
                                body.insert(k.clone(), serde_json::Value::Number(n.into()));
                            }
                            _ => return Err(ExecError::InvalidExpression),
                        }
                    }
                }
                _ => {}
            }
        }
        if !query.is_empty() {
            let qs = query
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("&");
            url = format!("{}?{}", url, qs);
        }

        let start = Instant::now();

        let mut req = match method {
            RequestMethod::GET => client.get(&url),
            RequestMethod::POST => client.post(&url),
            RequestMethod::PUT => client.put(&url),
            RequestMethod::PATCH => client.patch(&url),
            RequestMethod::DELETE => client.delete(&url),
        };

        for (k, v) in headers {
            req = req.header(&k, &v)
        }

        if !body.is_empty() {
            req = req.json(&body);
        }

        let res = req.send().await.map_err(|_| ExecError::RequestFailed)?;
        let duration = start.elapsed().as_millis();
        println!("[{}] {} → {} ({} ms)", name, url, res.status(), duration);
        return Ok(());
    }
}
