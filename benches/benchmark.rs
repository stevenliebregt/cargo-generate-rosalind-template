use criterion::{black_box, criterion_group, criterion_main, Criterion};
use app::{echo};
use std::fs;

fn criterion_benchmark_echo(criterion: &mut Criterion) {
    let mut group_samples = criterion.benchmark_group("samples");

    group_samples.bench_function("calculate_nucleotide_counts -> dataset [1]", |b| {
        b.iter(|| {
            echo(black_box(&String::from("Hello, world!")))
        })
    });

    group_samples.finish();
}

criterion_group!(
    benches,
    criterion_benchmark_echo,
);
criterion_main!(benches);
