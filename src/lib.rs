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

pub struct Formatter {
    /// Indent space size
    indent: usize,
    choice_first: bool,
    choice_space: usize,
    sequence_space: usize,
}

impl Default for Formatter {
    fn default() -> Self {
        Formatter { indent: 4, choice_first: true, choice_space: 1, sequence_space: 1 }
    }
}

impl Formatter {
    /// Create new formatter
    pub fn new() -> Formatter {
        Formatter::default()
    }
}
