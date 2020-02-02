use crate::{
    grammar::{PestParser, Rule},
    utils::{get_lines, indent, is_one_line, GrammarRule},
};
use pest::{iterators::Pair, Parser};

#[derive(Debug)]
pub struct Settings {
    pub pest_indent: usize,
    pub pest_end_line: bool,
    pub pest_sequence_first: bool,
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
                    println!("Comment:    {}\n", pair.as_str());
                }
                Rule::grammar_rule => {
                    let out = self.format_grammar_rule(pair);
                    println!("Grammar:    {:?}\n", out.clone());
                    code.push_str(&format!("{:?}", out))
                }
                _ => unreachable!(),
            };
        }
        unreachable!();
        return code;
    }
    fn format_grammar_rule(&self, pairs: Pair<Rule>) -> GrammarRule {
        let mut code = String::new();
        let mut modifier = " ".to_string();
        let mut identifier = String::new();
        let one_line = is_one_line(pairs.as_span());
        let lines = get_lines(pairs.as_span());
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::assignment_operator => continue,
                Rule::opening_brace => continue,
                Rule::closing_brace => continue,
                Rule::identifier => identifier = pair.as_str().to_string(),
                Rule::silent_modifier => modifier = pair.as_str().to_string(),
                Rule::atomic_modifier => modifier = pair.as_str().to_string(),
                Rule::compound_atomic_modifier => modifier = pair.as_str().to_string(),
                Rule::expression => {
                    let s = self.format_expression(pair);
                    if one_line {
                        code = format!("{{ {} }}", s.join(" | "));
                    }
                    else if self.pest_sequence_first {
                        let space = std::iter::repeat(' ').take(self.pest_indent - 2).collect::<String>();
                        code = format!("{{\n  {}}}", indent(&s.join("\n| "), &space));
                    }
                    else {
                        let space = std::iter::repeat(' ').take(self.pest_indent).collect::<String>();
                        code = format!("{{\n{}}}", indent(&s.join(" |\n"), &space));
                    }
                }
                _ => unreachable!(),
            };
        }
        return GrammarRule { identifier, modifier, code, lines };
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
                Rule::sequence_operator => term.push_str(" ~ "),
                Rule::term => term.push_str(&self.format_term(pair)),
                _ => unreachable!(),
            };
        }
        code.push(term.clone());
        return code;
    }
    fn format_term(&self, pairs: Pair<Rule>) -> String {
        let mut code = String::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::negative_predicate_operator => code.push_str(pair.as_str()),
                Rule::repeat_once_operator => code.push_str(pair.as_str()),
                Rule::optional_operator => code.push_str(pair.as_str()),
                Rule::repeat_operator => code.push_str(pair.as_str()),
                Rule::opening_paren => code.push_str(pair.as_str()),
                Rule::closing_paren => code.push_str(pair.as_str()),
                Rule::identifier => code.push_str(pair.as_str()),
                Rule::string => code.push_str(pair.as_str()),
                Rule::range => code.push_str(pair.as_str()),
                Rule::expression => {
                    let e = self.format_expression(pair);
                    code.push_str(&e.join(" | "))
                }
                Rule::repeat_exact => code.push_str(&format_repeat_exact(pair)),
                Rule::repeat_min_max => code.push_str(&format_repeat_min_max(pair)),
                _ => unreachable!(),
            };
        }
        return code;
    }
}

fn format_repeat_exact(pairs: Pair<Rule>) -> String {
    let mut code = String::new();
    for pair in pairs.into_inner() {
        match pair.as_rule() {
            Rule::opening_brace => code.push_str(pair.as_str()),
            Rule::closing_brace => code.push_str(pair.as_str()),
            Rule::number => code.push_str(pair.as_str()),
            _ => unreachable!(),
        };
    }
    return code;
}

fn format_repeat_min_max(pairs: Pair<Rule>) -> String {
    let mut code = String::new();
    for pair in pairs.into_inner() {
        match pair.as_rule() {
            Rule::opening_brace => code.push_str(pair.as_str()),
            Rule::closing_brace => code.push_str(pair.as_str()),
            Rule::comma => code.push_str(", "),
            Rule::number => code.push_str(pair.as_str()),
            _ => unreachable!(),
        };
    }
    return code;
}
