fn main() {
    let input: &str = include_str!("../input/day3.txt").trim();
    let result = aoc2023::day3::gear_ratios(input);

    println!("Result: {:?}", result);
}
