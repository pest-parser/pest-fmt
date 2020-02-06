use crate::Settings;
use pest::Span;
use std::fmt::{Debug, Error, Formatter};
pub use textwrap::indent;

impl Default for Settings {
    fn default() -> Self {
        Settings { pest_indent: 4, pest_sequence_first: true }
    }
}

pub fn is_one_line(span: Span) -> bool {
    let s = span.start_pos().line_col().0;
    let e = span.end_pos().line_col().0;
    return s == e;
}

pub fn get_lines(span: Span) -> (usize, usize) {
    let s = span.start_pos().line_col().0;
    let e = span.end_pos().line_col().0;
    return (s, e);
}

#[derive(Clone)]
pub struct GrammarRule {
    pub is_comment: bool,
    pub identifier: String,
    pub modifier: String,
    pub code: String,
    pub lines: (usize, usize),
}

impl GrammarRule {
    pub fn comment(c: &str) -> Self {
        GrammarRule { is_comment: true, identifier: "".to_string(), modifier: "".to_string(), code: c.to_string(), lines: (0, 0) }
    }
    pub fn to_string(&self, indent: usize) -> String {
        if self.is_comment {
            return self.code.clone();
        }
        let mut code = self.identifier.clone();
        while code.chars().count() < indent {
            code.push_str(" ")
        }
        code.push_str(" = ");
        code.push_str(&self.modifier);
        code.push_str(&self.code);
        code.push_str("\n");
        return code;
    }
}

impl Debug for GrammarRule {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}: {:?}", self.identifier, self.lines)
    }
}
