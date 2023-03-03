# Pest Formatter

[![Test](https://github.com/pest-parser/pest-fmt/actions/workflows/test.yml/badge.svg)](https://github.com/pest-parser/pest-fmt/actions/workflows/test.yml) [![Crate](https://img.shields.io/crates/v/pest_fmt?color=1t&label=Crate)](https://crates.io/crates/pest_fmt)

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

## Benchmark

Based on MacBook Pro (Apple M1 2020)

```
format (json.pest)                time:   [89.403 µs 89.632 µs 89.878 µs]
format (grammar.pest)             time:   [1.6018 ms 1.6054 ms 1.6105 ms]
```

## License

Mozilla Public License 2.0
