use crate::token::Span;

/// Represents all lexical analysis errors that can occur during tokenization.
///
/// Lexer errors are produced when the source code contains invalid
/// or malformed character sequences that cannot be converted into tokens.
#[derive(Debug, thiserror::Error)]
pub enum LexerError {
    /// Raised when a string literal is started but never properly terminated.
    ///
    /// This typically occurs when a closing quotation mark (`"`)
    /// is missing before the end of the file.
    ///
    /// # Example
    ///
    /// ```text
    /// "hello world
    /// ```
    #[error("unclosed string literal")]
    UnclosedString {
        /// The source span where the unterminated string originated.
        span: Span,
    },

    /// Raised when the lexer encounters a character
    /// that is not recognized by the language grammar.
    ///
    /// # Example
    ///
    /// ```text
    /// $
    /// ^
    /// ```
    #[error("unexpected character '{character}'")]
    UnexpectedCharacter {
        /// The unexpected character encountered by the lexer.
        character: char,

        /// The source span where the invalid character appeared.
        span: Span,
    },

    /// Raised when an invalid or unsupported HTTP method is encountered.
    ///
    /// Request methods are expected to follow the `@METHOD` syntax.
    ///
    /// # Example
    ///
    /// ```text
    /// @FETCH
    /// ```
    #[error("invalid method '{method}'")]
    InvalidMethod {
        /// The invalid method string encountered by the lexer.
        method: String,

        /// The source span where the invalid method appeared.
        span: Span,
    },

    /// Raised when a numeric literal cannot be parsed correctly.
    ///
    /// This may occur due to malformed integer or floating-point syntax.
    ///
    /// # Examples
    ///
    /// ```text
    /// 12.34.56
    /// ```
    ///
    /// ```text
    /// 999999999999999999999999999999
    /// ```
    #[error("invalid number")]
    InvalidNumber {
        /// The source span where the invalid number was detected.
        span: Span,
    },

    /// Raised when a string interpolation block is started
    /// but never properly closed.
    ///
    /// Interpolations must end with a closing brace (`}`).
    ///
    /// # Example
    ///
    /// ```text
    /// "Hello ${name"
    /// ```
    #[error("unclosed interpolation")]
    UnclosedInterpolation {
        /// The source span where the unterminated interpolation started.
        span: Span,
    },
}
