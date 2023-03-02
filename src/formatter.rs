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
                Rule::grammar_doc => nodes.push(Node::LineDoc(self.format_line_doc(pair, "//!"))),
                _ => nodes.push(Node::Str(pair.as_str().to_string())),
            };

            if let Some(next) = pairs.peek() {
                self.consume_newline(&mut nodes, (span.end(), next.as_span().start()))
            }
        }

        Ok(self.group_output(nodes))
    }

    fn group_output(&self, nodes: Vec<Node>) -> String {
        // println!("------ nodes: {:?}", nodes);

        let hardbreak = Node::Str("".to_string());

        let mut groups = vec![];
        let mut nodes = nodes.iter().peekable();

        // Iterate all nodes and group consecutive rules into a group.
        let mut last = 0_usize;
        let mut group = vec![];
        while let Some(node) = nodes.next() {
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

                    if let Some(Node::LineDoc(_)) = nodes.peek() {
                        group.push(&hardbreak);
                    }
                }
                _ => {
                    group.push(node);
                }
            }
        }
        groups.push(group);

        // Iterate all groups, output as string, and add \n after each groups.
        let mut output = String::new();
        for nodes in groups {
            let mut length = vec![];
            let mut max = 0;
            let mut has_modifier = false;

            // Iterate nodes first to know the indent size
            for node in &nodes {
                if let Node::Rule(rule) = node {
                    // To get max length of the identifiers, as the indent size.
                    length.push(rule.identifier.chars().count());
                    max = *length.iter().max().unwrap();

                    // Check if there is any modifier (@, _, !, $).
                    // If exists, we need to keep a space before `{` to let rules in a group in tidy.
                    if !rule.modifier.is_empty() && rule.modifier != " " {
                        has_modifier = true;
                    }
                }
            }

            // Build final code for each group
            let mut line_codes = vec![];
            for node in &nodes {
                if let Node::Rule(rule) = node {
                    let mut rule = rule.clone();
                    // If this group not have modifier, we need to trim the modifier to avoid
                    if !has_modifier {
                        rule.modifier = rule.modifier.trim().to_owned();
                    }

                    line_codes.push(rule.to_string(max));
                } else {
                    line_codes.push(node.to_string(max));
                }
            }

            output.push_str(&line_codes.join("\n"));
            output.push('\n');
        }

        // Remove leading and trailing whitespace
        output.trim().to_string()
    }

    fn format_grammar_rule(&self, pair: Pair<Rule>) -> PestResult<Node> {
        let mut code = String::new();
        let mut modifier = " ".to_string();
        let mut identifier = String::new();

        let start_line = pair.as_span().start_pos().line_col().0;
        let end_line = pair.as_span().end_pos().line_col().0;

        for pair in pair.into_inner() {
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
                    Ok(parts) => {
                        if start_line == end_line {
                            code = format!("{{ {} }}", parts.join(" | "));
                        } else {
                            code = format!("{{\n  {}}}", indent(parts.join("\n| "), 2));
                        }
                    }
                    Err(e) => return Err(e),
                },
                Rule::line_doc => {
                    return Ok(Node::LineDoc(self.format_line_doc(pair, "///")));
                }
                _ => {}
            };
        }
        Ok(Node::Rule(GrammarRule { identifier, modifier, code, lines: (start_line, end_line) }))
    }

    fn format_expression(&self, pairs: Pair<Rule>) -> PestResult<Vec<String>> {
        let mut code = vec![];
        let mut term = String::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::COMMENT => {
                    let comment = self.format_comment(pair);
                    if !term.is_empty() {
                        term.push(' ');
                    }
                    term.push_str(&comment);
                    term.push('\n');
                }
                Rule::choice_operator => {
                    code.push(term.clone());
                    term.clear();
                }
                Rule::sequence_operator => {
                    if !term.ends_with('\n') {
                        term.push(' ')
                    }
                    term.push('~');
                    term.push(' ')
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
                    let comment = self.format_comment(pair);
                    if !code.ends_with('\n') {
                        code.push(' ')
                    }
                    code.push_str(&comment);
                    code.push('\n');
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
        multi = {
        "a"
        | "b"
        }

        b1 = {"b"}
        b1_b1 = ${"b1"}
        // comment
        c1 = { "c" }

        d0 = {"d"}
        d01 = {"d1"}

        e0 = {"e"}
        e01 = !{"e1"}
        "#,
        r#"
        a1          =  { "A" }
        foo_bar_dar = @{ "A" }
        a2          = _{ "A" }
        multi       =  {
            "a"
          | "b"
        }

        b1    =  { "b" }
        b1_b1 = ${ "b1" }
        // comment
        c1 = { "c" }

        d0  = { "d" }
        d01 = { "d1" }

        e0  =  { "e" }
        e01 = !{ "e1" }
        "#
        }
    }
}
