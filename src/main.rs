use ignore::{overrides::OverrideBuilder, WalkBuilder};
use pest_fmt::Formatter;
use std::{error::Error, fs};
use toml::Value;

fn main() -> Result<(), Box<dyn Error>> {
    let walker = build_walker(".");

    let fmt = Formatter::new();

    println!("Pest Formatter");
    println!("-------------------------------------");

    let mut count = 0;

    for entry in walker.build() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            fmt.format_file(&path, &path).unwrap();
            count += 1;
        }
    }

    println!("{} files formatted.", count);
    Ok(())
}

fn build_walker(root: &str) -> WalkBuilder {
    let mut builder = ignore::WalkBuilder::new(root);
    builder.follow_links(true).git_ignore(true);

    let mut ingore_override = OverrideBuilder::new(&root);

    if let Ok(text) = fs::read_to_string("rustfmt.toml") {
        let excludes = read_rustfmt(&text);
        for exclude in excludes {
            ingore_override.add(&exclude).unwrap();
        }
    }

    let ingore_override = ingore_override.build().unwrap();

    builder.filter_entry(move |entry| {
        if entry.path().is_dir() {
            return true;
        }

        if ingore_override.matched(entry.path(), false).is_whitelist() {
            return false;
        }

        if let Some(path) = entry.path().to_str() {
            if path.ends_with(".pest") {
                return true;
            }
        }
        return false;
    });

    builder
}

fn read_rustfmt(input: &str) -> Vec<String> {
    if let Ok(rust_fmt) = input.parse::<Value>() {
        if let Some(pest) = rust_fmt.get("pest") {
            if let Some(value) = pest.get("exclude") {
                return toml_string_or_string_list(value);
            }
        }
    }

    return vec![];
}

fn toml_string_or_string_list(value: &Value) -> Vec<String> {
    let mut out = vec![];
    match value {
        Value::String(s) => out.push(s.to_string()),
        Value::Array(a) => {
            for v in a {
                if let Some(s) = v.as_str() {
                    out.push(s.to_string())
                }
            }
        }
        _ => (),
    }

    return out;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_rustfmt() {
        let text = fs::read_to_string("rustfmt.toml").unwrap();
        let excludes = read_rustfmt(&text);
        assert_eq!(vec!["tests/**/*.pest".to_string(), "tests/test.pest".to_string()], excludes);

        // test other
        let text = r#"
        [pest]
        exclude = "src/**/*"
        "#;

        let excludes = read_rustfmt(&text);
        assert_eq!(vec!["src/**/*".to_string()], excludes);
    }

    #[test]
    fn test_build_walker() {
        let walker = build_walker(".");

        let mut files: Vec<String> = vec![];
        for entry in walker.build() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                files.push(path.to_string_lossy().to_string());
            }
        }
        assert_eq!(vec!["./src/grammar.pest".to_string()], files);
    }
}
