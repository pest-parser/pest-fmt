use pest::Span;
use std::fmt::{Debug, Error};

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
    pub is_raw: bool,
    pub identifier: String,
    pub modifier: String,
    pub code: String,
    pub lines: (usize, usize),
}

impl GrammarRule {
    pub fn raw(c: &str, lines: (usize, usize)) -> Self {
        GrammarRule { is_raw: true, identifier: "".to_string(), modifier: "".to_string(), code: c.to_string(), lines }
    }

    pub fn to_string(&self, indent: usize) -> String {
        if self.is_raw {
            return self.code.clone();
        }
        let mut code = self.identifier.clone();
        while code.chars().count() < indent {
            code.push_str(" ")
        }
        code.push_str(" = ");
        code.push_str(&self.modifier.trim());
        code.push_str(&self.code);
        return code;
    }
}

impl Debug for GrammarRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}: {:?}", self.identifier, self.lines)
    }
}
