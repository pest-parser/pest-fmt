#[derive(Debug, Clone)]
pub enum PestError {
    IOError,
    Unreachable,
    ParseFail(String),
    FormatFail(String),
}

pub type PestResult<T> = Result<T, PestError>;