use crate::{
    error::PestError::Unreachable,
    grammar::{PestParser, Rule},
    utils::GrammarRule,
    PestError, PestResult, Settings,
};
use pest::{iterators::Pair, Parser};
use std::{
    fs::{read_to_string, File},
    io::Write,
};
use text_utils::indent;

impl Settings {
    pub fn format_file(&self, path_from: &str, path_to: &str) -> PestResult<()> {
        let r = read_to_string(path_from)?;
        let s = self.format(&r)?;
        let mut file = File::create(path_to)?;
        file.write_all(s.as_bytes())?;
        Ok(())
    }
    pub fn format(&self, text: &str) -> PestResult<String> {
        let pairs = match PestParser::parse(Rule::grammar_rules, text) {
            Ok(pairs) => pairs,
            Err(e) => return Err(PestError::ParseFail(e.to_string())),
        };
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
                Rule::grammar_rule => match self.format_grammar_rule(pair) {
                    Ok(rule) => codes.push(rule),
                    Err(e) => return Err(e),
                },
                Rule::WHITESPACE => continue,
                _ => return Err(Unreachable(unreachable_rule!())),
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
        return Ok(code);
    }
    fn format_grammar_rule(&self, pairs: Pair<Rule>) -> PestResult<GrammarRule> {
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
                Rule::expression => match self.format_expression(pair) {
                    Ok(s) => {
                        if start == end {
                            code = format!("{{{}}}", s.join("|"));
                        }
                        else if self.choice_first {
                            code = format!("{{\n  {}}}", indent(&s.join("\n| "), self.indent - 2));
                        }
                        else {
                            code = format!("{{\n{}}}", indent(&s.join(" |\n"), self.indent));
                        }
                    }
                    Err(e) => return Err(e),
                },
                _ => (),
            };
        }
        return Ok(GrammarRule { is_comment: false, identifier, modifier, code, lines: (start, end) });
    }
    fn format_expression(&self, pairs: Pair<Rule>) -> PestResult<Vec<String>> {
        let mut code = vec![];
        let mut term = String::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::COMMENT => code.push(format_comment(pair)),
                Rule::choice_operator => {
                    code.push(term.clone());
                    term = String::new()
                }
                Rule::sequence_operator => {
                    let joiner = format!("{0}~{0}", " ".repeat(self.sequence_space));
                    term.push_str(&joiner)
                }
                Rule::term => match self.format_term(pair) {
                    Ok(string) => term.push_str(&string),
                    Err(e) => return Err(e),
                },
                _ => return Err(Unreachable(unreachable_rule!())),
            };
        }
        code.push(term.clone());
        return Ok(code);
    }

    fn format_term(&self, pairs: Pair<Rule>) -> PestResult<String> {
        let mut code = String::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::COMMENT => code.push_str(&format_comment(pair)),
                Rule::negative_predicate_operator => code.push_str(pair.as_str()),
                Rule::positive_predicate_operator => code.push_str(pair.as_str()),
                Rule::repeat_once_operator => code.push_str(pair.as_str()),
                Rule::optional_operator => code.push_str(pair.as_str()),
                Rule::repeat_operator => code.push_str(pair.as_str()),
                Rule::opening_paren => code.push_str(pair.as_str()),
                Rule::closing_paren => code.push_str(pair.as_str()),
                Rule::identifier => code.push_str(pair.as_str()),
                Rule::string => code.push_str(pair.as_str()),
                Rule::insensitive_string => {
                    code.push('^');
                    for inner in pair.into_inner() {
                        match inner.as_rule() {
                            Rule::WHITESPACE => continue,
                            Rule::string => code.push_str(inner.as_str()),
                            _ => return Err(Unreachable(unreachable_rule!())),
                        }
                    }
                }
                Rule::range => code.push_str(pair.as_str()),
                Rule::expression => {
                    let e = self.format_expression(pair);
                    match e {
                        Ok(expression) => {
                            let joiner = format!("{0}|{0}", " ".repeat(self.choice_space));
                            code.push_str(&expression.join(&joiner))
                        }
                        Err(e) => return Err(e),
                    }
                }
                Rule::_push => match self.format_term(pair) {
                    Ok(string) => code.push_str(&string),
                    Err(e) => return Err(e),
                },
                Rule::repeat_min => code.push_str(&format_repeat_min_max(pair)?),
                Rule::repeat_exact => code.push_str(&format_repeat_min_max(pair)?),
                Rule::repeat_min_max => code.push_str(&format_repeat_min_max(pair)?),
                _ => return Err(Unreachable(unreachable_rule!())),
            };
        }
        return Ok(code);
    }
}

fn format_comment(pairs: Pair<Rule>) -> String {
    let mut code = String::new();
    let raw = pairs.as_str();
    if raw.starts_with("//") {
        code.push_str("//");
        code.push_str(raw[2..raw.len()].trim());
        code.push('\n')
    }
    else {
        // block comment
        unimplemented!()
    }
    return code;
}

#[allow(dead_code)]
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

fn format_repeat_min_max(pairs: Pair<Rule>) -> PestResult<String> {
    let mut code = String::new();
    for pair in pairs.into_inner() {
        match pair.as_rule() {
            Rule::WHITESPACE => continue,
            Rule::opening_brace => code.push_str(pair.as_str()),
            Rule::closing_brace => code.push_str(pair.as_str()),
            Rule::comma => code.push_str(", "),
            Rule::number => code.push_str(pair.as_str()),
            _ => return Err(Unreachable(unreachable_rule!())),
        };
    }
    return Ok(code);
}
