extern crate pest;

#[cfg(test)]
extern crate proc_macro;

#[cfg(test)]
extern crate quote;

#[cfg(test)]
macro_rules! expect_correction {
    ($source:expr, $expected:expr,) => {
        let source = indoc::indoc! { $source };
        let expected = indoc::indoc! { $expected };

        let fmt = crate::Formatter::new(source);

        pretty_assertions::assert_eq!(fmt.format().unwrap().trim_end(), expected.trim_end())
    };
    ($source:expr, $expected:expr) => {
        expect_correction!($source, $expected,)
    };
}

#[macro_use]

mod error;
mod comment;
pub mod formatter;
mod newline;
mod node;

pub use error::{PestError, PestResult};
pub(crate) use node::*;

pub struct Formatter<'a> {
    input: &'a str,

    /// Indent space size
    indent: usize,
    choice_first: bool,
    sequence_space: usize,
}

impl<'a> Formatter<'a> {
    /// Create new formatter
    pub fn new(input: &'a str) -> Formatter<'a> {
        Self { input, indent: 4, choice_first: true, sequence_space: 1 }
    }

    /// Returns the str of the range in self.input, return empty str if the
    /// range is valid (out of bounds).
    #[inline]
    #[allow(unused)]
    pub(crate) fn get_str(&self, span: (usize, usize)) -> &str {
        let (start, end) = span;
        // Avoid out of bounds
        if start > end || end > self.input.len() {
            return "";
        }

        &self.input[start..end]
    }
}
