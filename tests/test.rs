extern crate pest_fmt;

use pest_fmt::Formatter;

macro_rules! assert_format {
    ($source:expr, $expected:expr) => {
        let source = include_str!($source);
        let expected = include_str!($expected);

        let fmt = Formatter::new(source);

        let out = fmt.format().unwrap();
        pretty_assertions::assert_eq!(out.trim(), expected.trim());
    };
}

#[test]
#[cfg(not(target_os = "windows"))]
fn test_files() {
    assert_format!("fixtures/json.actual.pest", "fixtures/json.expected.pest");
    assert_format!("fixtures/arc.actual.pest", "fixtures/arc.expected.pest");
    assert_format!("fixtures/bad_cases.actual.pest", "fixtures/bad_cases.expected.pest");
    assert_format!("fixtures/pest.actual.pest", "fixtures/pest.expected.pest");
    assert_format!("fixtures/valkyrie.actual.pest", "fixtures/valkyrie.expected.pest");
}
