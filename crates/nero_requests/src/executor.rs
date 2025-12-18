use nero_core::{
    ast::{Expr, Req},
    resolver::{self, Resolver},
};
use reqwest;

#[derive(Debug)]
pub enum ExecutorError {
    UnsupportedMethod,
    RequestFailed,
}

impl std::error::Error for ExecutorError {}
impl std::fmt::Display for ExecutorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedMethod => write!(f, "Unsupported method"),
            Self::RequestFailed => write!(f, "Requests failed"),
        }
    }
}

pub struct Executor<'a> {
    pub resolver: &'a Resolver,
    pub client: reqwest::Client,
}

impl<'a> Executor<'a> {
    pub fn new(resolver: &'a Resolver) -> Self {
        Self {
            resolver,
            client: reqwest::Client::new(),
        }
    }

    fn resolve_string(&self, expr: &Expr) -> String {
        match self.resolver.resolve_expression(expr).unwrap() {
            resolver::Value::Number(n) => n.to_string(),
            resolver::Value::String(s) => s,
        }
    }

    pub async fn execute(&self, req: &Req) -> Result<(), ExecutorError> {
        let url = self.resolve_string(&req.url);
        let mut request = match req.method.as_str() {
            "GET" => self.client.get(&url),
            "POST" => self.client.post(&url),
            _ => return Err(ExecutorError::UnsupportedMethod),
        };

        for (k, v) in &req.headers {
            request = request.header(k, self.resolve_string(v));
        }

        if !req.query.is_empty() {
            let q: Vec<(String, String)> = req
                .query
                .iter()
                .map(|(k, v)| (k.clone(), self.resolve_string(v)))
                .collect();
            request = request.query(&q)
        }

        if let Some(body) = &req.body {
            let mut map = serde_json::Map::new();
            for (k, v) in body {
                map.insert(k.clone(), serde_json::json!(self.resolve_string(v)));
            }
            request = request.json(&map);
        }

        let res = request
            .send()
            .await
            .map_err(|_| ExecutorError::RequestFailed)?;

        println!("Status: {}", res.status());
        println!("Body:\n{}", res.text().await.unwrap());
        Ok(())
    }
}
