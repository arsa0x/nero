use crate::token::{Span, TokenKind};

/// Represents all parsing-related errors that can occur during syntax analysis.
///
/// Parser errors are produced when the token stream does not conform
/// to the expected grammar rules of the language.
#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    /// Raised when the parser expects a specific token
    /// but encounters a different token instead.
    ///
    /// # Example
    ///
    /// Expected:
    /// ```text
    /// )
    /// ```
    ///
    /// Found:
    /// ```text
    /// }
    /// ```
    #[error("expected token {expected:?}, found {found:?}")]
    ExpectedToken {
        /// The token kind that the parser expected to encounter.
        expected: TokenKind,

        /// The actual token kind encountered by the parser.
        found: TokenKind,

        /// The source span where the error occurred.
        span: Span,
    },

    /// Raised when the parser reaches the end of the token stream
    /// unexpectedly while still expecting additional syntax elements.
    ///
    /// This commonly occurs when delimiters such as `)`, `]`, or `}`
    /// are missing from the source code.
    ///
    /// # Example
    ///
    /// ```text
    /// foo(bar
    /// ```
    #[error("unexpected end of file")]
    UnexpectedEOF {
        /// The source span where parsing unexpectedly terminated.
        span: Span,
    },

    /// Raised when the parser encounters malformed or unsupported expressions.
    ///
    /// This error indicates that the current token sequence
    /// cannot be interpreted as a valid expression according
    /// to the language grammar.
    ///
    /// # Example
    ///
    /// ```text
    /// 1 + * 2
    /// ```
    #[error("invalid expression")]
    InvalidExpression {
        /// The source span where the invalid expression was detected.
        span: Span,
    },
}
