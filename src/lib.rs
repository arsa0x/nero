mod errors;
mod lang;
pub use errors::error;
pub use lang::*;

pub type Result<T> = std::result::Result<T, error::Error>;
