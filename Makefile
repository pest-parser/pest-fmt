test:
	cargo run . tests/fixtures/pest.expected.pest
	cat tests/fixtures/json.actual.pest | cargo run . --stdin