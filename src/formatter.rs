use crate::{error::PestError::Unreachable, Formatter, GrammarRule, Node, PestError, PestResult};
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use text_utils::indent;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct PestParser;

impl Formatter<'_> {
    pub fn format(&self) -> PestResult<String> {
        let input = self.input;

        let mut pairs = match PestParser::parse(Rule::grammar_rules, input) {
            Ok(pairs) => pairs,
            Err(e) => return Err(PestError::ParseFail(e.to_string())),
        }
        .peekable();

        let mut code = String::new();
        let mut nodes = vec![];

        while let Some(pair) = pairs.next() {
            let span = pair.as_span();

            match pair.as_rule() {
                Rule::COMMENT => {
                    let code = self.format_comment(pair);
                    nodes.push(Node::Comment(code));
                }
                Rule::grammar_rule => match self.format_grammar_rule(pair) {
                    Ok(node) => nodes.push(node),
                    Err(e) => return Err(e),
                },
                Rule::grammar_doc => {
                    let code = self.format_line_doc(pair, "//!");
                    nodes.push(Node::LineDoc(code));
                }
                _ => nodes.push(Node::Str(pair.as_str().to_string())),
            };

            if let Some(next) = pairs.peek() {
                self.consume_newline(&mut nodes, (span.end(), next.as_span().start()))
            }
        }

        // println!("------ nodes: {:?}", nodes);

        let mut last = 0_usize;
        let mut group = vec![];
        let mut groups = vec![];
        let mut nodes = nodes.iter().peekable();

        let hardbreak = Node::Str("".to_string());

        while let Some(node) = nodes.next() {
            let next_node = nodes.peek();

            match &node {
                Node::Rule(rule) => {
                    let (s, e) = rule.lines;
                    if last + 1 == s {
                        group.push(node);
                    } else {
                        if !group.is_empty() {
                            groups.push(group);
                        }
                        group = vec![node];
                    }
                    last = e;

                    if let Some(Node::LineDoc(_)) = next_node {
                        group.push(&hardbreak);
                    }
                }
                _ => {
                    group.push(node);
                }
            }
        }
        groups.push(group);

        for group in groups {
            let mut length = vec![];
            let mut max = 0;
            for r in &group {
                if let Node::Rule(rule) = r {
                    length.push(rule.identifier.chars().count());
                    max = *length.iter().max().unwrap();
                }
            }

            code.push_str(&group.iter().map(|rule| rule.to_string(max)).collect::<Vec<_>>().join("\n"));
            code.push('\n');
        }

        // Remove leading and trailing whitespace
        let out = code.trim().to_string();

        Ok(out)
    }

    fn format_grammar_rule(&self, pairs: Pair<Rule>) -> PestResult<Node> {
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
                            code = format!("{{ {} }}", s.join(" | "));
                        } else if self.choice_first {
                            code = format!("{{\n  {}}}", indent(&s.join("\n| "), self.indent - 2));
                        } else {
                            code = format!("{{\n{}}}", indent(&s.join(" |\n"), self.indent));
                        }
                    }
                    Err(e) => return Err(e),
                },
                Rule::line_doc => {
                    return Ok(Node::LineDoc(self.format_line_doc(pair, "///")));
                }
                _ => (),
            };
        }
        Ok(Node::Rule(GrammarRule { is_raw: false, identifier, modifier, code, lines: (start, end) }))
    }

    fn format_expression(&self, pairs: Pair<Rule>) -> PestResult<Vec<String>> {
        let mut code = vec![];
        let mut term = String::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::COMMENT => {
                    let comment = self.format_comment(pair);
                    term.push_str(&format!(" {}\n", &comment));
                }
                Rule::choice_operator => {
                    code.push(term.clone());
                    term.clear();
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
        Ok(code)
    }

    fn format_term(&self, pairs: Pair<Rule>) -> PestResult<String> {
        let mut code = String::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::COMMENT => {
                    let comment = &self.format_comment(pair);
                    code.push_str(&format!(" {comment}\n"));
                }
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
                        Ok(expression) => code.push_str(&expression.join(" | ")),
                        Err(e) => return Err(e),
                    }
                }
                Rule::_push => match self.format_term(pair) {
                    Ok(string) => {
                        code.push_str("PUSH");
                        code.push_str(&string)
                    }
                    Err(e) => return Err(e),
                },
                Rule::peek_slice => match self.format_term(pair) {
                    Ok(string) => {
                        code.push_str("PEEK");
                        code.push_str(&string)
                    }
                    Err(e) => return Err(e),
                },
                Rule::repeat_min => code.push_str(&format_repeat_min_max(pair)?),
                Rule::repeat_exact => code.push_str(&format_repeat_min_max(pair)?),
                Rule::repeat_min_max => code.push_str(&format_repeat_min_max(pair)?),
                _ => code.push_str(pair.as_str()),
            };
        }
        Ok(code)
    }
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
    code
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
    Ok(code)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_basic() {
        expect_correction! {
            r#"a = @ { "a"}"#,
            r#"a = @{ "a" }"#,
        };

        expect_correction! {
            r#"a = _{^  "e"~("+"|"-")  ? ~ ASCII_DIGIT+ }"#,
            r#"a = _{ ^"e" ~ ("+" | "-")? ~ ASCII_DIGIT+ }"#,
        };

        expect_correction! {
            r#"
            a ={ "a"}
                b = {a ~ "b"}
            "#,
            r#"
            a = { "a" }
            b = { a ~ "b" }
            "#,
        };
    }

    #[test]
    fn test_stack() {
        expect_correction! {
            r#"
            a = ${PUSH(^"a"  )  ~ (!(NEWLINE|PEEK)~ ANY)+ ~ POP }
            "#,
            r#"
            a = ${ PUSH(^"a") ~ (!(NEWLINE | PEEK) ~ ANY)+ ~ POP }
            "#,
        }
    }

    #[test]
    fn test_group_assigns() {
        expect_correction! {
            r#"
            a1 = {"A"}
            foo_bar_dar = @{"A"}
            a2 = _{"A"}

            b1 = {"b"}
            b1_b1 = ${"b1"}
            // comment
            c1 = { "c" }
            "#,
            r#"
            a1          = { "A" }
            foo_bar_dar = @{ "A" }
            a2          = _{ "A" }

            b1    = { "b" }
            b1_b1 = ${ "b1" }
            // comment
            c1 = { "c" }
            "#
        }
    }
}
