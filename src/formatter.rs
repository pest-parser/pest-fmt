use crate::{error::PestError::Unreachable, utils::GrammarRule, Formatter, PestError, PestResult};
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use std::{
    fs::{read_to_string, File},
    io::Write,
    path::Path,
};
use text_utils::indent;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct PestParser;

impl Formatter {
    pub fn format_file<P: AsRef<Path>>(&self, path_from: P, path_to: P) -> PestResult<()> {
        let input = read_to_string(path_from)?;
        let output = self.format(&input)?;

        let mut file = File::create(path_to)?;
        file.write_all(output.as_bytes())?;
        Ok(())
    }

    pub fn format(&self, input: &str) -> PestResult<String> {
        let mut pairs = match PestParser::parse(Rule::grammar_rules, input) {
            Ok(pairs) => pairs,
            Err(e) => return Err(PestError::ParseFail(e.to_string())),
        }
        .peekable();

        let mut code = String::new();
        let mut output = vec![];

        for pair in pairs.clone() {
            let next_pair = pairs.peek();

            let start = pair.as_span().start_pos().line_col().0;
            let end = pair.as_span().end_pos().line_col().0;

            match pair.as_rule() {
                Rule::EOI => continue,
                Rule::COMMENT => {
                    let code = self.format_comment(pair);
                    output.push(GrammarRule::raw(&code, (start, end)));
                }
                Rule::grammar_rule => match self.format_grammar_rule(pair) {
                    Ok(rule) => output.push(rule),
                    Err(e) => return Err(e),
                },
                Rule::grammar_doc => {
                    let code = self.format_line_doc(pair, "//!");
                    output.push(GrammarRule::raw(&code, (start, end)));
                }
                Rule::WHITESPACE => continue,
                _ => return Err(Unreachable(unreachable_rule!())),
            };
        }

        // println!("{:?}", output.iter().map(|s| s.to_string(0)).collect::<Vec<_>>());

        let mut last = 0 as usize;
        let mut group = vec![];
        let mut groups = vec![];
        for rule in output {
            let (s, e) = rule.lines;
            if last + 1 == s {
                group.push(rule)
            } else {
                if group.len() != 0 {
                    groups.push(group);
                }
                group = vec![rule]
            }
            last = e
        }
        groups.push(group);

        for group in groups {
            let mut length = vec![];
            for r in &group {
                length.push(r.identifier.chars().count())
            }
            let max = length.iter().max().unwrap();

            code.push_str(&group.iter().map(|rule| rule.to_string(*max)).collect::<Vec<_>>().join("\n"));
            code.push_str("\n");
        }

        // Remove leading and trailing whitespace
        let out = code.trim().to_string();

        return Ok(out);
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
                            code = format!("{{ {} }}", s.join("|"));
                        } else if self.choice_first {
                            code = format!("{{\n  {}}}", indent(&s.join("\n| "), self.indent - 2));
                        } else {
                            code = format!("{{\n{}}}", indent(&s.join(" |\n"), self.indent));
                        }
                    }
                    Err(e) => return Err(e),
                },
                Rule::line_doc => {
                    code.push_str(&self.format_line_doc(pair, "///"));
                    return Ok(GrammarRule::raw(&code, (start, end)));
                }
                _ => (),
            };
        }
        return Ok(GrammarRule { is_raw: false, identifier, modifier, code, lines: (start, end) });
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
        return Ok(code);
    }

    fn format_term(&self, pairs: Pair<Rule>) -> PestResult<String> {
        let mut code = String::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::COMMENT => {
                    let comment = &self.format_comment(pair);
                    code.push_str(&format!(" {}\n", comment));
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

    fn format_comment(&self, pairs: Pair<Rule>) -> String {
        let mut code = String::new();
        let raw = pairs.as_str().trim();

        if raw.starts_with("//") {
            code.push_str("// ");
            code.push_str(raw[2..raw.len()].trim());
        } else if raw.starts_with("/*") {
            let raw = raw.trim_start_matches("/*").trim_end_matches("*/").trim();

            let comment_lines = raw.split('\n');
            let comment_lines: Vec<String> = comment_lines.map(|c| c.trim().to_string()).collect();

            code = if comment_lines.len() == 1 {
                /* Foo */
                format!("/* {} */", comment_lines.join("").trim())
            } else {
                /*
                  Foo
                */
                format!("/*\n{}\n*/", indent(comment_lines.join("\n"), self.indent))
            };
        } else {
            unreachable!()
        }
        return code;
    }

    fn format_line_doc(&self, pairs: Pair<Rule>, prefix: &str) -> String {
        let raw = pairs.as_str();
        let code = format!("{} {}", prefix, raw.trim_start_matches(prefix).trim());

        code.trim().to_string()
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

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! expect_correction {
        ($source:expr, $expected:expr,) => {
            let fmt = Formatter::default();
            let source = indoc::indoc! { $source };
            let expected = indoc::indoc! { $expected };

            assert_eq!(fmt.format(source).unwrap().trim_end(), expected.trim_end())
        };
        ($source:expr, $expected:expr) => {
            expect_correction!($source, $expected,)
        };
    }

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
    fn test_comment() {
        expect_correction! {
            r#"
              //comment1
            //comment2
            a = { "a" }
            "#,
            r#"
            // comment1
            // comment2
            a = { "a" }
            "#,
        };

        expect_correction! {
            r#"
            ///comment1
                ///comment2
            a = { "a" }
            "#,
            r#"
            /// comment1
            /// comment2
            a = { "a" }
            "#,
        };

        expect_correction! {
            r#"
            //!comment1
                //!comment2
            a = { "a" }"#,
            r#"
            //! comment1
            //! comment2
            a = { "a" }"#,
        };
    }
}
