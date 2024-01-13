use criterion::{criterion_group, criterion_main, Criterion};

fn day2(c: &mut Criterion) {
    let input_first: &str = include_str!("../input/day2.txt").trim();
    let mut group = c.benchmark_group("day2");

    group.bench_function("part2", |b| {
        b.iter(|| aoc2023::day2::cube_conundrum(input_first))
    });
}

criterion_group!(benches, day2);
criterion_main!(benches);
