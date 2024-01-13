fn main() {
    let input: &str = include_str!("../input/day2_2.txt").trim();
    let result = aoc2023::day2::cube_conundrum_two(input);

    println!("Result: {:?}", result);
}
