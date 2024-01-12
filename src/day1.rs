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
            let mut state = InputState::new();
            let mut chars = line.chars();
            let mut c: Option<char>;

            while {
                let mut index = 0u32;
                for number in NUMBERS {
                    index += 1;
                    if chars.as_str().starts_with(number) {
                        for _ in 0..number.len() - 2 {
                            chars.next();
                        }

                        state = state.update(index);
                    }
                }

                c = chars.next();
                c.is_some()
            } {
                if let Some(value) = c.unwrap().to_digit(10) {
                    state = state.update(value);
                }
            }

            state.sum()
        })
        .sum()
}
