use crate::formatter::Rule;
use pest::iterators::Pair;
use text_utils::indent;

use crate::Formatter;

impl Formatter<'_> {
    pub(super) fn format_comment(&self, pairs: Pair<Rule>) -> String {
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
        code
    }

    pub(super) fn format_line_doc(&self, pairs: Pair<Rule>, prefix: &str) -> String {
        let raw = pairs.as_str();
        let code = format!("{} {}", prefix, raw.trim_start_matches(prefix).trim());

        code.trim().to_string()
    }
}

#[cfg(test)]
mod tests {

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
            ///comment3
            "#,
            r#"
            /// comment1
            /// comment2
            a = { "a" }
            
            /// comment3
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
