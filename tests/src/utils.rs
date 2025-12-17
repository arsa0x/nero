use nero_core::{
    ast::Stmt,
    lexer::Lexer,
    parser::{Parser, ParserError},
};

#[allow(unused)]
pub struct TestUtils;

#[allow(unused)]
impl TestUtils {
    pub fn parse_ok(input: &str) -> Vec<Stmt> {
        let tokens = Lexer::tokenize(input).unwrap();
        let mut parser = Parser { tokens, pos: 0 };
        parser.parse().unwrap()
    }

    pub fn parse_err(input: &str) -> ParserError {
        let tokens = Lexer::tokenize(input).unwrap();
        let mut parser = Parser { tokens, pos: 0 };
        parser.parse().unwrap_err()
    }
}
