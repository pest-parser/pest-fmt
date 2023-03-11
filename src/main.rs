use ignore::{overrides::OverrideBuilder, WalkBuilder};
use pest_fmt::{Formatter, PestResult};
use std::{error::Error, fs, path::Path};
use toml::Value;

fn main() -> Result<(), Box<dyn Error>> {
    let argv: Vec<String> = std::env::args().collect();
    // Check if we have a path to format
    if argv.len() > 1 {
        let paths = &argv[1..];
        for path in paths {
            if Path::new(path).exists() {
                if Path::new(path).is_file() {
                    if let Ok(changed) = format_file(path, path) {
                        if changed {
                            println!("Formatted {}", path);
                        }
                    }
                } else {
                    let walker = build_walker(path);
                    let files = format_directory(walker)?;
                    if files == 0 {
                        println!("No file has been formatted");
                    } else {
                        println!("Formatted {} files", files);
                    }
                }
            } else {
                println!("No such file or directory: {}", path);
            }
        }
    } else {
        // Format all files in the current directory
        let walker = build_walker(".");
        let files = format_directory(walker)?;
        if files == 0 {
            println!("No file has been formatted");
        } else {
            println!("Formatted {} files", files);
        }
    }

    Ok(())
}

pub fn format_file<P: AsRef<Path>>(path_from: P, path_to: P) -> PestResult<bool> {
    let input = std::fs::read_to_string(path_from)?;
    let fmt = Formatter::new(&input);
    let output = fmt.format()?;

    let mut file = std::fs::File::create(path_to)?;
    std::io::Write::write_all(&mut file, output.as_bytes())?;
    Ok(input != output)
}

/// Format all files in the given directory.
/// Returns the number of files that were formatted.
pub fn format_directory(walker: WalkBuilder) -> Result<usize, Box<dyn Error>> {
    let mut files = 0;
    for entry in walker.build() {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(path) = path.to_str() {
                if path.ends_with(".pest") {
                    if let Ok(changed) = format_file(path, path) {
                        if changed {
                            println!("Formatted {}", path);
                            files += 1;
                        }
                    }
                }
            }
        }
    }

    Ok(files)
}

fn build_walker(root: &str) -> WalkBuilder {
    let mut builder = ignore::WalkBuilder::new(root);
    builder.follow_links(true).git_ignore(true);

    let mut ingore_override = OverrideBuilder::new(root);

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

        false
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

    vec![]
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

    out
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

        let excludes = read_rustfmt(text);
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

        #[cfg(not(target_os = "windows"))]
        assert_eq!(vec!["./src/grammar.pest".to_string()], files);
    }
}
