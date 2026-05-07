use std::time::Instant;

use reqwest::Client;

use crate::{
    ast::{Expression, RequestOption, Statement},
    error::ExecError,
    interpreter::{Interpreter, Value},
    token::RequestMethod,
};

pub struct Executor {
    interpreter: Interpreter,
    client: Client,
}

impl Executor {
    pub fn new(interpreter: Interpreter) -> Self {
        Self {
            interpreter,
            client: Client::new(),
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

        let url_expr = args.get(0).ok_or(ExecError::MissingUrl)?;

        let url = match self.eval(url_expr)? {
            Value::String(s) => s,
            Value::Number(n) => n.to_string(),
            _ => return Err(ExecError::InvalidExpression),
        };

        let mut query = Vec::new();
        let mut headers = Vec::new();
        let mut body = serde_json::Map::new();

        for opt in options {
            match opt {
                RequestOption::Headers(expr) => {
                    let obj = self.eval(expr)?;
                    let obj = obj.as_object()?;

                    for (k, v) in obj {
                        headers.push((k.clone(), v.as_string()?));
                    }
                }

                RequestOption::Query(expr) => {
                    let obj = self.eval(expr)?;
                    let obj = obj.as_object()?;

                    for (k, v) in obj {
                        query.push((k.clone(), v.to_string_value()?));
                    }
                }

                RequestOption::Body(expr) => {
                    let obj = self.eval(expr)?;
                    let obj = obj.as_object()?;

                    for (k, v) in obj {
                        body.insert(k.clone(), v.to_json());
                    }
                }

                _ => {}
            }
        }

        let mut req = match method {
            RequestMethod::GET => self.client.get(&url),
            RequestMethod::POST => self.client.post(&url),
            RequestMethod::PUT => self.client.put(&url),
            RequestMethod::PATCH => self.client.patch(&url),
            RequestMethod::DELETE => self.client.delete(&url),
        };

        if !query.is_empty() {
            req = req.query(&query);
        }

        for (k, v) in headers {
            req = req.header(k, v);
        }

        if !body.is_empty() {
            req = req.json(&body);
        }

        let start = Instant::now();

        let res = req.send().await.map_err(|_| ExecError::RequestFailed)?;

        let status = res.status();
        let duration = start.elapsed().as_millis();

        println!("[{}] {} -> {} ({} ms)", name, url, status, duration);

        let content_type = res
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        if content_type.contains("application/json") {
            let json: serde_json::Value = res.json().await.map_err(|_| ExecError::RequestFailed)?;

            println!("{}", serde_json::to_string_pretty(&json).unwrap());
        } else {
            let text = res.text().await.map_err(|_| ExecError::RequestFailed)?;

            println!("{}", text);
        }

        println!();

        Ok(())
    }
}
