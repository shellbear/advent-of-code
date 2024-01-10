use criterion::{criterion_group, criterion_main, Criterion};

fn day1(c: &mut Criterion) {
    let input_first: &str = include_str!("../input/day1.txt").trim();
    let input_two: &str = include_str!("../input/day1_2.txt").trim();
    let mut group = c.benchmark_group("day1");

    group.bench_function("part1", |b| {
        b.iter(|| aoc2023::day1::trebuchet_one(input_first))
    });
    group.bench_function("part2", |b| {
        b.iter(|| aoc2023::day1::trebuchet_two(input_two));
    });
}

criterion_group!(benches, day1);
criterion_main!(benches);
