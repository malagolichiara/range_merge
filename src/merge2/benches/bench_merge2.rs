use criterion::{black_box, criterion_group, criterion_main, Criterion};
use merge2::merge;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("merge", |b| {
        b.iter(|| {
            let input = vec![1..2, 4..5, 1..52, 22..33, 1..3, 67..90];
            merge(black_box(input))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
