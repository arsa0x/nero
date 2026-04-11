use std::{
    iter::Peekable,
    str::{Chars, FromStr},
};

use crate::token::{Keyword, MathOperator, RequestMethod, Token};

/// A lexical analysis for the Nero
///
/// The `Lexer` is responsible for converting raw source code into a sequence
/// of tokens thar will be consumed by the parser
///
/// It processes the input character by character and produces tokens such as:
/// identifier, keywords, literals ans symbols
pub struct Lexer<'a> {
    /// Current byte position in the source string
    pub pos: usize,

    /// Iterator over the characters of the source
    pub chars: Peekable<Chars<'a>>,

    /// Original source code
    pub source: &'a str,
}

/// Represents error that can occur during lexical analysis
#[derive(Debug, PartialEq)]
pub enum LexerError {
    /// A string literal was not properly closed
    ///
    /// Contains the starting position of the string
    UnclosedString(usize),

    /// An unexpected character was encountered
    ///
    /// Contains the character and its position
    UnexpectedCharacter(String, usize),

    /// An invalid HTTP method was found after '@'
    ///
    /// Contains the character and its position
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
    /// Create a new `Lexer` instance from the given source code
    ///
    /// # Arguments
    /// - `source` - the input string to be tokenized
    ///
    /// # Example
    /// ```
    /// use nero::lexer::Lexer;
    ///
    /// let lexer = Lexer::new("url = \"http://127.0.0.1\"");
    /// ```
    pub fn new(source: &'a str) -> Self {
        Self {
            pos: 0,
            source,
            chars: source.chars().peekable(),
        }
    }

    /// Extracts a slice from the source string starting at `start` up to the
    /// current lexer position
    ///
    /// # Arguments
    /// - `start` - the starting byte index of the slice
    ///
    /// # Returns
    /// Returns a string slice from the `start` position to the current position
    fn next_slice(&mut self, start: usize) -> &'a str {
        &self.source[start..self.pos]
    }

    /// Advances the lexer to the next character and updates byte position
    fn advance(&mut self) {
        let next = self.chars.next();
        if let Some(c) = next {
            self.pos += c.len_utf8();
        }
    }

    /// Consumes characters while the given condition is true
    ///
    /// # Arguments
    /// - `test` - a predicate function used to determine whether
    ///            the current character should be consumed
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

    /// Reads a string literal fron the source.
    ///
    /// Assumes the current character is a double quote (`"`)
    ///
    /// Returns
    /// Returns the string slice without the surrounding quotes
    ///
    /// # Error
    /// Returns `LexerError::UnclosedString` if the string is not properly closed
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

    /// Tokenize the entire source input into a sequence of tokens
    ///
    /// This function iterates through the source code and produces tokens such as:
    /// - identifiers
    /// - keywords
    /// - string literals
    /// - numbers
    /// - HTTP methods
    ///
    /// # Returns
    /// A vector of tokens is successful
    ///
    /// # Errors
    /// - `UnexpectedCharacter` if an invalid character is encountered
    /// - `UnclosedString` if a string is not closed
    /// - `InvalidMethod` if an unknown HTTP mathod is used
    ///
    /// # Example
    /// ```
    /// use nero::lexer::Lexer;
    ///
    /// let mut lexer = Lexer::new("url = \"http://localhost\"");
    /// let tokens = lexer.tokenize().unwrap();
    /// ```
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
                '+' => {
                    tokens.push(Token::Operator(MathOperator::Add));
                    self.advance();
                    continue;
                }
                '-' => {
                    tokens.push(Token::Operator(MathOperator::Sub));
                    self.advance();
                    continue;
                }
                '*' => {
                    tokens.push(Token::Operator(MathOperator::Mul));
                    self.advance();
                    continue;
                }
                '/' => {
                    tokens.push(Token::Operator(MathOperator::Div));
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
                _ => {
                    return Err(LexerError::UnexpectedCharacter(ch.to_string(), self.pos));
                }
            }
        }
        tokens.push(Token::EOF);
        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn assignment_test() {
        let mut lexer = Lexer::new("url = \"http://127.0.0.1\"");
        let token = lexer.tokenize().unwrap();

        assert_eq!(
            token,
            vec![
                Token::Identifier("url"),
                Token::Operator(MathOperator::Add),
                Token::String("http://127.0.0.1"),
                Token::EOF
            ]
        );
    }

    #[test]
    fn keyword_test() {
        let mut lexer = Lexer::new("body headers query timeout retry");
        let token = lexer.tokenize().unwrap();

        assert_eq!(
            token,
            vec![
                Token::Keyword(Keyword::Body),
                Token::Keyword(Keyword::Headers),
                Token::Keyword(Keyword::Query),
                Token::Keyword(Keyword::Timeout),
                Token::Keyword(Keyword::Retry),
                Token::EOF,
            ]
        );
    }

    #[test]
    fn error_unclosed_string_test() {
        let mut lexer = Lexer::new("\"hello, world!");
        let token = lexer.tokenize();
        assert!(token.is_err());
    }

    #[test]
    fn method_test() {
        let mut lexer = Lexer::new("@GET @POST @put @patch @DELete");
        let token = lexer.tokenize().unwrap();
        assert_eq!(
            token,
            vec![
                Token::Method(RequestMethod::GET),
                Token::Method(RequestMethod::POST),
                Token::Method(RequestMethod::PUT),
                Token::Method(RequestMethod::PATCH),
                Token::Method(RequestMethod::DELETE),
                Token::EOF,
            ]
        );
    }

    #[test]
    fn error_unknown_method_test() {
        let mut lexer = Lexer::new("@TEG");
        let token = lexer.tokenize();
        assert!(token.is_err());
    }
}
