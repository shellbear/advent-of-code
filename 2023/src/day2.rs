use std::cmp::max;

#[derive(Default, Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl Round {
    fn multiply(self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug)]

struct Game {
    rounds: Vec<Round>,
}

fn compute_game(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let rounds = line
                .split_once(':')
                .unwrap()
                .1
                .split(';')
                .map(|round| {
                    round.split(',').fold(Round::default(), |round, values| {
                        let (value, color) = values.trim().split_once(' ').unwrap();
                        let value = value.parse::<u32>().unwrap();

                        match color {
                            "red" => Round {
                                red: value,
                                ..round
                            },
                            "green" => Round {
                                green: value,
                                ..round
                            },
                            "blue" => Round {
                                blue: value,
                                ..round
                            },
                            _ => panic!("Invalid color"),
                        }
                    })
                })
                .collect();

            Game { rounds }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn cube_conundrum(input: &str) -> u32 {
    let mut game_count = 0;

    compute_game(input)
        .iter()
        .filter_map(|game| {
            game_count += 1;

            for round in game.rounds.iter() {
                if round.red > 12 || round.green > 13 || round.blue > 14 {
                    return None;
                }
            }

            Some(game_count)
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn cube_conundrum_two(input: &str) -> u32 {
    compute_game(input)
        .iter()
        .map(|game| {
            game.rounds
                .iter()
                .fold(Round::default(), |round, line| Round {
                    red: max(line.red, round.red),
                    green: max(line.green, round.green),
                    blue: max(line.blue, round.blue),
                })
                .multiply()
        })
        .sum()
}
