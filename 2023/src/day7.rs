use std::collections::HashMap;

#[derive(Debug)]
enum Suite {
    FiveOfAKind(Vec<u32>),
    FourOfAKind(Vec<u32>),
    ThreeOfAKind(Vec<u32>),
    FullHouse(Vec<u32>),
    TwoPair(Vec<u32>),
    OnePair(Vec<u32>),
    HighCard(Vec<u32>),
}

impl Suite {
    fn get_value(self: &Self) -> (&Vec<u32>, u8) {
        match self {
            Suite::HighCard(cards) => (cards, 0),
            Suite::OnePair(cards) => (cards, 1),
            Suite::TwoPair(cards) => (cards, 2),
            Suite::ThreeOfAKind(cards) => (cards, 3),
            Suite::FullHouse(cards) => (cards, 4),
            Suite::FourOfAKind(cards) => (cards, 5),
            Suite::FiveOfAKind(cards) => (cards, 6),
        }
    }
}

const fn parse_card(card: char) -> Option<u32> {
    match card {
        'A' => Some(14),
        'K' => Some(13),
        'Q' => Some(12),
        'J' => Some(11),
        'T' => Some(10),
        _ => card.to_digit(10),
    }
}

fn parse_cards(line: &str) -> Suite {
    let mut counts = HashMap::new();
    for c in line.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    let cards: Vec<u32> = line.chars().filter_map(parse_card).collect();

    match counts.len() {
        5 => Suite::HighCard(cards),
        4 => Suite::OnePair(cards),
        3 => match counts.values().filter(|&&count| count > 1).count() {
            2 => Suite::TwoPair(cards),
            _ => Suite::ThreeOfAKind(cards),
        },
        2 => match counts.values().filter(|&&count| count > 3).count() {
            1 => Suite::FourOfAKind(cards),
            _ => Suite::FullHouse(cards),
        },
        1 => Suite::FiveOfAKind(cards),
        _ => panic!("Unknown combination: {}", line),
    }
}

pub fn camel_cards(input: &str) -> u64 {
    let mut games: Vec<(Suite, u64)> = input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(" ").unwrap();
            let bid: u64 = bid.parse::<u64>().unwrap();
            let cards = parse_cards(cards);

            (cards, bid)
        })
        .collect();

    games.sort_by(|(a, _), (b, _)| {
        let (first_cards, first_position) = a.get_value();
        let (second_cards, second_position) = b.get_value();

        return first_position
            .cmp(&second_position)
            .then(first_cards.cmp(&second_cards));
    });

    let mut total = 0;
    for (index, (_, bid)) in games.iter().enumerate() {
        total += (index + 1) as u64 * bid;
    }

    total
}
