# Pest Formatter

[![Test](https://github.com/pest-parser/pest-fmt/actions/workflows/test.yml/badge.svg)](https://github.com/pest-parser/pest-fmt/actions/workflows/test.yml)

Pest Formatter is a tool to format [Pest](https://pest.rs) grammar files.

## Installation

```bash
cargo install pest_fmt
```

## Usage

Then use the `pestfmt` command to format your `.pest` files.

```shell
pestfmt .
```

It will find all `.pest` files in the current directory and format them.

Output:

```
Pest Formatter
-------------------------------------
2 files formatted.
```

## Usage as a library

Add `pest_fmt` into your `Cargo.toml`:

```
cargo add pest_fmt
```

Then use the `Formatter` struct to format pest grammar.

```rust
use pest_fmt::Formatter;

let mut fmt = Formatter::new("a={ASCII_DIGIT}");
let out = fmt.format().unwrap();
println!("{out}");
// a = { ASCII_DIGIT }
```

## Development Tool Integration

### VS Code

https://github.com/pest-parser/pest-ide-tools

## License

Mozilla Public License 2.0
