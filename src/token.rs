#[derive(Debug)]
pub enum Token<'a> {
    String(&'a str),
    Identifier(&'a str),
    Number(i32),
    Keyword(Keyword),
    Method(RequestMethod),
    Return(&'a str),
    Assignment,
    Colon,
    Semicolon,
    Comma,
    OpenBrace,
    CloseBrace,
    OpenParenthesis,
    CloseParenthesis,
    EOF,
}

#[derive(Debug, PartialEq)]
pub enum TokenError {
    InvalidKeyword(String),
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

#[derive(Debug)]
pub enum Keyword {
    Query,
    Body,
    Headers,
    Retry,
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

#[derive(Debug)]
pub enum RequestMethod {
    GET,
    POST,
    PUT,
    PATCH,
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
