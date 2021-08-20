use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use merge2::merge;
use std::ops::Range;

fn criterion_benchmark(c: &mut Criterion) {
    let input: Vec<Range<usize>> = vec![1..2, 4..5, 1..52, 22..33, 1..3, 67..90];
    c.bench_with_input(BenchmarkId::new("merge2", ""), &input, |b, i| {
        b.iter(|| merge(black_box(i.to_vec())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
