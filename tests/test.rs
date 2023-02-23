extern crate pest_fmt;

use pest_fmt::Formatter;

macro_rules! assert_format {
    ($source:expr, $expected:expr) => {
        let fmt = Formatter::default();
        let source = include_str!($source);
        let expected = include_str!($expected);

        let out = fmt.format(source).unwrap();
        pretty_assertions::assert_eq!(out, expected);
    };
}

#[test]
fn test_files() {
    assert_format!("fixtures/arc.actual.pest", "fixtures/arc.expected.pest");
    // assert_format!("fixtures/bad_cases.actual.pest", "fixtures/bad_cases.expected.pest");
    // assert_format!("fixtures/pest.actual.pest", "fixtures/pest.expected.pest");
    // assert_format!("fixtures/valkyrie.actual.pest", "fixtures/valkyrie.expected.pest");
}
