use criterion::{criterion_group, criterion_main, Criterion};

fn day3(c: &mut Criterion) {
    let input_first: &str = include_str!("../input/day3.txt").trim();
    let mut group = c.benchmark_group("day3");

    group.bench_function("part1", |b| {
        b.iter(|| aoc2023::day3::gear_ratios(input_first))
    });
}

criterion_group!(benches, day3);
criterion_main!(benches);
