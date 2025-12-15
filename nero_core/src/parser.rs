use crate::{
    ast::{Expr, Req, Stmt, StringPart},
    token::Token,
};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub pos: usize,
}

#[derive(Debug)]
pub enum ParserError {
    InvalidExpression,
    UnexpectedEOF,
    UnexpectedToken { expected: Token, found: Token },
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::InvalidExpression => {
                write!(f, "Invalid expression")
            }
            ParserError::UnexpectedEOF => {
                write!(f, "Unexpected Enf of File")
            }
            ParserError::UnexpectedToken { expected, found } => {
                write!(f, "Expected: {:?}, found: {:?}", expected, found)
            }
        }
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    /// Helper untuk mengambil token saat ini
    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    /// Helper untuk memindahkan parser ke token berikutnya
    fn advance(&mut self) {
        self.pos += 1;
    }

    /// Helper untuk
    fn consume(&mut self, expected: &Token) -> Result<(), ParserError> {
        if let Some(t) = self.current() {
            if t == expected {
                self.advance();
                Ok(())
            } else {
                Err(ParserError::UnexpectedToken {
                    expected: expected.clone(),
                    found: t.clone(),
                })
            }
        } else {
            Err(ParserError::UnexpectedEOF)
        }
    }

    /// Fungsi untuk membantu parse sebuah statement
    fn parse_statement(&mut self) -> Result<Stmt, ParserError> {
        match self.current() {
            Some(Token::Identifier(_)) => self.parse_assignment(),
            Some(Token::Hash) | Some(Token::At) => self.parse_request(),
            Some(t) => Err(ParserError::UnexpectedToken {
                expected: Token::Identifier("statement".into()),
                found: t.clone(),
            }),
            None => Err(ParserError::UnexpectedEOF),
        }
    }

    fn parse_interpolated_string(&mut self) -> Result<Expr, ParserError> {
        let mut parts: Vec<StringPart> = Vec::new();

        loop {
            match self.current() {
                Some(Token::StringLiteral(t)) => {
                    parts.push(StringPart::Text(t.clone()));
                    self.advance();
                }
                Some(Token::TemplateStart) => {
                    self.advance();

                    let expr = match self.current() {
                        Some(Token::Identifier(name)) => {
                            let e = Expr::Identifier(name.clone());
                            self.advance();
                            e
                        }
                        _ => return Err(ParserError::InvalidExpression),
                    };
                    self.consume(&Token::TemplateEnd)?;
                    parts.push(StringPart::Expression(expr));
                }
                _ => break,
            }
        }
        Ok(Expr::String(parts))
    }

    fn parse_expression(&mut self) -> Result<Expr, ParserError> {
        match self.current() {
            // number literal
            Some(Token::NumberLiteral(n)) => {
                let val = *n;
                self.advance();
                Ok(Expr::Number(val))
            }

            // identifier
            Some(Token::Identifier(name)) => {
                let id = name.clone();
                self.advance();
                Ok(Expr::Identifier(id))
            }
            // string template (interpolated string)
            Some(Token::StringLiteral(_)) => self.parse_interpolated_string(),
            _ => Err(ParserError::InvalidExpression),
        }
    }

    /// Fungsi untuk parse operator penugasan (assignment)
    ///
    /// # Grammar
    /// `assignment = Identifier "=" expr ";"`
    ///
    fn parse_assignment(&mut self) -> Result<Stmt, ParserError> {
        let name = if let Some(Token::Identifier(name)) = self.current() {
            name.clone()
        } else {
            return Err(ParserError::InvalidExpression);
        };
        self.advance();
        self.consume(&Token::Equals)?;
        let value = self.parse_expression()?;
        self.consume(&Token::SemiColon)?;

        Ok(Stmt::Assignment { name, value })
    }

    /// Fungsi untuk parse request
    ///
    /// # Grammar
    /// request = label? "@" Identifier expr block
    ///
    fn parse_request(&mut self) -> Result<Stmt, ParserError> {
        let label: Option<String> = if matches!(self.current(), Some(Token::Hash)) {
            self.advance();
            self.consume(&Token::OpenBracket)?;

            let name = if let Some(Token::Label(name)) = self.current() {
                name.clone()
            } else {
                return Err(ParserError::InvalidExpression);
            };
            self.advance();
            self.consume(&Token::CloseBracket)?;
            Some(name)
        } else {
            None
        };

        self.consume(&Token::At)?;
        let method = if let Some(Token::Identifier(m)) = self.current() {
            m.clone()
        } else {
            return Err(ParserError::InvalidExpression);
        };

        self.advance();
        let url = self.parse_expression()?;

        self.consume(&Token::OpenBrace)?;
        while !matches!(self.current(), Some(Token::CloseBrace)) {
            // todo
            self.advance();
        }
        self.consume(&Token::CloseBrace)?;

        Ok(Stmt::Request(Req {
            label,
            method,
            url,
            headers: vec![],
            query: vec![],
            body: None,
        }))
    }

    /// Entry point fungsi parser
    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParserError> {
        let mut statements: Vec<Stmt> = Vec::new();

        while self.pos < self.tokens.len() {
            statements.push(self.parse_statement()?);
        }

        Ok(statements)
    }
}
