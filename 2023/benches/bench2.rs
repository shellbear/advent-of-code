use criterion::{criterion_group, criterion_main, Criterion};

fn day2(c: &mut Criterion) {
    let input_first: &str = include_str!("../input/day2.txt").trim();
    let input_second: &str = include_str!("../input/day2_2.txt").trim();
    let mut group = c.benchmark_group("day2");

    group.bench_function("part1", |b| {
        b.iter(|| aoc2023::day2::cube_conundrum(input_first))
    });
    group.bench_function("part2", |b| {
        b.iter(|| aoc2023::day2::cube_conundrum_two(input_second))
    });
}

criterion_group!(benches, day2);
criterion_main!(benches);
