use criterion::{criterion_group, criterion_main, Criterion};

fn day7(c: &mut Criterion) {
    let input_first: &str = include_str!("../input/day7.txt").trim();
    let mut group = c.benchmark_group("day7");

    group.bench_function("part1", |b| {
        b.iter(|| aoc2023::day7::camel_cards(input_first))
    });
}

criterion_group!(benches, day7);
criterion_main!(benches);
