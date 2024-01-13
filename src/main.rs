fn main() {
    let input: &str = include_str!("../input/day2.txt").trim();
    let result = aoc2023::day2::cube_conundrum(input);

    println!("Result: {:?}", result);
}
