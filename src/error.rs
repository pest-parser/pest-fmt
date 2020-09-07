use std::io;

#[derive(Debug, Clone)]
pub enum PestError {
    IOError(String),
    Unreachable(String),
    ParseFail(String),
    FormatFail(String),
}

pub type PestResult<T> = Result<T, PestError>;

impl From<io::Error> for PestError {
    fn from(e: io::Error) -> Self {
        PestError::IOError(e.to_string())
    }
}

impl From<&str> for PestError {
    fn from(s: &str) -> Self {
        PestError::ParseFail(String::from(s))
    }
}

#[macro_export]
macro_rules! unreachable_rule {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        format!("Unreachable Rule: {} at line {}", &name[..name.len() - 3], line!())
    }};
}

#[macro_export]
macro_rules! debug_cases {
    ($i:ident) => {{
        println!("Rule::{:?}=>continue,", $i.as_rule());
        println!("Span: {:?}", $i.as_span());
        println!("Text: {}", $i.as_str());
        unreachable!();
    }};
}
