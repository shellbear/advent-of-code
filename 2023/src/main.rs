fn main() {
    let input: &str = include_str!("../input/day7.txt").trim();
    let result = aoc2023::day7::camel_cards(input);

    println!("Result: {:?}", result);
}
