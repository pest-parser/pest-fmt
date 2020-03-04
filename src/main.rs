use pest_fmt::Settings;
use std::{error::Error, fs};
use toml::Value;
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn Error>> {
    let mut cfg = Settings::default();
    let mut exclude = vec![];
    if let Ok(string) = fs::read_to_string("rustfmt.toml") {
        if let Ok(rust_fmt) = string.parse::<Value>() {
            if let Some(pest) = rust_fmt.get("pest") {
                if let Some(value) = pest.get("exclude") {
                    exclude = toml_string_or_string_list(value)
                }
                // TODO: use macros
                if let Some(value) = pest.get("choice_first") {
                    if let Some(b) = value.as_bool() {
                        cfg.choice_first = b
                    }
                }
                if let Some(value) = pest.get("choice_hanging") {
                    if let Some(b) = value.as_bool() {
                        cfg.choice_hanging = b
                    }
                }
                if let Some(value) = pest.get("indent") {
                    if let Some(i) = value.as_integer() {
                        cfg.indent = i as usize
                    }
                }
            }
        }
    }
    // TODO: remove exclude
    println!("Excluded: {:?}", exclude);
    for entry in WalkDir::new(".").follow_links(true).into_iter().filter_map(|e| e.ok()) {
        let f_name = entry.file_name().to_string_lossy();
        if f_name.ends_with(".pest") {
            println!("{}", f_name);
        }
    }
    Ok(())
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
    // TODO: delete dup
    return out;
}
