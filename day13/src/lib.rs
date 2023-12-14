pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE: &str = include_str!("sample.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Object {
    Ash,
    Rock,
}

impl Object {
    fn from_ch(ch: char) -> Self {
        match ch {
            '.' => Self::Ash,
            '#' => Self::Rock,

            _ => panic!(),
        }
    }

    pub fn flip(self) -> Self {
        match self {
            Self::Ash => Self::Rock,
            Self::Rock => Self::Ash,
        }
    }
}

pub fn parse_input(input: &str) -> Vec<Vec<Vec<Object>>> {
    input
        .trim()
        .split("\n\n")
        .map(|map| {
            map.lines()
                .map(|line| line.chars().map(Object::from_ch).collect())
                .collect()
        })
        .collect()
}

pub mod part1;
pub mod part2;
