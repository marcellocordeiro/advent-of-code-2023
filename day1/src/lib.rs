pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE_PART1: &str = include_str!("inputs/sample_part1.txt");
pub const SAMPLE_PART2: &str = include_str!("inputs/sample_part2.txt");

pub fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub mod part1;
pub mod part2;
