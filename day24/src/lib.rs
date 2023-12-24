use itertools::Itertools;

pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

struct Hail {
    position: (isize, isize, isize),
    velocity: (isize, isize, isize),
}

fn parse_input(input: &str) -> Vec<Hail> {
    input
        .lines()
        .map(|line| {
            let (position, velocity) = line
                .split_once(" @ ")
                .map(|(a, b)| (a.trim(), b.trim()))
                .unwrap();

            let position = position
                .split(", ")
                .map(|p| p.parse().unwrap())
                .collect_tuple()
                .unwrap();
            
            let velocity = velocity
                .split(", ")
                .map(|v| v.parse().unwrap())
                .collect_tuple()
                .unwrap();

            Hail { position, velocity }
        })
        .collect_vec()
}

pub mod part1;
pub mod part2;
