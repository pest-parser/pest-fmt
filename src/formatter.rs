use crate::grammar::{PestParser, Rule};
use pest::{Parser, Span};
use pest::iterators::Pair;

#[derive(Debug)]
pub struct Settings {
    pub pest_indent: u8,
    pub pest_end_line: bool,
}

impl Settings {
    pub fn format_file(&self, path: &str) -> String {
        return String::new();
    }
    pub fn format(&self, text: &str) -> String {
        let pairs = PestParser::parse(Rule::grammar_rules, text).unwrap_or_else(|e| panic!("{}", e));
        let mut code = String::new();
        for pair in pairs {
            match pair.as_rule() {
                Rule::EOI => {
                    if self.pest_end_line { code.push_str("\n") }
                }
                Rule::grammar_rule => {
                    let out = self.format_grammar_rule(pair);
                    code.push_str(&out)
                }
                _ => unreachable!()
            };
        }
        unreachable!();
        return code;
    }
    fn format_grammar_rule(&self, pairs: Pair<Rule>) -> String {
        let mut code = String::new();
        let mut modifier = " ";
        let mut id = "";
        let one_line = is_one_line(pairs.as_span());
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::assignment_operator => continue,
                Rule::opening_brace => continue,
                Rule::closing_brace => continue,
                Rule::identifier => {
                    id = pair.as_str()
                }
                Rule::silent_modifier => {
                    modifier = pair.as_str()
                }
                Rule::atomic_modifier => {
                    modifier = pair.as_str()
                }
                Rule::compound_atomic_modifier => {
                    modifier = pair.as_str()
                }
                _ => {
                    println!("Rule:    {:?}", pair.as_rule());
                    println!("Span:    {:?}", pair.as_span());
                    println!("Text:    {}\n", pair.as_str());
                }
            };
        }
        return code;
    }
    fn format_expression(&self, pairs: Pair<Rule>) -> String {
        let mut code = String::new();
        let mut modifier = " ";
        let mut id = "";
        let one_line = is_one_line(pairs.as_span());
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                _ => {
                    println!("Rule:    {:?}", pair.as_rule());
                    println!("Span:    {:?}", pair.as_span());
                    println!("Text:    {}\n", pair.as_str());
                }
            };
        }
        return code;
    }
}

fn is_one_line(span: Span) -> bool {
    let s = span.start_pos().line_col().0;
    let e = span.end_pos().line_col().0;
    return s == e;
}