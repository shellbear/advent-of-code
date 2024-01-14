use criterion::{criterion_group, criterion_main, Criterion};

fn day4(c: &mut Criterion) {
    let input_first: &str = include_str!("../input/day4.txt").trim();
    let mut group = c.benchmark_group("day4");

    group.bench_function("part1", |b| {
        b.iter(|| aoc2023::day4::scratchcards(input_first))
    });
}

criterion_group!(benches, day4);
criterion_main!(benches);
