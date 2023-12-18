pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

pub struct Card {
    pub id: i32,
    pub winning_nums: Vec<i32>,
    pub revealed_nums: Vec<i32>,
}

pub fn parse_input(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            let (id, card) = {
                let (id, card) = line.split_once(": ").unwrap();

                let id = id.split_whitespace().nth(1).unwrap().parse().unwrap();

                (id, card)
            };

            let (winning_nums, revealed_nums) = {
                let (winning_nums, revealed_nums) = card.split_once(" | ").unwrap();

                let winning_nums = winning_nums
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();

                let revealed_nums = revealed_nums
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();

                (winning_nums, revealed_nums)
            };

            Card {
                id,
                winning_nums,
                revealed_nums,
            }
        })
        .collect()
}

pub fn number_matches_count(card: &Card) -> usize {
    card.revealed_nums
        .iter()
        .filter(|c| card.winning_nums.contains(c))
        .count()
}

pub mod part1;
pub mod part2;
