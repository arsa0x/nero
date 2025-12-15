mod utils;

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = include_str!("../../example/get_method.ns");
    use crate::utils::TestUtils;
    use nero_core::{
        self,
        ast::{Expr, Stmt},
        lexer::Lexer,
        parser::{Parser, ParserError},
    };

    #[test]
    fn test_assignment_number() {
        let ast = TestUtils::parse_ok("port = 3000;");

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
        let err = TestUtils::parse_err("port = 3000");

        assert!(matches!(err, ParserError::UnexpectedEOF));
    }

    #[test]
    fn parse_simple_get() {
        let tokens = Lexer::tokenize(EXAMPLE).unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        assert_eq!(ast.len(), 3);
    }
}
