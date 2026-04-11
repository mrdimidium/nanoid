use criterion::{criterion_group, criterion_main, Criterion};

fn default(c: &mut Criterion) {
    c.bench_function("rngs::default", |b| b.iter(|| nanoid::rngs::default(21)));
}

fn non_secure(c: &mut Criterion) {
    c.bench_function("rngs::non_secure", |b| {
        b.iter(|| nanoid::rngs::non_secure(21))
    });
}

criterion_group!(benches, default, non_secure);
criterion_main!(benches);
