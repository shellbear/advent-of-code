fn main() {
    let input: &str = include_str!("../input/day4.txt").trim();
    let result = aoc2023::day4::scratchcards(input);

    println!("Result: {:?}", result);
}
