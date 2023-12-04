use common::split_by_line;
use std::collections::HashSet;

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE: &str = include_str!("sample.txt");

#[derive(Debug)]
pub struct Card {
    pub id: i32,
    pub winning_numbers: Vec<i32>,
    pub numbers: Vec<i32>,
}

pub fn parse_input(input: &str) -> Vec<Card> {
    let lines = split_by_line(input);

    lines
        .into_iter()
        .map(|line| {
            let (id, raw_card) = {
                let (id_part, card_part) = line.split_once(": ").unwrap();

                let id = id_part.split_whitespace().nth(1).unwrap().parse().unwrap();
                let card_part = card_part.trim();

                (id, card_part)
            };

            let (winning_numbers, numbers) = {
                let (winning_part, numbers_part) = raw_card.split_once(" | ").unwrap();

                let winning_numbers = winning_part
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();

                let numbers = numbers_part
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();

                (winning_numbers, numbers)
            };

            Card {
                id,
                winning_numbers,
                numbers,
            }
        })
        .collect()
}

pub fn number_intersection_count(card: &Card) -> usize {
    card.winning_numbers
        .iter()
        .collect::<HashSet<&i32>>()
        .intersection(&card.numbers.iter().collect::<HashSet<&i32>>())
        .count()
}

pub mod part1;
pub mod part2;
