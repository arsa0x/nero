use crate::{
    Result,
    error::Error,
    errors::lexer_error::LexerError,
    token::{Keyword, Operator, RequestMethod, Span, Token, TokenKind},
};

use std::{
    iter::Peekable,
    str::{Chars, FromStr},
};

/// Represents the current position of the lexer inside the source code.
///
/// This structure is mainly used to create accurate spans for tokens
/// and error reporting.
#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    /// Absolute byte position in the source string.
    pub pos: usize,

    /// Current line number (1-based).
    pub line: usize,

    /// Current column number (1-based).
    pub column: usize,
}

/// A lexical analyzer responsible for converting raw source code
/// into a sequence of tokens.
///
/// The lexer processes the input character by character and produces
/// tokens that can later be consumed by the parser.
pub struct Lexer<'a> {
    /// Original source code.
    source: &'a str,

    /// Iterator over the source characters with peek support.
    chars: Peekable<Chars<'a>>,

    /// Current byte position in the source.
    pos: usize,

    /// Current line number.
    line: usize,

    /// Current column number.
    column: usize,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer instance from the provided source code.
    ///
    /// # Arguments
    ///
    /// * `source` - Raw source code to tokenize.
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars().peekable(),

            pos: 0,
            line: 1,
            column: 1,
        }
    }

    /// Returns the current cursor position.
    ///
    /// This is commonly used before scanning a token so the lexer
    /// can later generate an accurate span.
    fn cursor(&self) -> Cursor {
        Cursor {
            pos: self.pos,
            line: self.line,
            column: self.column,
        }
    }

    /// Creates a [`Span`] from a starting cursor up to the current position.
    ///
    /// # Arguments
    ///
    /// * `start` - The cursor position where the token started.
    fn span(&self, start: Cursor) -> Span {
        Span {
            start: start.pos,
            end: self.pos,
            line: start.line,
            column: start.column,
        }
    }

    /// Constructs a token with the provided kind and span information.
    ///
    /// # Arguments
    ///
    /// * `kind` - The token kind to create.
    /// * `start` - Cursor position where the token started.
    fn make_token(&self, kind: TokenKind, start: Cursor) -> Token {
        Token {
            kind,
            span: self.span(start),
        }
    }

    /// Returns the current character without consuming it.
    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    /// Consumes the next character and advances the lexer position.
    ///
    /// This method also updates line and column tracking.
    fn advance(&mut self) -> Option<char> {
        let ch = self.chars.next()?;

        self.pos += ch.len_utf8();

        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        Some(ch)
    }

    /// Checks whether the remaining source starts with the given string.
    ///
    /// # Arguments
    ///
    /// * `s` - String pattern to compare against.
    fn starts_with(&self, s: &str) -> bool {
        self.source[self.pos..].starts_with(s)
    }

    /// Returns a slice of the source from `start` up to the current position.
    ///
    /// # Arguments
    ///
    /// * `start` - Starting byte offset.
    fn slice_from(&self, start: usize) -> &'a str {
        &self.source[start..self.pos]
    }

    /// Continues consuming characters while the predicate returns `true`.
    ///
    /// # Arguments
    ///
    /// * `test` - Predicate used to determine whether scanning should continue.
    fn read_while<F>(&mut self, test: F)
    where
        F: Fn(char) -> bool,
    {
        while let Some(ch) = self.peek() {
            if test(ch) {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Consumes the next character only if it matches the expected value.
    ///
    /// Returns `true` if the character matched and was consumed.
    ///
    /// # Arguments
    ///
    /// * `expected` - Character expected at the current position.
    fn match_char(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Scans an identifier or keyword token.
    ///
    /// Identifiers may contain:
    /// - ASCII alphabetic characters
    /// - Digits
    /// - Underscores
    ///
    /// If the scanned text matches a reserved keyword,
    /// a keyword token is returned instead.
    fn scan_identifier_or_keyword(&mut self) -> TokenKind {
        let start = self.pos;

        self.read_while(|c| c.is_ascii_alphanumeric() || c == '_');

        let text = self.slice_from(start);

        if let Ok(keyword) = Keyword::from_str(text) {
            TokenKind::Keyword(keyword)
        } else if text == "true" {
            TokenKind::Boolean(true)
        } else if text == "false" {
            TokenKind::Boolean(false)
        } else {
            TokenKind::Identifier(text.to_string())
        }
    }

    /// Scans either an integer or floating-point number.
    ///
    /// # Errors
    ///
    /// Returns [`LexerError::InvalidNumber`] if the parsed number
    /// cannot be converted into its numeric representation.
    fn scan_number(&mut self) -> Result<TokenKind> {
        let start = self.pos;

        self.read_while(|c| c.is_ascii_digit());

        let mut clone = self.chars.clone();

        if let Some('.') = clone.next() {
            if let Some(next) = clone.next() {
                if next.is_ascii_digit() {
                    self.advance();

                    self.read_while(|c| c.is_ascii_digit());

                    let value = self.slice_from(start).parse::<f64>().map_err(|_| {
                        Error::Lexer(LexerError::InvalidNumber {
                            span: self.span(self.cursor()),
                        })
                    })?;

                    return Ok(TokenKind::Float(value));
                }
            }
        }

        let value = self.slice_from(start).parse::<i64>().map_err(|_| {
            Error::Lexer(LexerError::InvalidNumber {
                span: self.span(self.cursor()),
            })
        })?;

        Ok(TokenKind::Number(value))
    }

    /// Scans an HTTP request method token prefixed with `@`.
    ///
    /// Example:
    /// ```text
    /// @GET
    /// @POST
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`LexerError::InvalidMethod`] if the method
    /// is not recognized.
    fn scan_method(&mut self) -> Result<TokenKind> {
        let start = self.cursor();

        self.advance();

        let method_start = self.pos;

        self.read_while(|c| c.is_ascii_alphabetic());

        let method_str = self.slice_from(method_start);

        let method = RequestMethod::from_str(method_str).map_err(|_| {
            Error::Lexer(LexerError::InvalidMethod {
                method: method_str.to_string(),
                span: self.span(start),
            })
        })?;

        Ok(TokenKind::Method(method))
    }

    /// Scans and skips a single-line comment.
    ///
    /// Comments start with `#` and continue until the end of the line.
    fn scan_comment(&mut self) {
        self.advance();

        self.read_while(|c| c != '\n');
    }

    /// Scans a string literal with interpolation support.
    ///
    /// Supported interpolation syntax:
    /// ```text
    /// "Hello ${name}"
    /// ```
    ///
    /// This method emits:
    /// - `StringStart`
    /// - `StringText`
    /// - `InterpolationStart`
    /// - expression tokens
    /// - `InterpolationEnd`
    /// - `StringEnd`
    ///
    /// # Errors
    ///
    /// Returns:
    /// - [`LexerError::UnclosedString`]
    /// - [`LexerError::UnclosedInterpolation`]
    /// - [`LexerError::UnexpectedCharacter`]
    fn scan_string(&mut self, tokens: &mut Vec<Token>) -> Result<()> {
        let quote_start = self.cursor();

        self.advance();

        tokens.push(self.make_token(TokenKind::StringStart, quote_start));

        let mut text_start = self.cursor();

        while let Some(ch) = self.peek() {
            if ch == '"' {
                if text_start.pos != self.pos {
                    let text = self.slice_from(text_start.pos);

                    tokens
                        .push(self.make_token(TokenKind::StringText(text.to_string()), text_start));
                }

                let end_start = self.cursor();

                self.advance();

                tokens.push(self.make_token(TokenKind::StringEnd, end_start));

                return Ok(());
            }

            if self.starts_with("${") {
                if text_start.pos != self.pos {
                    let text = self.slice_from(text_start.pos);

                    tokens
                        .push(self.make_token(TokenKind::StringText(text.to_string()), text_start));
                }

                let interp_start = self.cursor();

                self.advance();
                self.advance();

                tokens.push(self.make_token(TokenKind::InterpolationStart, interp_start));

                while let Some(ch) = self.peek() {
                    if ch == '}' {
                        break;
                    }

                    let start = self.cursor();

                    match ch {
                        c if c.is_whitespace() => {
                            self.advance();
                        }

                        c if c.is_ascii_digit() => {
                            let kind = self.scan_number()?;

                            tokens.push(self.make_token(kind, start));
                        }

                        c if c.is_ascii_alphabetic() || c == '_' => {
                            let kind = self.scan_identifier_or_keyword();

                            tokens.push(self.make_token(kind, start));
                        }

                        '+' => {
                            self.advance();

                            tokens
                                .push(self.make_token(TokenKind::Operator(Operator::Plus), start));
                        }

                        '-' => {
                            self.advance();

                            tokens
                                .push(self.make_token(TokenKind::Operator(Operator::Minus), start));
                        }

                        '*' => {
                            self.advance();

                            tokens
                                .push(self.make_token(TokenKind::Operator(Operator::Star), start));
                        }

                        '/' => {
                            self.advance();

                            tokens
                                .push(self.make_token(TokenKind::Operator(Operator::Slash), start));
                        }

                        '.' => {
                            self.advance();

                            tokens.push(self.make_token(TokenKind::Dot, start));
                        }

                        '(' => {
                            self.advance();

                            tokens.push(self.make_token(TokenKind::LParen, start));
                        }

                        ')' => {
                            self.advance();

                            tokens.push(self.make_token(TokenKind::RParen, start));
                        }

                        '[' => {
                            self.advance();

                            tokens.push(self.make_token(TokenKind::LBracket, start));
                        }

                        ']' => {
                            self.advance();

                            tokens.push(self.make_token(TokenKind::RBracket, start));
                        }

                        _ => {
                            return Err(Error::Lexer(LexerError::UnexpectedCharacter {
                                character: ch,
                                span: self.span(start),
                            }));
                        }
                    }
                }

                if self.peek() != Some('}') {
                    return Err(Error::Lexer(LexerError::UnclosedInterpolation {
                        span: self.span(interp_start),
                    }));
                }

                let interp_end = self.cursor();

                self.advance();

                tokens.push(self.make_token(TokenKind::InterpolationEnd, interp_end));

                text_start = self.cursor();

                continue;
            }

            self.advance();
        }

        Err(Error::Lexer(LexerError::UnclosedString {
            span: self.span(quote_start),
        }))
    }

    /// Converts the entire source code into a sequence of tokens.
    ///
    /// This is the main entry point of the lexer.
    ///
    /// # Returns
    ///
    /// A vector containing all generated tokens, including the final `EOF` token.
    ///
    /// # Errors
    ///
    /// Returns lexer-related errors if invalid syntax or characters are encountered.
    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.peek() {
            let start = self.cursor();

            match ch {
                c if c.is_whitespace() => {
                    self.advance();
                }

                '#' => {
                    self.scan_comment();
                }

                c if c.is_ascii_alphabetic() || c == '_' => {
                    let kind = self.scan_identifier_or_keyword();

                    tokens.push(self.make_token(kind, start));
                }

                c if c.is_ascii_digit() => {
                    let kind = self.scan_number()?;

                    tokens.push(self.make_token(kind, start));
                }

                '"' => {
                    self.scan_string(&mut tokens)?;
                }

                '@' => {
                    let kind = self.scan_method()?;

                    tokens.push(self.make_token(kind, start));
                }

                '(' => {
                    self.advance();
                    tokens.push(self.make_token(TokenKind::LParen, start));
                }

                ')' => {
                    self.advance();
                    tokens.push(self.make_token(TokenKind::RParen, start));
                }

                '{' => {
                    self.advance();
                    tokens.push(self.make_token(TokenKind::LBrace, start));
                }

                '}' => {
                    self.advance();
                    tokens.push(self.make_token(TokenKind::RBrace, start));
                }

                '[' => {
                    self.advance();
                    tokens.push(self.make_token(TokenKind::LBracket, start));
                }

                ']' => {
                    self.advance();
                    tokens.push(self.make_token(TokenKind::RBracket, start));
                }

                ':' => {
                    self.advance();
                    tokens.push(self.make_token(TokenKind::Colon, start));
                }

                ';' => {
                    self.advance();
                    tokens.push(self.make_token(TokenKind::Semicolon, start));
                }

                ',' => {
                    self.advance();
                    tokens.push(self.make_token(TokenKind::Comma, start));
                }

                '.' => {
                    self.advance();

                    if self.match_char('.') {
                        tokens.push(self.make_token(TokenKind::Range, start));
                    } else {
                        tokens.push(self.make_token(TokenKind::Dot, start));
                    }
                }

                '+' => {
                    self.advance();

                    tokens.push(self.make_token(TokenKind::Operator(Operator::Plus), start));
                }

                '-' => {
                    self.advance();

                    if self.match_char('>') {
                        tokens.push(self.make_token(TokenKind::Arrow, start));
                    } else {
                        tokens.push(self.make_token(TokenKind::Operator(Operator::Minus), start));
                    }
                }

                '*' => {
                    self.advance();

                    tokens.push(self.make_token(TokenKind::Operator(Operator::Star), start));
                }

                '/' => {
                    self.advance();

                    tokens.push(self.make_token(TokenKind::Operator(Operator::Slash), start));
                }

                '=' => {
                    self.advance();

                    if self.match_char('=') {
                        tokens.push(self.make_token(TokenKind::Operator(Operator::Eq), start));
                    } else {
                        tokens.push(self.make_token(TokenKind::Assign, start));
                    }
                }

                '!' => {
                    self.advance();

                    if self.match_char('=') {
                        tokens.push(self.make_token(TokenKind::Operator(Operator::Ne), start));
                    } else {
                        return Err(Error::Lexer(LexerError::UnexpectedCharacter {
                            character: '!',
                            span: self.span(start),
                        }));
                    }
                }

                '<' => {
                    self.advance();

                    if self.match_char('=') {
                        tokens.push(self.make_token(TokenKind::Operator(Operator::Le), start));
                    } else {
                        tokens.push(self.make_token(TokenKind::Operator(Operator::Lt), start));
                    }
                }

                '>' => {
                    self.advance();

                    if self.match_char('=') {
                        tokens.push(self.make_token(TokenKind::Operator(Operator::Ge), start));
                    } else {
                        tokens.push(self.make_token(TokenKind::Operator(Operator::Gt), start));
                    }
                }

                _ => {
                    return Err(Error::Lexer(LexerError::UnexpectedCharacter {
                        character: ch,
                        span: self.span(start),
                    }));
                }
            }
        }

        let eof = self.cursor();

        tokens.push(Token {
            kind: TokenKind::EOF,
            span: Span {
                start: eof.pos,
                end: eof.pos,
                line: eof.line,
                column: eof.column,
            },
        });

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::*, token::*};

    #[test]
    fn tokenize_get_method_test() {
        let mut lexer = Lexer::new("@GET");

        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 2);

        let token = &tokens[0];

        assert_eq!(token.kind, TokenKind::Method(RequestMethod::GET));

        assert_eq!(token.span.start, 0);
        assert_eq!(token.span.end, 4);

        assert_eq!(token.span.line, 1);
        assert_eq!(token.span.column, 1);

        assert_eq!(tokens[1].kind, TokenKind::EOF);
    }

    #[test]
    fn tokenize_method_insensitive_test() {
        let mut lexer = Lexer::new("@post");

        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].kind, TokenKind::Method(RequestMethod::POST));
        assert_eq!(tokens[1].kind, TokenKind::EOF);
    }

    #[test]
    fn invalid_method_test() {
        let mut lexer = Lexer::new("@INVALID");

        let result = lexer.tokenize();

        assert!(result.is_err());
    }
}
