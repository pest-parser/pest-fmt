use crate::grammar::{PestParser, Rule};
use pest::{iterators::Pair, Parser, Span};

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
                    if self.pest_end_line {
                        code.push_str("\n")
                    }
                }
                Rule::COMMENT => {
                    println!("Text:    {}\n", pair.as_str());
                }
                Rule::grammar_rule => {
                    let out = self.format_grammar_rule(pair);
                    code.push_str(&out)
                }
                _ => unreachable!(),
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
                Rule::identifier => id = pair.as_str(),
                Rule::silent_modifier => modifier = pair.as_str(),
                Rule::atomic_modifier => modifier = pair.as_str(),
                Rule::compound_atomic_modifier => modifier = pair.as_str(),
                Rule::expression => {
                    let s = self.format_expression(pair);
                    ()
                }
                _ => unreachable!()
            };
        }
        return code;
    }
    fn format_expression(&self, pairs: Pair<Rule>) -> Vec<String> {
        let mut code = vec![];
        let mut term = String::new();
        let mut id = "";
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::choice_operator => {
                    code.push(term.clone());
                    term = String::new()
                }
                Rule::sequence_operator => {
                    term.push_str(" ~ ")
                }
                Rule::term => {
                    term.push_str(&self.format_term(pair))
                }
                _ => {
                    println!("Rule:    {:?}", pair.as_rule());
                    println!("Span:    {:?}", pair.as_span());
                    println!("Text:    {}\n", pair.as_str());
                }
            };
        }
        code.push(term.clone());
        return code;
    }
    fn format_term(&self, pairs: Pair<Rule>) -> String {
        let mut code = String::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::repeat_once_operator => {
                    code.push_str(pair.as_str())
                }
                Rule::optional_operator => {
                    code.push_str(pair.as_str())
                }
                Rule::repeat_operator => {
                    code.push_str(pair.as_str())
                }
                Rule::opening_paren => {
                    code.push_str(pair.as_str())
                }
                Rule::closing_paren => {
                    code.push_str(pair.as_str())
                }
                Rule::identifier => {
                    code.push_str(pair.as_str())
                }
                Rule::string => {
                    code.push_str(pair.as_str())
                }
                Rule::range => {
                    code.push_str(pair.as_str())
                }
                Rule::negative_predicate_operator => {
                    code.push_str(pair.as_str())
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
}

fn is_one_line(span: Span) -> bool {
    let s = span.start_pos().line_col().0;
    let e = span.end_pos().line_col().0;
    return s == e;
}
