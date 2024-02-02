fn main() {
    let input: &str = include_str!("../input/day6.txt").trim();
    let result = aoc2023::day6::wait_for_it(input);

    println!("Result: {:?}", result);
}
