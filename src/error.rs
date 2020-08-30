use std::io::Error;

#[derive(Debug, Clone)]
pub enum PestError {
    IOError,
    Unreachable,
    ParseFail(String),
    FormatFail(String),
}

pub type PestResult<T> = Result<T, PestError>;

impl std::convert::From<std::io::Error> for PestError {
    fn from(_: Error) -> Self {
        PestError::IOError
    }
}

impl std::convert::From<&str> for PestError {
    fn from(s: &str) -> Self {
        PestError::ParseFail(String::from(s))
    }
}
