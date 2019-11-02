extern crate pest_fmt;

use pest_fmt::Settings;

#[test]
fn it_works() {
    let set = Settings { pest_indent: 4, pest_end_line: true };
    let file = include_str!("pest.pest");
    println!("{}", set.format(file))
}
