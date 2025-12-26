use criterion::{Criterion, criterion_group, criterion_main};
use day03::{part_1, part_2};
use libaoc::load_input;

fn bench(c: &mut Criterion) {
    let day = 3;
    let year = 2025;
    let input = load_input(year, day);
    c.bench_function("part_1", |b| b.iter(|| part_1(&input)));
    c.bench_function("part_2", |b| b.iter(|| part_2(&input)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
