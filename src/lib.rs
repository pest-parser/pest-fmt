extern crate pest;
#[cfg(test)]
extern crate pest_generator;
#[cfg(test)]
extern crate proc_macro;
#[cfg(test)]
#[macro_use]
extern crate quote;

#[cfg(test)]
mod pre_build;
#[macro_use]
mod error;
pub mod formatter;
pub mod grammar;
pub mod utils;

pub use error::{PestError, PestResult};

pub struct Settings {
    pub indent: usize,
    pub choice_hanging: bool,
    pub choice_first: bool,
    pub set_alignment: bool,
    pub blank_lines: Option<usize>,
    /// spaces between `=`
    pub set_space: usize,
    /// spaces between `|`
    pub choice_space: usize,
    /// spaces between `{ }`
    pub braces_space: usize,
    /// spaces between `~`
    pub sequence_space: usize,
    /// spaces between `( )`
    pub parentheses_space: usize,
}
