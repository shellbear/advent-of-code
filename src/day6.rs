fn get_values(line: &str) -> Vec<u64> {
    let (_, parts) = line.split_once(':').unwrap();
    parts
        .trim()
        .split_ascii_whitespace()
        .filter_map(|value| value.parse().ok())
        .collect()
}

fn process_distance(distance: u64, time: u64) -> u64 {
    let mut total = 0u64;

    for i in 2..time - 1 {
        let result = i * (time - i);

        if result > 0 && result > distance {
            total += 1;
        }
    }

    total
}

#[aoc(day6, part1)]
pub fn wait_for_it(input: &str) -> u64 {
    let (time, distance) = input.split_once("\n").unwrap();
    let times = get_values(time);
    let distances = get_values(distance);
    let mut total = 0u64;

    for (time, distance) in times.iter().zip(distances.iter()) {
        let time = *time;
        let distance = *distance;

        let result = process_distance(distance, time);

        if total == 0 {
            total = result;
        } else if result > 0 {
            total *= result;
        }
    }

    total
}
