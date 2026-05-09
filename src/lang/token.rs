/// Represents a lexical token produced by the lexer.
///
/// A token consists of:
/// - [`TokenKind`] describing the token category
/// - [`Span`] describing its location in the source code
#[derive(Debug, PartialEq)]
pub struct Token {
    /// The category/type of the token.
    pub kind: TokenKind,

    /// The source location information for this token.
    pub span: Span,
}

/// Represents a region in the source code.
///
/// A span tracks:
/// - byte/index positions
/// - line number
/// - column number
#[derive(Debug, PartialEq)]
pub struct Span {
    /// The starting byte/index position in the source.
    pub start: usize,

    /// The ending byte/index position in the source.
    pub end: usize,

    /// The line number where the token appears.
    pub line: usize,

    /// The column number where the token starts.
    pub column: usize,
}

/// Represents all possible token categories recognized by the lexer.
///
/// Each variant describes a specific lexical construct that can appear
/// in the source code.
#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    /// An identifier token.
    ///
    /// Identifiers are user-defined names such as variables,
    /// function names, or object properties.
    ///
    /// # Examples
    ///
    /// ```text
    /// user
    /// response_data
    /// _temporary
    /// ```
    Identifier(String),

    /// Marks the beginning of a string literal.
    ///
    /// # Example
    ///
    /// ```text
    /// "hello"
    /// ^
    /// ```
    StringStart,

    /// Plain text content inside a string literal.
    ///
    /// This token excludes interpolation syntax.
    ///
    /// # Example
    ///
    /// ```text
    /// "hello world"
    ///  ^^^^^^^^^^^
    /// ```
    StringText(String),

    /// Marks the end of a string literal.
    ///
    /// # Example
    ///
    /// ```text
    /// "hello"
    ///       ^
    /// ```
    StringEnd,

    /// Marks the start of string interpolation.
    ///
    /// # Example
    ///
    /// ```text
    /// ${name}
    /// ^^
    /// ```
    InterpolationStart,

    /// Marks the end of string interpolation.
    ///
    /// # Example
    ///
    /// ```text
    /// ${name}
    ///       ^
    /// ```
    InterpolationEnd,

    /// An integer numeric literal.
    ///
    /// # Examples
    ///
    /// ```text
    /// 42
    /// 1000
    /// ```
    Number(i64),

    /// A floating-point numeric literal.
    ///
    /// # Examples
    ///
    /// ```text
    /// 3.14
    /// 0.5
    /// ```
    Float(f64),

    /// A boolean value representing either true or false.
    ///
    /// # Examples
    ///
    /// ```text
    /// false
    /// true
    /// ```
    Boolean(bool),

    /// A reserved language keyword.
    ///
    /// Keywords have special meaning in the language grammar.
    ///
    /// # Examples
    ///
    /// ```text
    /// for
    /// group
    /// timeout
    /// ```
    Keyword(Keyword),

    /// An HTTP request method token.
    ///
    /// Methods are prefixed with `@` in the source code.
    ///
    /// # Examples
    ///
    /// ```text
    /// @GET
    /// @POST
    /// ```
    Method(RequestMethod),

    /// Left parenthesis: `(`
    LParen,

    /// Right parenthesis: `)`
    RParen,

    /// Left brace: `{`
    LBrace,

    /// Right brace: `}`
    RBrace,

    /// Left bracket: `[`
    LBracket,

    /// Right bracket: `]`
    RBracket,

    /// Colon token: `:`
    Colon,

    /// Semicolon token: `;`
    Semicolon,

    /// Comma token: `,`
    Comma,

    /// Dot token: `.`
    Dot,

    /// Arrow token: `->`
    ///
    /// Commonly used for mappings, return types,
    /// or expression transformations.
    Arrow,

    /// Assignment operator: `=`
    Assign,

    /// An operator token.
    ///
    /// See [`Operator`] for the full list of supported operators.
    Operator(Operator),

    /// Range operator: `..`
    ///
    /// # Example
    ///
    /// ```text
    /// 1..10
    /// ```
    Range,

    /// A comment token.
    ///
    /// This variant may be used when comments are preserved
    /// instead of ignored by the lexer.
    Comment,

    /// End-of-file marker.
    ///
    /// This token is always emitted at the end of tokenization
    /// to indicate that no more input remains.
    EOF,
}

/// Represents supported operators recognized by the lexer.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    /// Addition operator: `+`
    Plus,

    /// Subtraction operator: `-`
    Minus,

    /// Multiplication operator: `*`
    Star,

    /// Division operator: `/`
    Slash,

    /// Equality comparison operator: `==`
    Eq,

    /// Inequality comparison operator: `!=`
    Ne,

    /// Less-than comparison operator: `<`
    Lt,

    /// Greater-than comparison operator: `>`
    Gt,

    /// Less-than-or-equal comparison operator: `<=`
    Le,

    /// Greater-than-or-equal comparison operator: `>=`
    Ge,

    /// Logical negation operator: `!`
    ///
    /// This operator is commonly used for boolean negation.
    Not,
}

/// Represents reserved keywords in the language.
///
/// Keywords are case-insensitive and cannot be used
/// as regular identifiers.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Keyword {
    /// `for`
    ///
    /// Used for loop iteration constructs.
    For,

    /// `in`
    ///
    /// Used together with `for` to iterate over collections or ranges.
    In,

    /// `group`
    ///
    /// Declares or groups related request definitions.
    Group,

    /// `timeout`
    ///
    /// Defines a request timeout configuration.
    Timeout,

    /// `retry`
    ///
    /// Defines retry behavior for failed requests.
    Retry,

    /// `sleep`
    ///
    /// Pauses execution for a specified duration.
    Sleep,

    /// `body`
    ///
    /// Declares or accesses an HTTP request body.
    Body,

    /// `headers`
    ///
    /// Declares or accesses HTTP headers.
    Headers,

    /// `query`
    ///
    /// Declares or accesses query parameters.
    Query,

    /// `assert`
    ///
    /// Performs assertion checks against responses or expressions.
    Assert,
}

/// Parses a string into a [`Keyword`].
///
/// Parsing is case-insensitive.
///
/// # Examples
///
/// ```rust
/// use std::str::FromStr;
///
/// # use nero::token::Keyword;
/// assert_eq!(Keyword::from_str("for").unwrap(), Keyword::For);
/// assert_eq!(Keyword::from_str("GROUP").unwrap(), Keyword::Group);
/// assert_eq!(Keyword::from_str("Timeout").unwrap(), Keyword::Timeout);
/// ```
impl std::str::FromStr for Keyword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            method if method.eq_ignore_ascii_case("for") => Ok(Self::For),
            method if method.eq_ignore_ascii_case("in") => Ok(Self::In),

            method if method.eq_ignore_ascii_case("group") => Ok(Self::Group),

            method if method.eq_ignore_ascii_case("retry") => Ok(Self::Retry),
            method if method.eq_ignore_ascii_case("sleep") => Ok(Self::Sleep),
            method if method.eq_ignore_ascii_case("timeout") => Ok(Self::Timeout),

            method if method.eq_ignore_ascii_case("body") => Ok(Self::Body),
            method if method.eq_ignore_ascii_case("headers") => Ok(Self::Headers),
            method if method.eq_ignore_ascii_case("query") => Ok(Self::Query),
            method if method.eq_ignore_ascii_case("assert") => Ok(Self::Assert),

            _ => Err(()),
        }
    }
}

/// Represents supported HTTP request methods.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RequestMethod {
    /// HTTP GET request.
    ///
    /// Used to retrieve resources from a server.
    GET,

    /// HTTP POST request.
    ///
    /// Used to create new resources or submit data.
    POST,

    /// HTTP PUT request.
    ///
    /// Used to replace an existing resource.
    PUT,

    /// HTTP PATCH request.
    ///
    /// Used to partially update a resource.
    PATCH,

    /// HTTP DELETE request.
    ///
    /// Used to remove a resource.
    DELETE,
}

/// Parses a string into a [`RequestMethod`].
///
/// This implementation is case-insensitive.
///
/// # Examples
///
/// ```rust
/// use std::str::FromStr;
///
/// # use nero::token::RequestMethod;
/// assert_eq!(RequestMethod::from_str("GET").unwrap(), RequestMethod::GET);
/// assert_eq!(RequestMethod::from_str("post").unwrap(), RequestMethod::POST);
/// assert_eq!(RequestMethod::from_str("PuT").unwrap(), RequestMethod::PUT);
/// ```
///
/// # Errors
///
/// Returns [`TokenError::InvalidMethod`] if the provided string
/// does not match any supported HTTP request method.
impl std::str::FromStr for RequestMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            method if method.eq_ignore_ascii_case("GET") => Ok(Self::GET),
            method if method.eq_ignore_ascii_case("POST") => Ok(Self::POST),
            method if method.eq_ignore_ascii_case("PUT") => Ok(Self::PUT),
            method if method.eq_ignore_ascii_case("PATCH") => Ok(Self::PATCH),
            method if method.eq_ignore_ascii_case("DELETE") => Ok(Self::DELETE),
            _ => Err(()),
        }
    }
}
