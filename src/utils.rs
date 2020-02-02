use pest::Span;
pub use textwrap::indent;
use std::fmt::{Display, Formatter, Error, Debug};

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
    pub identifier: String,
    pub modifier: String,
    pub code: String,
    pub lines: (usize, usize),
}

impl GrammarRule {
    pub fn to_string(&self, indent: usize) -> String {
        let mut code = self.identifier.clone();
        while code.chars().count() < indent {
            code.push_str(" ")
        }
        code.push_str(" = ");
        code.push_str(&self.modifier);
        code.push_str(&self.code);
        return code;
    }
}


impl Debug for GrammarRule {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}: {:?}", self.identifier, self.lines)
    }
}