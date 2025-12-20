use anyhow::Ok;
use nero_core::{
    ast::Stmt, lexer::Lexer, parser::Parser, resolver::Resolver, semantic::SemanticChecker,
};
use nero_requests::executor::Executor;
use std::{fs, time::Instant};

pub struct RunCmd {
    pub label: String,
    pub status: u16,
    pub method: String,
    pub size: u64,
    pub headers: Vec<(String, String)>,
    pub body: String,
    pub duration_ms: u128,
}
impl RunCmd {
    pub async fn from_file(file: &str) -> Result<Vec<RunCmd>, anyhow::Error> {
        let source_code = fs::read_to_string(file)?;
        let tokens = Lexer::tokenize(&source_code)?;
        let ast = Parser::new(tokens).parse()?;

        let mut result: Vec<RunCmd> = Vec::new();

        let mut resolver = Resolver::new();
        for stmt in &ast {
            resolver.resolve_statement(stmt)?;
        }

        let mut semantic = SemanticChecker::new(&resolver);
        for stmt in &ast {
            semantic.check_statement(stmt)?;
        }

        let executor = Executor::new(&resolver);
        for stmt in &ast {
            if let Stmt::Request(req) = stmt {
                let start = Instant::now();
                let res = executor.execute(req).await?;
                let size = res.content_length().unwrap_or(0);
                let status = res.status().as_u16();
                let duration_ms = start.elapsed().as_millis();
                let headers = res
                    .headers()
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                    .collect::<Vec<_>>();
                let body = res.text().await?;
                let label = req.label.clone();
                let method = req.method.clone();

                result.push(RunCmd {
                    method,
                    size,
                    label,
                    status,
                    headers,
                    body,
                    duration_ms,
                });
            }
        }
        Ok(result)
    }
}
