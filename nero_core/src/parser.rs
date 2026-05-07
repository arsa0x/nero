use crate::{
    ast::{Expression, RequestOption, Statement},
    error::ParserError,
    token::{Keyword, MathOperator, Token},
};

pub struct Parser<'a> {
    pub tokens: Vec<Token<'a>>,
    pub pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn current(&self) -> Option<&Token<'a>> {
        self.tokens.get(self.pos)
    }

    fn parse_assignment(&mut self) -> Result<Statement, ParserError> {
        let id = match self.current() {
            Some(Token::Identifier(n)) => {
                let name = n.to_string();
                self.advance();
                name
            }
            _ => return Err(ParserError::InvalidStatement),
        };

        match self.current() {
            Some(Token::Assignment) => self.advance(),
            _ => return Err(ParserError::InvalidStatement),
        }

        let value = self.parse_expression(0)?;

        Ok(Statement::Assignment { name: id, value })
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.current() {
            Some(Token::Identifier(_)) => match self.tokens.get(self.pos + 1) {
                Some(Token::Assignment) => self.parse_assignment(),
                _ => Err(ParserError::UnexpectedToken),
            },
            Some(Token::Method(_)) => self.parse_request(),
            _ => return Err(ParserError::InvalidStatement),
        }
    }

    fn parse_args(&mut self) -> Result<Vec<Expression>, ParserError> {
        let mut args = Vec::new();

        match self.current() {
            Some(Token::OpenParenthesis) => self.advance(),
            _ => return Err(ParserError::UnexpectedToken),
        }

        if matches!(self.current(), Some(Token::CloseParenthesis)) {
            return Err(ParserError::UnexpectedToken);
        }

        while !matches!(self.current(), Some(Token::CloseParenthesis)) {
            args.push(self.parse_expression(0)?);
            if matches!(self.current(), Some(Token::Comma)) {
                self.advance();
            } else {
                break;
            }
        }

        match self.current() {
            Some(Token::CloseParenthesis) => self.advance(),
            _ => return Err(ParserError::UnexpectedToken),
        }

        Ok(args)
    }
    fn parse_block(&mut self) -> Result<Vec<RequestOption>, ParserError> {
        let mut opts = Vec::new();

        match self.current() {
            Some(Token::OpenBrace) => self.advance(),
            _ => return Err(ParserError::UnexpectedToken),
        }

        while let Some(token) = self.current() {
            if matches!(token, Token::CloseBrace) {
                break;
            }
            opts.push(self.parse_opts()?);
        }

        match self.current() {
            Some(Token::CloseBrace) => self.advance(),
            None => return Err(ParserError::UnexpectedEOF),
            _ => return Err(ParserError::UnexpectedToken),
        }

        Ok(opts)
    }

    fn parse_opts(&mut self) -> Result<RequestOption, ParserError> {
        match self.current() {
            Some(Token::Keyword(Keyword::Query)) => self.parse_query(),
            Some(Token::Keyword(Keyword::Body)) => self.parse_body(),
            Some(Token::Keyword(Keyword::Headers)) => self.parse_headers(),
            Some(Token::Keyword(Keyword::Timeout)) => self.parse_timeout(),
            Some(Token::Keyword(Keyword::Retry)) => self.parse_retry(),
            _ => Err(ParserError::UnexpectedToken),
        }
    }

    fn parse_object(&mut self) -> Result<Expression, ParserError> {
        let mut entries = Vec::new();
        match self.current() {
            Some(Token::OpenBrace) => self.advance(),
            _ => return Err(ParserError::UnexpectedToken),
        }

        while let Some(token) = self.current() {
            if matches!(token, Token::CloseBrace) {
                break;
            }
            let key = match self.current() {
                Some(Token::String(s)) => {
                    let s = s.to_string();
                    self.advance();
                    s
                }
                _ => return Err(ParserError::UnexpectedToken),
            };
            match self.current() {
                Some(Token::Colon) => self.advance(),
                _ => return Err(ParserError::UnexpectedToken),
            }
            let value = self.parse_expression(0)?;

            entries.push((key, value));

            if matches!(self.current(), Some(Token::Comma)) {
                self.advance();
            } else if !matches!(self.current(), Some(Token::CloseBrace)) {
                return Err(ParserError::UnexpectedToken);
            }
        }
        match self.current() {
            Some(Token::CloseBrace) => self.advance(),
            _ => return Err(ParserError::UnexpectedEOF),
        }

        Ok(Expression::Object(entries))
    }

    fn parse_query(&mut self) -> Result<RequestOption, ParserError> {
        // let mut query = Vec::new();
        // self.advance();

        // match self.current() {
        //     Some(Token::OpenBrace) => self.advance(),
        //     _ => return Err(ParserError::UnexpectedToken),
        // }

        // while let Some(token) = self.current() {
        //     if matches!(token, Token::CloseBrace) {
        //         break;
        //     }
        //     let key = match self.current() {
        //         Some(Token::String(k)) => {
        //             let k = k.to_string();
        //             self.advance();
        //             k
        //         }
        //         _ => return Err(ParserError::UnexpectedToken),
        //     };
        //     match self.current() {
        //         Some(Token::Colon) => self.advance(),
        //         _ => return Err(ParserError::UnexpectedToken),
        //     }

        //     let value = self.parse_expression(0)?;
        //     query.push((key, value));
        //     if matches!(self.current(), Some(Token::Comma)) {
        //         self.advance();
        //     } else if !matches!(self.current(), Some(Token::CloseBrace)) {
        //         return Err(ParserError::UnexpectedToken);
        //     }
        // }

        // self.advance();

        // Ok(RequestOption::Query(query))
        self.advance();
        let expr = self.parse_expression(0)?;
        Ok(RequestOption::Query(expr))
    }

    fn parse_headers(&mut self) -> Result<RequestOption, ParserError> {
        // let mut headers = Vec::new();
        // self.advance();

        // match self.current() {
        //     Some(Token::OpenBrace) => self.advance(),
        //     _ => return Err(ParserError::UnexpectedToken),
        // }

        // while let Some(token) = self.current() {
        //     if matches!(token, Token::CloseBrace) {
        //         break;
        //     }
        //     let key = match self.current() {
        //         Some(Token::String(k)) => {
        //             let k = k.to_string();
        //             self.advance();
        //             k
        //         }
        //         _ => return Err(ParserError::UnexpectedToken),
        //     };
        //     match self.current() {
        //         Some(Token::Colon) => self.advance(),
        //         _ => return Err(ParserError::UnexpectedToken),
        //     }

        //     let value = self.parse_expression(0)?;
        //     headers.push((key, value));
        //     if matches!(self.current(), Some(Token::Comma)) {
        //         self.advance();
        //     } else if !matches!(self.current(), Some(Token::CloseBrace)) {
        //         return Err(ParserError::UnexpectedToken);
        //     }
        // }

        // self.advance();

        // Ok(RequestOption::Headers(headers))
        self.advance();
        let expr = self.parse_expression(0)?;
        Ok(RequestOption::Headers(expr))
    }
    fn parse_body(&mut self) -> Result<RequestOption, ParserError> {
        // let mut body = Vec::new();
        // self.advance();

        // match self.current() {
        //     Some(Token::OpenBrace) => self.advance(),
        //     _ => return Err(ParserError::UnexpectedToken),
        // }

        // while let Some(token) = self.current() {
        //     if matches!(token, Token::CloseBrace) {
        //         break;
        //     }
        //     let key = match self.current() {
        //         Some(Token::String(k)) => {
        //             let k = k.to_string();
        //             self.advance();
        //             k
        //         }
        //         _ => return Err(ParserError::UnexpectedToken),
        //     };
        //     match self.current() {
        //         Some(Token::Colon) => self.advance(),
        //         _ => return Err(ParserError::UnexpectedToken),
        //     }

        //     let value = self.parse_expression(0)?;
        //     body.push((key, value));
        //     if matches!(self.current(), Some(Token::Comma)) {
        //         self.advance();
        //     } else if !matches!(self.current(), Some(Token::CloseBrace)) {
        //         return Err(ParserError::UnexpectedToken);
        //     }
        // }

        // self.advance();

        // Ok(RequestOption::Body(body))
        self.advance();
        let expr = self.parse_expression(0)?;
        Ok(RequestOption::Body(expr))
    }
    fn parse_timeout(&mut self) -> Result<RequestOption, ParserError> {
        self.advance();
        let value = self.parse_expression(0)?;
        Ok(RequestOption::Timeout(value))
    }
    fn parse_retry(&mut self) -> Result<RequestOption, ParserError> {
        self.advance();
        let value = self.parse_expression(0)?;
        Ok(RequestOption::Retry(value))
    }

    fn parse_request(&mut self) -> Result<Statement, ParserError> {
        let method = match self.current() {
            Some(Token::Method(m)) => {
                let m = m.clone();
                self.advance();
                m
            }
            _ => return Err(ParserError::UnexpectedToken),
        };

        let name = match self.current() {
            Some(Token::Identifier(id)) => {
                let n = id.to_string();
                self.advance();
                n
            }
            _ => return Err(ParserError::UnexpectedToken),
        };

        let args = self.parse_args()?;
        let options = if matches!(self.current(), Some(Token::OpenBrace)) {
            self.parse_block()?
        } else {
            Vec::new()
        };

        Ok(Statement::Request {
            method,
            name,
            args,
            options,
        })
    }

    pub fn parse_expression(&mut self, min_bp: u8) -> Result<Expression, ParserError> {
        let mut lhs = self.parse_primary()?;

        loop {
            let op = match self.current() {
                Some(Token::Operator(op)) => op.clone(),
                _ => break,
            };

            let (l_bp, r_bp) = op.get_bind_power();
            if l_bp < min_bp {
                break;
            }

            self.advance();

            let rhs = self.parse_expression(r_bp)?;

            lhs = Expression::Binary {
                left: Box::new(lhs),
                op: op,
                right: Box::new(rhs),
            }
        }

        Ok(lhs)
    }

    fn parse_primary(&mut self) -> Result<Expression, ParserError> {
        match self.current() {
            Some(Token::Number(n)) => {
                let val = *n;
                self.advance();
                Ok(Expression::Number(val))
            }
            Some(Token::String(s)) => {
                let val = s.to_string();
                self.advance();
                Ok(Expression::String(val))
            }
            Some(Token::Identifier(id)) => {
                let val = id.to_string();
                self.advance();
                Ok(Expression::Identifier(val))
            }
            Some(Token::OpenParenthesis) => {
                self.advance();
                let expr = self.parse_expression(0)?;
                match self.current() {
                    Some(Token::CloseParenthesis) => {
                        self.advance();
                        Ok(expr)
                    }
                    _ => Err(ParserError::InvalidExpression),
                }
            }

            Some(Token::OpenBrace) => self.parse_object(),

            Some(Token::Operator(MathOperator::Sub)) => {
                self.advance();
                let expr = self.parse_expression(5)?;
                Ok(Expression::Unary {
                    op: MathOperator::Sub,
                    expr: Box::new(expr),
                })
            }
            _ => Err(ParserError::InvalidExpression),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, ParserError> {
        let mut stmts = Vec::new();

        while self.current().is_some() && !matches!(self.current(), Some(Token::EOF)) {
            stmts.push(self.parse_statement()?);
        }

        Ok(stmts)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{Expression, Statement},
        lexer::Lexer,
        parser::Parser,
        token::RequestMethod,
    };

    // Helpers
    fn parse(input: &str) -> Vec<Statement> {
        Parser::new(Lexer::new(input).tokenize().unwrap())
            .parse()
            .unwrap()
    }

    fn parse_req(input: &str) -> Statement {
        Parser::new(Lexer::new(input).tokenize().unwrap())
            .parse_request()
            .unwrap()
    }

    fn parse_req_err(input: &str) -> bool {
        Parser::new(Lexer::new(input).tokenize().unwrap())
            .parse_request()
            .is_err()
    }

    // Assignment
    #[test]
    fn assignment_number_should_parse() {
        let ast = Parser::new(Lexer::new("port = 3000").tokenize().unwrap())
            .parse_assignment()
            .unwrap();

        assert_eq!(
            ast,
            Statement::Assignment {
                name: "port".into(),
                value: Expression::Number(3000)
            }
        );
    }

    #[test]
    fn assignment_string_should_parse() {
        let ast = Parser::new(
            Lexer::new("url = \"http://localhost:3000\"")
                .tokenize()
                .unwrap(),
        )
        .parse_assignment()
        .unwrap();

        assert_eq!(
            ast,
            Statement::Assignment {
                name: "url".into(),
                value: Expression::String("http://localhost:3000".into())
            }
        );
    }
    // Request
    #[test]
    fn request_basic_should_parse() {
        let ast = parse_req(r#"@GET test("http://localhost"){}"#);

        assert_eq!(
            ast,
            Statement::Request {
                method: RequestMethod::GET,
                name: "test".into(),
                args: vec![Expression::String("http://localhost".into())],
                options: vec![],
            }
        );
    }

    #[test]
    fn request_identifier_arg_should_parse() {
        let ast = parse_req("@GET test(url){}");

        match ast {
            Statement::Request { args, .. } => {
                assert!(matches!(args[0], Expression::Identifier(_)));
            }
            _ => panic!("Expected request"),
        }
    }

    #[test]
    fn request_multiple_args_should_parse() {
        let ast = parse_req(r#"@GET test(url, "v1", 123){}"#);

        match ast {
            Statement::Request { args, .. } => {
                assert_eq!(args.len(), 3);
            }
            _ => panic!("Expected request"),
        }
    }

    #[test]
    fn request_expression_arg_should_parse() {
        let ast = parse_req(r#"@GET test(1 + 2 * 3){}"#);

        match ast {
            Statement::Request { args, .. } => {
                assert!(matches!(args[0], Expression::Binary { .. }));
            }
            _ => panic!("Expected request"),
        }
    }
    // Options
    #[test]
    fn request_multiple_options_should_parse() {
        let ast = parse_req(
            r#"
            @GET test(url){
                query {"a": "1"}
                headers {"Auth": "Bearer"}
                retry 3
                timeout 100
            }
        "#,
        );

        match ast {
            Statement::Request { options, .. } => {
                assert_eq!(options.len(), 4);
            }
            _ => panic!("Expected request"),
        }
    }

    // #[test]
    // fn request_body_expression_should_parse() {
    //     let ast = parse_req(
    //         r#"
    //         @POST test(url){
    //             body {
    //                 "count": 1 + 2 * 3
    //             }
    //         }
    //     "#,
    //     );

    //     match ast {
    //         Statement::Request { options, .. } => match &options[0] {
    //             RequestOption::Body(body) => {
    //                 assert!(matches!(body[0].1, Expression::Binary { .. }));
    //             }
    //             _ => panic!("Expected body"),
    //         },
    //         _ => panic!("Expected request"),
    //     }
    // }

    // #[test]
    // fn request_empty_query_should_parse() {
    //     let ast = parse_req(
    //         r#"
    //         @GET test(url){
    //             query {}
    //         }
    //     "#,
    //     );

    //     match ast {
    //         Statement::Request { options, .. } => match &options[0] {
    //             RequestOption::Query(q) => assert!(q.is_empty()),
    //             _ => panic!("Expected query"),
    //         },
    //         _ => panic!("Expected request"),
    //     }
    // }
    // Error Cases
    #[test]
    fn error_missing_arg_should_fail() {
        assert!(parse_req_err("@GET test()"));
    }

    #[test]
    fn error_invalid_syntax_should_fail() {
        assert!(parse_req_err("@GET (url){}"));
    }

    #[test]
    fn error_missing_colon_should_fail() {
        let input = r#"
        @GET test(url){
            query {
                "a" "1"
            }
        }
        "#;

        assert!(parse_req_err(input));
    }
    // Statements
    #[test]
    fn multiple_statements_should_parse() {
        let ast = parse(
            r#"
            port = 3000
            url = "http://localhost:3000"

            @GET get_user(url){}
        "#,
        );

        assert_eq!(ast.len(), 3);

        assert!(matches!(ast[0], Statement::Assignment { .. }));
        assert!(matches!(ast[2], Statement::Request { .. }));
    }
    // Expression
    #[test]
    fn arithmetic_should_follow_precedence() {
        let ast = Parser::new(Lexer::new("5 + 2 * 9").tokenize().unwrap())
            .parse_expression(0)
            .unwrap();

        assert!(matches!(ast, Expression::Binary { .. }));
    }

    #[test]
    fn unary_binary_mix_should_parse() {
        let ast = Parser::new(Lexer::new("-5 * (2 + 3)").tokenize().unwrap())
            .parse_expression(0)
            .unwrap();

        assert!(matches!(ast, Expression::Binary { .. }));
    }
}
