use nero_core::{
    lexer::Lexer,
    parser::Parser,
    resolver::Resolver,
    semantic::{SemanticChecker, SemanticError},
};

const EXAMPLE: &str = include_str!("../../../example/get_method.ns");

fn main() -> Result<(), SemanticError> {
    println!("{}\n", EXAMPLE);
    let lex = Lexer::tokenize(EXAMPLE);

    match lex {
        Ok(result) => {
            let mut parser = Parser::new(result);
            let ast = parser.parse().unwrap();

            let mut resolver = Resolver::new();

            for stmt in &ast {
                resolver.resolve_statement(stmt).unwrap();
            }

            let mut semantic = SemanticChecker::new(&resolver);

            for stmt in &ast {
                semantic.check_statement(stmt)?;
            }
        }
        Err(err) => println!("{:?}", err),
    }
    Ok(())
}
