use crate::grammar::{PestParser, Rule};
use pest::Parser;

#[derive(Debug)]
pub struct Settings {
    pub pest_indent: u8,
    pub pest_end_line: bool,
}

impl Settings {
    pub fn format(self, text: &str) -> String {
        let pairs = PestParser::parse(Rule::grammar_rules, text).unwrap_or_else(|e| panic!("{}", e));
        let mut code = String::new();
        for pair in pairs {
            match pair.as_rule() {
                _ => {
                    println!("Rule:    {:?}", pair.as_rule());
                    println!("Span:    {:?}", pair.as_span());
                    println!("Text:    {}\n", pair.as_str());
                }
            };
        }
        unreachable!();
        return code;
    }
    pub fn format_file(self, path: &str) -> String {
        return String::new();
    }
}
