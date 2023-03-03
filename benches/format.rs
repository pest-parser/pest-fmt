use criterion::{criterion_group, criterion_main, Criterion};

fn bench_format(c: &mut Criterion) {
    let input = include_str!("../tests/fixtures/json.actual.pest");
    c.bench_function("format (json.pest)", |b| {
        let fmt = pest_fmt::Formatter::new(input);
        b.iter(|| {
            fmt.format().unwrap();
        })
    });

    let input = include_str!("../src/grammar.pest");
    c.bench_function("format (grammar.pest)", |b| {
        let fmt = pest_fmt::Formatter::new(input);

        b.iter(|| {
            fmt.format().unwrap();
        })
    });
}

criterion_group!(benches, bench_format);
criterion_main!(benches);
