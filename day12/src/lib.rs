pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE: &str = include_str!("sample.txt");

pub struct Springs {
    conditions: String,
    ranges: Vec<usize>,
}

pub fn parse_input(input: &str) -> Vec<Springs> {
    input
        .lines()
        .map(|line| {
            let (conditions, ranges) = line.split_once(' ').unwrap();

            let conditions = conditions.to_owned();
            let ranges = ranges.split(',').map(|num| num.parse().unwrap()).collect();

            Springs { conditions, ranges }
        })
        .collect()
}

pub mod part1;
pub mod part2;
