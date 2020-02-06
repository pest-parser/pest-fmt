extern crate pest_fmt;

use pest_fmt::Settings;
use std::io::Error;

#[test]
fn pest_a() -> Result<(), Error> {
    let set = Settings::default();
    let file = include_str!("pest.pest");
    println!("{}", set.format(file));
    set.format_file("tests/pest.pest", "tests/out/pest_a.pest")
}

#[test]
fn pest_b() -> Result<(), Error> {
    let set = Settings { pest_indent: 2, pest_sequence_first: false };
    let file = include_str!("pest.pest");
    println!("{}", set.format(file));
    set.format_file("tests/pest.pest", "tests/out/pest_b.pest")
}

#[test]
fn valkyrie_a() -> Result<(), Error> {
    let set = Settings::default();
    let file = include_str!("valkyrie.pest");
    println!("{}", set.format(file));
    set.format_file("tests/valkyrie.pest", "tests/out/valkyrie_a.pest")
}

#[test]
fn valkyrie_b() -> Result<(), Error> {
    let set = Settings { pest_indent: 2, pest_sequence_first: false };
    let file = include_str!("valkyrie.pest");
    println!("{}", set.format(file));
    set.format_file("tests/valkyrie.pest", "tests/out/valkyrie_b.pest")
}


#[test]
fn arc_a() -> Result<(), Error> {
    let set = Settings::default();
    let file = include_str!("arc.pest");
    println!("{}", set.format(file));
    set.format_file("tests/arc.pest", "tests/out/arc_a.pest")
}
