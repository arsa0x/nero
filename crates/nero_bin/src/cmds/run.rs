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
    pub file: String,
    pub date: String,
}
impl RunCmd {
    pub async fn from_file(file: &str) -> anyhow::Result<Vec<RunCmd>> {
        let source_code = fs::read_to_string(file)?;
        let tokens = Lexer::tokenize(&source_code)?;
        let ast = Parser::new(tokens).parse()?;

        let mut resolver = Resolver::new();
        for stmt in &ast {
            resolver.resolve_statement(stmt)?;
        }

        let mut semantic = SemanticChecker::new(&resolver);
        for stmt in &ast {
            semantic.check_statement(stmt)?;
        }

        let executor = Executor::new(&resolver);
        let mut result: Vec<RunCmd> = Vec::new();

        for stmt in &ast {
            let Stmt::Request(req) = stmt else {
                continue;
            };
            let start = Instant::now();

            let response = executor.execute(req).await?;

            let size = response.content_length().unwrap_or(0);
            let status = response.status().as_u16();
            let duration_ms = start.elapsed().as_millis();
            let headers = response
                .headers()
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                .collect::<Vec<_>>();
            let body = response.text().await?;

            result.push(RunCmd {
                file: file.to_string(),
                date: chrono::Utc::now().to_rfc3339(),
                method: req.method.clone(),
                size,
                label: req.label.clone(),
                status,
                headers,
                body,
                duration_ms,
            });
        }
        Ok(result)
    }
}
