use std::{
    iter::Peekable,
    panic,
    str::{Chars, FromStr},
};

use crate::token::{Keyword, RequestMethod, Token};

pub struct Lexer<'a> {
    pub pos: usize,
    pub chars: Peekable<Chars<'a>>,
    pub source: &'a str,
}

#[derive(Debug, PartialEq)]
pub enum LexerError {
    UnclosedString(usize),
    UnexpectedCharacter(String, usize),
    InvalidMethod(String, usize),
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::UnclosedString(u) => write!(f, "unclosed string at position: {}", u),
            LexerError::UnexpectedCharacter(s, u) => {
                write!(f, "unexpected character {} at position {}", s, u)
            }
            LexerError::InvalidMethod(s, u) => {
                write!(f, "invalid method {} at position {}", s, u)
            }
        }
    }
}

impl std::error::Error for LexerError {}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            pos: 0,
            source,
            chars: source.chars().peekable(),
        }
    }

    fn next_slice(&mut self, start: usize) -> &'a str {
        &self.source[start..self.pos]
    }

    fn advance(&mut self) {
        let next = self.chars.next();
        if let Some(c) = next {
            self.pos += c.len_utf8();
        }
    }

    fn read_while<T>(&mut self, test: T)
    where
        T: Fn(char) -> bool,
    {
        while let Some(&ch) = self.chars.peek() {
            if test(ch) {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_string(&mut self) -> Result<&'a str, LexerError> {
        self.advance();

        let start = self.pos;
        while let Some(&ch) = self.chars.peek() {
            if ch == '"' {
                let s = self.next_slice(start);
                self.advance();
                return Ok(s);
            }
            self.advance();
        }
        Err(LexerError::UnclosedString(start))
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token<'a>>, LexerError> {
        let mut tokens = Vec::new();

        while let Some(&ch) = self.chars.peek() {
            if ch.is_whitespace() {
                self.advance();
                continue;
            }

            let start: usize = self.pos;

            if ch.is_numeric() {
                self.read_while(|c| c.is_numeric());
                let s = self.next_slice(start);

                if let Some(&next_ch) = self.chars.peek() {
                    if next_ch.is_alphabetic() || next_ch == '=' {
                        return Err(LexerError::UnexpectedCharacter(
                            next_ch.to_string(),
                            self.pos,
                        ));
                    }
                }

                tokens.push(Token::Number(s.parse().unwrap()));
                continue;
            }

            if ch.is_alphabetic() {
                self.read_while(|c| c == '_' || c.is_ascii_alphanumeric());

                let s = self.next_slice(start);

                if let Ok(k) = Keyword::from_str(s) {
                    tokens.push(Token::Keyword(k));
                } else {
                    tokens.push(Token::Identifier(s));
                }

                continue;
            }

            match ch {
                '"' => {
                    tokens.push(Token::String(self.read_string()?));
                    continue;
                }
                '@' => {
                    self.advance();
                    let start_m = self.pos;
                    self.read_while(|c| c.is_alphabetic());

                    let m = self.next_slice(start_m);

                    tokens
                        .push(Token::Method(RequestMethod::from_str(m).map_err(|_| {
                            LexerError::InvalidMethod(m.to_string(), start_m)
                        })?));

                    continue;
                }
                ',' => {
                    tokens.push(Token::Comma);
                    self.advance();
                    continue;
                }
                '=' => {
                    tokens.push(Token::Assignment);
                    self.advance();
                    continue;
                }
                '(' => {
                    tokens.push(Token::OpenParenthesis);
                    self.advance();
                    continue;
                }
                ')' => {
                    tokens.push(Token::CloseParenthesis);
                    self.advance();
                    continue;
                }
                '{' => {
                    tokens.push(Token::OpenBrace);
                    self.advance();
                    continue;
                }
                '}' => {
                    tokens.push(Token::CloseBrace);
                    self.advance();
                    continue;
                }
                ':' => {
                    tokens.push(Token::Colon);
                    self.advance();
                    continue;
                }
                ';' => {
                    tokens.push(Token::Semicolon);
                    self.advance();
                    continue;
                }
                // '-' => {
                //     self.advance();

                //     if let Some('>') = self.chars.peek() {
                //         self.advance();
                //         let start_r = self.pos;
                //         self.read_while(|c| c.is_alphabetic());

                //         tokens.push(Token::Return(self.next_slice(start_r)));
                //     } else {
                //         return Err(LexerError::UnexpectedCharacter('-'.to_string(), start));
                //     }

                //     continue;
                // }
                _ => {
                    return Err(LexerError::UnexpectedCharacter(ch.to_string(), self.pos));
                }
            }
            // self.advance();
        }
        tokens.push(Token::EOF);
        Ok(tokens)
    }
}
