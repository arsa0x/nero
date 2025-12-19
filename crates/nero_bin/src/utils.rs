use anyhow::Ok;
use nero_core::{
    ast::Stmt, lexer::Lexer, parser::Parser, resolver::Resolver, semantic::SemanticChecker,
};
use nero_requests::executor::Executor;
use std::fs;

pub struct Utils;

impl Utils {
    pub async fn run_file(file: &String) -> Result<(), anyhow::Error> {
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
        for stmt in &ast {
            if let Stmt::Request(req) = stmt {
                let response = executor.execute(req).await?;
                println!("{}", &response.status());
            }
        }
        Ok(())
    }
}
