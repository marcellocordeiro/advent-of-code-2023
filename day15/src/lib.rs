pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE: &str = include_str!("sample.txt");

pub fn hash(value: &str) -> usize {
    value
        .chars()
        .fold(0_u8, |acc, ch| acc.wrapping_add(ch as u8).wrapping_mul(17)) as usize
}

pub fn parse_input(input: &str) -> Vec<&str> {
    input.trim().split(',').collect()
}

pub mod part1;
pub mod part2;
