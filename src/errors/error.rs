use crate::errors::{lexer_error::LexerError, parser_error::ParserError};

/// Represents the top-level error type used throughout the compiler
/// or interpreter pipeline.
///
/// This enum acts as a unified error wrapper that groups together
/// all subsystem-specific errors, such as lexer and parser errors.
///
/// Using a single error type simplifies error propagation and handling
/// across the application.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Wraps errors produced during lexical analysis.
    ///
    /// Lexer errors occur while converting raw source code
    /// into a sequence of tokens.
    #[error(transparent)]
    Lexer(
        /// The underlying lexer error.
        #[from]
        LexerError,
    ),

    /// Wraps errors produced during syntax parsing.
    ///
    /// Parser errors occur while transforming tokens
    /// into an abstract syntax tree (AST).
    #[error(transparent)]
    Parser(
        /// The underlying parser error.
        #[from]
        ParserError,
    ),
}
