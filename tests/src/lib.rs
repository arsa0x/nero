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
        resolver::Resolver,
        semantic::{SemanticChecker, SemanticError},
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

    #[test]
    fn duplicate_label() {
        let src = r#"
            #[test_get]
            @GET "url" {}
            #[test_get]
            @GET "url" {}
        "#;

        let ast = Parser::new(Lexer::tokenize(src).unwrap()).parse().unwrap();

        let mut resolver = Resolver::new();
        ast.iter().for_each(|s| {
            resolver.resolve_statement(s).unwrap();
        });

        let mut semantic = SemanticChecker::new(&resolver);

        let res = ast.iter().try_for_each(|s| semantic.check_statement(s));

        assert!(matches!(res, Err(SemanticError::DuplicateLabel(_))));
    }
}
