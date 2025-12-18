use nero_core::{
    ast::Stmt, lexer::Lexer, parser::Parser, resolver::Resolver, semantic::SemanticChecker,
};
use nero_requests::executor::Executor;

const EXAMPLE: &str = include_str!("../../../dev/script/simple_get.ns");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tokens = Lexer::tokenize(EXAMPLE)?;
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
            executor.execute(req).await?;
        }
    }

    Ok(())
}
