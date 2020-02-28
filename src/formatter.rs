use crate::{
    grammar::{PestParser, Rule},
    utils::{indent, GrammarRule},
};
use pest::{iterators::Pair, Parser};
use std::{
    fs::{read_to_string, File},
    io::Write,
};

pub struct Settings {
    pub pest_indent: usize,
    pub pest_choice_hanging: bool,
    pub pest_choice_first: bool,
    pub pest_set_alignment: bool,
    pub pest_blank_lines: Option<usize>,
    /// spaces between `=`
    pub pest_set_space: usize,
    /// spaces between `|`
    pub pest_choice_space: usize,
    /// spaces between `{ }`
    pub pest_braces_space: usize,
    /// spaces between `~`
    pub pest_sequence_space: usize,
    /// spaces between `( )`
    pub pest_parentheses_space: usize,
}

impl Settings {
    pub fn format_file(&self, path_from: &str, path_to: &str) -> Result<(), std::io::Error> {
        let r = read_to_string(path_from)?;
        let s = self.format(&r);
        let mut file = File::create(path_to)?;
        file.write_all(s.as_bytes())?;
        return Ok(());
    }
    pub fn format(&self, text: &str) -> String {
        let pairs = PestParser::parse(Rule::grammar_rules, text).unwrap_or_else(|e| panic!("{}", e));
        let mut code = String::new();
        let mut codes = vec![];
        for pair in pairs {
            match pair.as_rule() {
                Rule::EOI => continue,
                Rule::COMMENT => {
                    let start = pair.as_span().start_pos().line_col().0;
                    let end = pair.as_span().end_pos().line_col().0;
                    codes.push(GrammarRule { is_comment: true, identifier: String::new(), modifier: String::new(), code: pair.as_str().to_string(), lines: (start, end) })
                }
                Rule::grammar_rule => codes.push(self.format_grammar_rule(pair)),
                Rule::WHITESPACE => continue,
                _ => unreachable!(),
            };
        }
        let mut last = 0 as usize;
        let mut group = vec![];
        let mut groups = vec![];
        for i in codes {
            let (s, e) = i.lines;
            if last + 1 == s {
                group.push(i)
            }
            else {
                if group.len() != 0 {
                    groups.push(group);
                }
                group = vec![i]
            }
            last = e
        }
        groups.push(group);
        for g in groups {
            let mut length = vec![];
            for r in &g {
                length.push(r.identifier.chars().count())
            }
            let max = length.iter().max().unwrap();

            for r in &g {
                code.push_str(&r.to_string(*max));
                code.push_str("\n");
            }
            code.push_str("\n");
        }
        return code;
    }
    fn format_grammar_rule(&self, pairs: Pair<Rule>) -> GrammarRule {
        let mut code = String::new();
        let mut modifier = " ".to_string();
        let mut identifier = String::new();
        let start = pairs.as_span().start_pos().line_col().0;
        let end = pairs.as_span().end_pos().line_col().0;
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::assignment_operator => continue,
                Rule::opening_brace => continue,
                Rule::closing_brace => continue,
                Rule::identifier => identifier = pair.as_str().to_string(),
                Rule::silent_modifier => modifier = pair.as_str().to_string(),
                Rule::atomic_modifier => modifier = pair.as_str().to_string(),
                Rule::non_atomic_modifier => modifier = pair.as_str().to_string(),
                Rule::compound_atomic_modifier => modifier = pair.as_str().to_string(),
                Rule::expression => {
                    let s = self.format_expression(pair);
                    if start == end {
                        code = format!("{{{}}}", s.join("|"));
                    }
                    else if self.pest_choice_first {
                        let space = std::iter::repeat(' ').take(self.pest_indent - 2).collect::<String>();
                        code = format!("{{\n  {}}}", indent(&s.join("\n| "), &space));
                    }
                    else {
                        let space = std::iter::repeat(' ').take(self.pest_indent).collect::<String>();
                        code = format!("{{\n{}}}", indent(&s.join(" |\n"), &space));
                    }
                }
                _ => {
                    println!("Rule:    {:?}", pair.as_rule());
                    println!("Span:    {:?}", pair.as_span());
                    println!("Text:    {}\n", pair.as_str());
                    unreachable!();
                }
            };
        }
        return GrammarRule { is_comment: false, identifier, modifier, code, lines: (start, end) };
    }
    fn format_expression(&self, pairs: Pair<Rule>) -> Vec<String> {
        let mut code = vec![];
        let mut term = String::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::choice_operator => {
                    code.push(term.clone());
                    term = String::new()
                }
                Rule::sequence_operator => {
                    let joiner = format!("{0}~{0}", " ".repeat(self.pest_sequence_space));
                    term.push_str(&joiner)
                }
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
                Rule::WHITESPACE => continue,
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
                    let joiner = format!("{0}|{0}", " ".repeat(self.pest_choice_space));
                    code.push_str(&e.join(&joiner))
                }
                Rule::_push => code.push_str(&self.format_term(pair)),
                Rule::repeat_min => code.push_str(&format_repeat_min_max(pair)),
                Rule::repeat_exact => code.push_str(&format_repeat_min_max(pair)),
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
            Rule::WHITESPACE => continue,
            Rule::opening_brace => code.push_str(pair.as_str()),
            Rule::closing_brace => code.push_str(pair.as_str()),
            Rule::comma => code.push_str(", "),
            Rule::number => code.push_str(pair.as_str()),
            _ => unreachable!(),
        };
    }
    return code;
}
