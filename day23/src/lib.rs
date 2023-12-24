pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

pub type Grid = Vec<Vec<char>>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct You {
    position: Position,
    direction: Direction,
    weigth: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position(usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

fn possible_directions(u: &You) -> Vec<Direction> {
    vec![
        u.direction.turn_left(),
        u.direction.turn_right(),
        u.direction,
    ]
}

pub fn parse_input(input: &str) -> Grid {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect()
}

pub mod part1;
pub mod part2;
