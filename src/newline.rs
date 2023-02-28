use crate::{Formatter, Node};

impl Formatter<'_> {
    /// If match the text containes "\n" between current pair and next pair, then push a new line
    /// For example:
    ///
    /// 1. `a = { "a" }\nb = { "b" }` => `a = { "a" }\nb = { "b" }`
    /// 2. `a = { "a" }\n\nb = { "b" }` => `a = { "a" }\n\nb = { "b" }`
    /// 3. `a = { "a" }\n\n\nb = { "b" }` => `a = { "a" }\n\nb = { "b" }`
    pub(super) fn consume_newline(&self, nodes: &mut Vec<Node>, span: (usize, usize)) {
        if let Some(last) = nodes.last() {
            if last.to_string(self.indent).ends_with('\n') {
                return;
            }
        }

        let part = self.get_str((span.0, span.1));
        // If there have at least 2 "\n" then push a new line
        if part.matches('\n').count() >= 2 {
            nodes.push(Node::Str("".to_string()));
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_keep_exist_newline() {
        expect_correction! {
            r#"
            a = { "a" }

            b = { "b" }
            
            
            c = { "c" }
            d = { "d" }
            // This is comment
            e = { "e" }
            "#,
            r#"
            a = { "a" }

            b = { "b" }
            
            c = { "c" }
            d = { "d" }
            // This is comment
            e = { "e" }
            "#,
        }
    }
}
