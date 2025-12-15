use nero_core::{
    lexer::Lexer,
    parser::{Parser, ParserError},
    resolver::Resolver,
};

const EXAMPLE: &str = include_str!("../../example/get_method.ns");

fn main() -> Result<(), ParserError> {
    println!("{}\n", EXAMPLE);
    let lex = Lexer::tokenize(EXAMPLE);

    match lex {
        Ok(result) => {
            let mut parser = Parser::new(result);
            let ast = parser.parse()?;

            let mut resolver = Resolver::new();

            for stmt in &ast {
                match resolver.resolve_statement(stmt) {
                    Ok(_) => {
                        println!("{:?}", resolver.variables);
                    }
                    Err(e) => println!("{:?}", e),
                }
            }
        }
        Err(err) => println!("{:?}", err),
    }
    Ok(())
}
