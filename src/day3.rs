fn check_around(x: usize, y: usize, lines: &Vec<&str>) -> bool {
    let previous_x = x.checked_sub(1).unwrap_or_default();
    let previous_y = y.checked_sub(1).unwrap_or_default();

    let mut around: Vec<Option<char>> = vec![
        lines[x].chars().nth(previous_y),
        lines[x].chars().nth(y + 1),
    ];

    if let Some(previous_line) = lines.iter().nth(previous_x) {
        around.extend([
            previous_line.chars().nth(previous_y),
            previous_line.chars().nth(y),
            previous_line.chars().nth(y + 1),
        ]);
    }

    if let Some(next_line) = lines.iter().nth(x + 1) {
        around.extend([
            next_line.chars().nth(previous_y),
            next_line.chars().nth(y),
            next_line.chars().nth(y + 1),
        ]);
    }

    around
        .iter()
        .filter_map(|c| *c)
        .any(|c| !c.is_digit(10) && c != '.')
}

#[aoc(day3, part1)]
pub fn gear_ratios(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut total = 0u32;

    for (x, line) in lines.iter().enumerate() {
        let mut current_number = 0u32;
        let mut is_valid = false;

        for (y, char) in line.chars().enumerate() {
            if let Some(digit) = char.to_digit(10) {
                current_number = current_number * 10 + digit;

                if current_number > 0 && !is_valid {
                    is_valid = check_around(x, y, &lines)
                }
            } else {
                if is_valid {
                    total += current_number;
                }

                current_number = 0;
                is_valid = false;
            }
        }

        if is_valid && current_number > 0 {
            total += current_number;
        }
    }

    total
}
