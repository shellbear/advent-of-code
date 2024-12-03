fn main() {
    let input: &str = include_str!("../inputs/day-01.txt").trim();

    let (mut first, mut second): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            line.split_once(" ")
                .map(|(a, b)| {
                    (
                        a.trim().parse::<u32>().unwrap(),
                        b.trim().parse::<u32>().unwrap(),
                    )
                })
                .unwrap()
        })
        .unzip();

    first.sort();
    second.sort();

    let mut total: u32 = 0;
    for (a, b) in first.iter().zip(second.iter()) {
        total += {
            if a > b {
                a - b
            } else {
                b - a
            }
        }
    }

    println!("Total: {}", total);
}
