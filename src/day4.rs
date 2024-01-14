#[aoc(day4, part1)]
pub fn scratchcards(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let (_, game) = line.split_once(':').unwrap();
            let (numbers, results) = game.split_once('|').unwrap();

            let numbers: Vec<u64> = numbers
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            let results: Vec<u64> = results
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            let mut total = 0;

            for number in numbers {
                if results.contains(&number) {
                    if total == 0 {
                        total = 1;
                    } else {
                        total *= 2;
                    }
                }
            }

            total
        })
        .sum()
}
