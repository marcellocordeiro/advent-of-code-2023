pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

pub fn hash(value: &str) -> u8 {
    value
        .chars()
        .fold(0_u8, |acc, ch| acc.wrapping_add(ch as u8).wrapping_mul(17))
}

pub fn parse_input(input: &str) -> Vec<&str> {
    input.trim().split(',').collect()
}

pub mod part1;
pub mod part2;
