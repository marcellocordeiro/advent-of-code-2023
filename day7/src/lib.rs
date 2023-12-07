use common::split_by_line;

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE: &str = include_str!("sample.txt");

pub struct Hand {
    pub cards: String,
    pub bid: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

pub fn parse_input(input: &str) -> Vec<Hand> {
    let lines = split_by_line(input);

    lines
        .iter()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();

            let cards = cards.to_owned();
            let bid = bid.parse().unwrap();

            Hand { cards, bid }
        })
        .collect()
}

pub mod part1;
pub mod part2;
