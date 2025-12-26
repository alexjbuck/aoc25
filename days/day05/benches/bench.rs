use criterion::{Criterion, criterion_group, criterion_main};
use day05::{part_1, part_2};

fn bench(c: &mut Criterion) {
    let input = include_str!("../.input/2025/5.txt");
    c.bench_function("part1", |b| b.iter(|| part_1(input)));
    c.bench_function("part2", |b| b.iter(|| part_2(input)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
