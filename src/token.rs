/// Represents all possible tokens produced by the lexer
///
/// A `Token` is the smallest unit of meaning in the Nero Script
/// These tokens are later consumed by the parser to build the AST
///
/// Each variant represent a spesific syntactic element such as:
/// - literals (string, number)
/// - identifiers
/// - keywords
/// - symbols (e.g. `{`, `}`, `:`)
/// - HTTP methods
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum Token<'a> {
    /// A string literal
    ///
    /// The value does not include the surrounding qoutes
    String(&'a str),

    /// An identifier, e.g. variable names or labels
    Identifier(&'a str),

    /// A number literal
    ///
    /// Currently support integer values only
    Number(i32),

    /// A reserved keyword in the Nero Script.
    ///
    /// See [`Keyword`] for the list of supported keywords
    Keyword(Keyword),

    /// An HTTP request method (e.g. GET, POST)
    ///
    /// See [`RequestMethod`] for the list of supported methods
    Method(RequestMethod),

    /// A return expression
    Return(&'a str),

    /// Assignment operator (`=`)
    Assignment,

    /// Colon symbol (`:`)
    ///
    /// Used in key-value pairs
    Colon,

    /// Semicolon (`;`)
    ///
    /// Used to determine statements (optional)
    Semicolon,

    /// Comma (`,`)
    ///
    /// Used to separate value
    Comma,

    /// Opening brace (`{`).
    OpenBrace,

    /// Closing brace (`}`).
    CloseBrace,

    /// Opening parenthesis (`(`).
    OpenParenthesis,

    /// Closing parenthesis (`)`).
    CloseParenthesis,

    /// End of file
    ///
    /// Always appended as the last token
    EOF,
}

/// Represents errors related to token parsing or conversion
#[derive(Debug, PartialEq)]
pub enum TokenError {
    /// An invalid keyword was encountered
    ///
    /// Contains the original string value
    InvalidKeyword(String),

    /// An invalid HTTP method was encountered
    ///
    /// Contains the original string value
    InvalidMethod(String),
}

impl std::fmt::Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidKeyword(s) => write!(f, "invalid keyword: {}", s),
            Self::InvalidMethod(s) => write!(f, "invalid method: {}", s),
        }
    }
}

impl std::error::Error for TokenError {}

/// Represents all reserved keywords in the Nero Script
///
/// Keywords are used to define request configuration blocks
#[derive(Debug, PartialEq)]
pub enum Keyword {
    /// Query parameter block
    Query,

    /// Request body block
    Body,

    /// HTTP headers block
    Headers,

    /// Retry configuration
    Retry,

    /// Timeout configuration (in miliseconds)
    Timeout,
}

impl std::str::FromStr for Keyword {
    type Err = TokenError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "query" => Ok(Keyword::Query),
            "body" => Ok(Keyword::Body),
            "headers" => Ok(Keyword::Headers),
            "retry" => Ok(Keyword::Retry),
            "timeout" => Ok(Keyword::Timeout),
            _ => Err(TokenError::InvalidKeyword(s.to_string())),
        }
    }
}

/// Represents supported HTTP request methods
#[derive(Debug, PartialEq)]
pub enum RequestMethod {
    /// HTTP GET request
    GET,

    /// HTTP POST request
    POST,

    /// HTTP PUT request
    PUT,

    /// HTTP PATCH request
    PATCH,

    /// HTTP DELETE request
    DELETE,
}

impl std::str::FromStr for RequestMethod {
    type Err = TokenError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "get" => Ok(RequestMethod::GET),
            "post" => Ok(RequestMethod::POST),
            "put" => Ok(RequestMethod::PUT),
            "patch" => Ok(RequestMethod::PATCH),
            "delete" => Ok(RequestMethod::DELETE),
            _ => Err(TokenError::InvalidMethod(s.to_string())),
        }
    }
}
