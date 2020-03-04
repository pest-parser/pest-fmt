extern crate pest_fmt;

use pest_fmt::Settings;
use std::io::Error;

#[test]
fn pest_a() -> Result<(), Error> {
    let cfg = Settings::default();
    let file = include_str!("pest.pest");
    println!("{}", cfg.format(file));
    cfg.format_file("tests/pest.pest", "tests/out/pest_a.pest")
}

#[test]
fn pest_b() -> Result<(), Error> {
    let mut cfg = Settings::default();
    cfg.indent = 2;
    cfg.choice_first = false;
    let file = include_str!("pest.pest");
    println!("{}", cfg.format(file));
    cfg.format_file("tests/pest.pest", "tests/out/pest_b.pest")
}

#[test]
fn valkyrie_a() -> Result<(), Error> {
    let cfg = Settings::default();
    let file = include_str!("valkyrie.pest");
    println!("{}", cfg.format(file));
    cfg.format_file("tests/valkyrie.pest", "tests/out/valkyrie_a.pest")
}

#[test]
fn valkyrie_b() -> Result<(), Error> {
    let mut cfg = Settings::default();
    cfg.indent = 2;
    cfg.choice_first = false;
    let file = include_str!("valkyrie.pest");
    println!("{}", cfg.format(file));
    cfg.format_file("tests/valkyrie.pest", "tests/out/valkyrie_b.pest")
}

#[test]
fn arc_a() -> Result<(), Error> {
    let cfg = Settings::default();
    let file = include_str!("arc.pest");
    println!("{}", cfg.format(file));
    cfg.format_file("tests/arc.pest", "tests/out/arc_a.pest")
}
