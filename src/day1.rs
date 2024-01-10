use core::panic;

#[derive(Debug)]
enum InputState {
    Empty,
    Single(u32),
    Double(u32, u32),
}

impl InputState {
    fn new() -> Self {
        InputState::Empty
    }

    fn update(self, value: u32) -> Self {
        match self {
            InputState::Empty => InputState::Single(value),
            InputState::Single(prev) => InputState::Double(prev, value),
            InputState::Double(prev, _) => InputState::Double(prev, value),
        }
    }

    fn sum(self) -> u32 {
        match self {
            InputState::Empty => 0,
            InputState::Single(value) => value * 10 + value,
            InputState::Double(first, second) => first * 10 + second,
        }
    }
}

pub fn trebuchet_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .fold(InputState::new(), |state, value| state.update(value))
                .sum()
        })
        .sum()
}

static NUMBERS: &'static [&str; 9] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn trebuchet_two(input: &str) -> u32 {
    input
        .lines()
        .map(|line: &str| {
            let mut results: Vec<u32> = vec![];
            let mut chars = line.chars();

            while let Some(c) = chars.next() {
                if c.is_numeric() {
                    let value = c.to_string().parse::<u32>().unwrap();
                    results.push(value);
                } else {
                    let to_find = format!("{}{}", c, chars.as_str());

                    let mut index = 0u32;
                    for number in NUMBERS {
                        index += 1;
                        if to_find.starts_with(number) {
                            for _ in 0..number.len() - 2 {
                                chars.next();
                            }

                            results.push(index);
                        }
                    }
                }
            }

            let result = match results[..] {
                [first, .., second] => {
                    format!("{}{}", first, second)
                }
                [num] => format!("{}{}", num, num),
                _ => panic!("Invalid Line: {:?}", line),
            };

            result.parse::<u32>().unwrap()
        })
        .sum()
}
