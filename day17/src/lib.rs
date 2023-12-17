pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE1: &str = include_str!("sample1.txt");
pub const SAMPLE2: &str = include_str!("sample2.txt");

pub type Grid = Vec<Vec<usize>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position(usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    #[must_use]
    pub fn offset(self) -> (isize, isize) {
        use Direction::{East, North, South, West};
        match self {
            North => (-1, 0),
            South => (1, 0),
            East => (0, 1),
            West => (0, -1),
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        use Direction::{East, North, South, West};
        match self {
            North => West,
            South => East,
            East => North,
            West => South,
        }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        use Direction::{East, North, South, West};
        match self {
            North => East,
            South => West,
            East => South,
            West => North,
        }
    }
}

pub fn parse_input(input: &str) -> Grid {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect()
}

pub mod part1;
pub mod part2;
