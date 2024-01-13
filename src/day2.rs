static RED_CUBES: u32 = 12;
static GREEN_CUBES: u32 = 13;
static BLUE_CUBES: u32 = 14;

#[aoc(day2, part1)]
pub fn cube_conundrum(input: &str) -> u32 {
    let mut game_count = 0;

    input
        .lines()
        .filter_map(|line| {
            let mut chars = line.chars();
            game_count += 1;

            // Skip first characters
            while chars.next().is_some_and(|c| c != ':') {}

            let games = chars.as_str().trim().split(';');

            for game in games {
                let parts = game.trim().split(',');

                for part in parts {
                    let (value, color) = part.trim().split_once(' ').unwrap();
                    let value = value.parse::<u32>().unwrap();

                    match color {
                        "red" => {
                            if value > RED_CUBES {
                                return None;
                            }
                        }
                        "green" => {
                            if value > GREEN_CUBES {
                                return None;
                            }
                        }
                        "blue" => {
                            if value > BLUE_CUBES {
                                return None;
                            }
                        }
                        _ => panic!("Invalid color"),
                    }
                }
            }

            Some(game_count)
        })
        .sum()
}
