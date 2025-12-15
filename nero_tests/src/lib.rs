#[allow(unused)]
use nero_core::{
    self,
    ast::{Expr, Stmt},
    lexer::Lexer,
    parser::{Parser, ParserError},
};

#[allow(unused)]
fn parse_ok(input: &str) -> Vec<Stmt> {
    let tokens = Lexer::tokenize(input).unwrap();
    let mut parser = Parser { tokens, pos: 0 };
    parser.parse().unwrap()
}

#[allow(unused)]
fn parse_err(input: &str) -> ParserError {
    let tokens = Lexer::tokenize(input).unwrap();
    let mut parser = Parser { tokens, pos: 0 };
    parser.parse().unwrap_err()
}

#[test]
fn test_assignment_number() {
    let ast = parse_ok("port = 3000;");

    assert_eq!(
        ast,
        vec![Stmt::Assignment {
            name: "port".into(),
            value: Expr::Number(3000),
        }]
    );
}

#[test]
fn test_assignment_missing_semicolon() {
    let err = parse_err("port = 3000");

    assert!(matches!(err, ParserError::UnexpectedEOF));
}
