test:
	cargo test
	cargo run . tests/fixtures/pest.expected.pest
	cat tests/fixtures/json.actual.pest | cargo run . --stdin
update_grammar:
	curl -sSL https://github.com/pest-parser/pest/raw/master/meta/src/grammar.pest > src/grammar.pest
	patch src/grammar.pest src/grammar.patch
