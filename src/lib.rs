extern crate pest;

#[cfg(test)]
extern crate proc_macro;

#[cfg(test)]
extern crate quote;

#[macro_use]
mod error;
pub mod formatter;
pub mod utils;

pub use error::{PestError, PestResult};
use utils::GrammarRule;

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

#[derive(Debug, Clone)]
enum Node {
    Rule(GrammarRule),
    Comment(String),
    LineDoc(String),
    Str(String),
}

impl Node {
    fn to_string(&self, indent: usize) -> String {
        match self {
            Node::Rule(rule) => rule.to_string(indent),
            Node::Comment(c) => c.to_owned(),
            Node::LineDoc(c) => c.to_owned(),
            Node::Str(c) => c.to_owned(),
        }
    }
}
