use itertools::Itertools;

pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

pub fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect()
}

pub fn extrapolate(sequence: &[i64]) -> i64 {
    let differences = sequence
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect::<Vec<_>>();

    if differences.iter().all(|n| *n == 0) {
        0
    } else {
        differences.last().unwrap() + extrapolate(&differences)
    }
}

pub mod part1;
pub mod part2;
