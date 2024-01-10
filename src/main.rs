fn main() {
    let input: &str = include_str!("../input/day1.txt").trim();
    let result = aoc2023::day1::trebuchet_one(input);

    println!("Result: {:?}", result);
}
