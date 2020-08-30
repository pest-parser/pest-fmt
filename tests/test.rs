extern crate pest_fmt;

use pest_fmt::{PestResult, Settings};
use std::io::Error;

#[test]
fn basic() {
    let cfg = Settings::default();
    const INPUT: &str = r#"exponent_part = _{ ^  "e" ~ ("+" | "-")? ~ ASCII_DIGIT+ }"#;
    const OUTPUT: &str = r#"exponent_part = _{^"e" ~ ("+"|"-")? ~ ASCII_DIGIT+}"#;
    assert_eq!(cfg.format(INPUT).unwrap().trim_end(), OUTPUT)
}

#[test]
fn bad_cases() -> PestResult<()> {
    let cfg = Settings::default();
    cfg.format_file("tests/bad_cases.pest", "tests/out/bad_cases.pest")
}

#[test]
fn pest_a() -> PestResult<()> {
    let cfg = Settings::default();
    cfg.format_file("tests/pest.pest", "tests/out/pest_a.pest")
}

#[test]
fn pest_b() -> PestResult<()> {
    let mut cfg = Settings::default();
    cfg.indent = 2;
    cfg.choice_first = false;
    cfg.format_file("tests/pest.pest", "tests/out/pest_b.pest")
}

#[test]
fn valkyrie_a() -> PestResult<()> {
    let cfg = Settings::default();
    cfg.format_file("tests/valkyrie.pest", "tests/out/valkyrie_a.pest")
}

#[test]
fn valkyrie_b() -> PestResult<()> {
    let mut cfg = Settings::default();
    cfg.indent = 2;
    cfg.choice_first = false;
    cfg.format_file("tests/valkyrie.pest", "tests/out/valkyrie_b.pest")
}

#[test]
fn arc_a() -> PestResult<()> {
    let cfg = Settings::default();
    cfg.format_file("tests/arc.pest", "tests/out/arc_a.pest")
}
