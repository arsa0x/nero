use crate::{
    ast::Expression,
    token::{MathOperator, Token},
};

pub struct Parser<'a> {
    pub tokens: Vec<Token<'a>>,
    pub pos: usize,
}

#[derive(Debug, PartialEq)]
pub enum ParserError {
    InvalidExpression,
    UnexpectedToken,
    UnexpectedEOF,
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

    fn get_binding_power(&self, op: &MathOperator) -> (u8, u8) {
        match op {
            MathOperator::Add | MathOperator::Sub => (1, 2),
            MathOperator::Mul | MathOperator::Div => (3, 4),
        }
    }

    fn parse_expression(&mut self, min_bp: u8) -> Result<Expression, ParserError> {
        let mut lhs = self.parse_primary()?;

        loop {
            let op = match self.current() {
                Some(Token::Operator(op)) => op.clone(),
                _ => break,
            };

            let (l_bp, r_bp) = self.get_binding_power(&op);
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

    pub fn parse(&mut self) -> Result<Expression, ParserError> {
        let expr = self.parse_expression(0)?;
        // if self.current().is_some() {
        //     return Err(ParserError::UnexpectedToken);
        // }

        Ok(expr)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::Expression, lexer::Lexer, parser::Parser, token::MathOperator};

    #[test]
    fn aritmethic_parser_test() {
        let case_1 = Parser::new(Lexer::new("5 + 2 * 9").tokenize().unwrap())
            .parse()
            .unwrap();
        let expected_1 = Expression::Binary {
            left: Box::new(Expression::Number(5)),
            op: MathOperator::Add,
            right: Box::new(Expression::Binary {
                left: Box::new(Expression::Number(2)),
                op: MathOperator::Mul,
                right: Box::new(Expression::Number(9)),
            }),
        };

        let case_2 = Parser::new(Lexer::new("10 / 2 - 8").tokenize().unwrap())
            .parse()
            .unwrap();
        let expected_2 = Expression::Binary {
            left: Box::new(Expression::Binary {
                left: Box::new(Expression::Number(10)),
                op: MathOperator::Div,
                right: Box::new(Expression::Number(2)),
            }),
            op: MathOperator::Sub,
            right: Box::new(Expression::Number(8)),
        };

        let case_3 = Parser::new(Lexer::new("18 - 6 + 9 / 3").tokenize().unwrap())
            .parse()
            .unwrap();
        let expected_3 = Expression::Binary {
            left: Box::new(Expression::Binary {
                left: Box::new(Expression::Number(18)),
                op: MathOperator::Sub,
                right: Box::new(Expression::Number(6)),
            }),
            op: MathOperator::Add,
            right: Box::new(Expression::Binary {
                left: Box::new(Expression::Number(9)),
                op: MathOperator::Div,
                right: Box::new(Expression::Number(3)),
            }),
        };

        let case_4a = Parser::new(Lexer::new("18 - ( 6 + 9 ) / 3").tokenize().unwrap())
            .parse()
            .unwrap();
        let case_4b = Parser::new(Lexer::new("18 - 6 + 9 / 3").tokenize().unwrap())
            .parse()
            .unwrap();

        let case_5 = Parser::new(Lexer::new("-4 + 2").tokenize().unwrap())
            .parse()
            .unwrap();
        let expected_5 = Expression::Binary {
            left: Box::new(Expression::Unary {
                op: MathOperator::Sub,
                expr: Box::new(Expression::Number(4)),
            }),
            op: MathOperator::Add,
            right: Box::new(Expression::Number(2)),
        };

        assert_eq!(case_1, expected_1);

        assert_eq!(case_2, expected_2);

        assert_eq!(case_3, expected_3);

        assert_ne!(case_4a, case_4b);

        assert_eq!(case_5, expected_5);
    }
}
