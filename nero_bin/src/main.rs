use nero_core::{
    lexer::Lexer,
    parser::{Parser, ParserError},
};

const EXAMPLE: &str = include_str!("../../example/get_method.ns");

fn main() -> Result<(), ParserError> {
    println!("{}\n", EXAMPLE);
    let lex = Lexer::tokenize(EXAMPLE);

    match lex {
        Ok(result) => {
            let mut parser: Parser = Parser::new(result);
            let _ = parser.parse()?;
        }
        Err(err) => println!("{:?}", err),
    }
    Ok(())
}
