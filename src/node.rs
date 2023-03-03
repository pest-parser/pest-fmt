use std::fmt::{Debug, Error};

#[derive(Debug, Clone)]
pub(crate) enum Node {
    Rule(GrammarRule),
    Comment(String),
    LineDoc(String),
    Str(String),
}

impl Node {
    pub(crate) fn to_string(&self, indent: usize) -> String {
        match self {
            Node::Rule(rule) => rule.to_string(indent),
            Node::Comment(c) => c.to_owned(),
            Node::LineDoc(c) => c.to_owned(),
            Node::Str(c) => c.to_owned(),
        }
    }
}

#[derive(Clone)]
pub(crate) struct GrammarRule {
    /// Rule identifier
    pub identifier: String,
    /// Modifier `!`, `@`, `_` and ` `
    pub modifier: String,
    /// Expression code
    pub code: String,
    /// (start_line, end_line)
    pub lines: (usize, usize),
}

impl GrammarRule {
    pub(crate) fn to_string(&self, indent: usize) -> String {
        let mut code = self.identifier.clone();

        while code.chars().count() < indent {
            code.push(' ')
        }
        code.push_str(" = ");

        code.push_str(&self.modifier);
        code.push_str(&self.code);

        code
    }
}

impl Debug for GrammarRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}: {:?}", self.identifier, self.lines)
    }
}
