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
}

#[derive(Debug, Clone)]
enum Node {
    Rule(GrammarRule),
    Comment(String),
    LineDoc(String),
    Other(String),
}

impl Node {
    fn to_string(&self, indent: usize) -> String {
        match self {
            Node::Rule(rule) => rule.to_string(indent),
            Node::Comment(c) => c.to_owned(),
            Node::LineDoc(c) => c.to_owned(),
            Node::Other(c) => c.to_owned(),
        }
    }
}
