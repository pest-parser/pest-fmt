use ignore::{overrides::OverrideBuilder, WalkBuilder};
use pest_fmt::{Formatter, PestResult};
use std::{error::Error, fs, io::Read, path::Path};
use toml::Value;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The file or path to format
    #[arg(default_value = ".")]
    file: Vec<String>,
    #[clap(long, short, default_value = "false")]
    stdin: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    if cli.stdin {
        let mut source = String::new();
        std::io::stdin().read_to_string(&mut source).expect("failed read source from stdin");
        println!("{}", source);
    } else {
        process_files(cli.file)?;
    }

    Ok(())
}

fn process_files(paths: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut paths = paths;
    let mut updated = 0;

    // If there not argument, format the current directory
    if paths.is_empty() {
        paths.push(".".to_string());
    }

    for path in paths {
        let path = Path::new(&path);
        if path.exists() {
            if path.is_file() {
                if let Ok(changed) = format_file(&path, &path) {
                    if changed {
                        updated += 1
                    }
                }
            } else {
                let walker = build_walker(path);
                updated += format_directory(walker)?;
            }
        } else {
            eprintln!("no such file or directory: {}", path.display());
        }
    }

    println!("Formatted {} files", updated);
    Ok(())
}

fn format(source: &str) -> PestResult<String> {
    let fmt = Formatter::new(source);
    fmt.format()
}

fn format_file<P: AsRef<Path>>(path_from: P, path_to: P) -> PestResult<bool> {
    let input = std::fs::read_to_string(path_from)?;
    let output = format(&input)?;

    let mut file = std::fs::File::create(path_to)?;
    std::io::Write::write_all(&mut file, output.as_bytes())?;
    Ok(input != output)
}

/// Format all files in the given directory.
/// Returns the number of files that were formatted.
fn format_directory(walker: WalkBuilder) -> Result<usize, Box<dyn Error>> {
    let mut updated = 0;
    for entry in walker.build() {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(path) = path.to_str() {
                if path.ends_with(".pest") {
                    if let Ok(changed) = format_file(path, path) {
                        if changed {
                            updated += 1;
                        }
                    }
                }
            }
        }
    }

    Ok(updated)
}

fn build_walker<P: AsRef<Path> + Copy>(root: P) -> WalkBuilder {
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
