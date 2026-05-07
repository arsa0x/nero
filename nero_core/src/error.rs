#[derive(Debug, PartialEq)]
pub enum Error {
    LexerError(LexerError),
    ParserError(ParserError),
    ExecError(ExecError),
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

#[derive(Debug, PartialEq)]
pub enum ParserError {
    InvalidExpression,
    InvalidStatement,
    UnexpectedToken,
    UnexpectedEOF,
}

#[derive(Debug, PartialEq)]
pub enum ExecError {
    MissingUrl,
    InvalidExpression,
    RequestFailed,
}
