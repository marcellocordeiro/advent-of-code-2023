pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE: &str = include_str!("sample.txt");

#[derive(Debug)]
pub struct Maze {
    tiles: Vec<Vec<Tile>>,
    start: (usize, usize),
    i_len: usize,
    j_len: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Connects([Direction; 2]),
    Ground,
    Start,
}

impl Tile {
    fn from_ch(ch: char) -> Self {
        match ch {
            '|' => Self::Connects([Direction::North, Direction::South]),
            '-' => Self::Connects([Direction::East, Direction::West]),
            'L' => Self::Connects([Direction::North, Direction::East]),
            'J' => Self::Connects([Direction::North, Direction::West]),
            '7' => Self::Connects([Direction::South, Direction::West]),
            'F' => Self::Connects([Direction::South, Direction::East]),
            '.' => Self::Ground,
            'S' => Self::Start,

            ch => panic!("Invalid char: {ch}"),
        }
    }

    pub fn to_ch(self) -> char {
        match self {
            Self::Connects(dirs) => match dirs {
                [Direction::North, Direction::South] => '|',
                [Direction::East, Direction::West] => '-',
                [Direction::North, Direction::East] => 'L',
                [Direction::North, Direction::West] => 'J',
                [Direction::South, Direction::West] => '7',
                [Direction::South, Direction::East] => 'F',

                _ => panic!("Invalid tile"),
            },
            Self::Ground => '.',
            Self::Start => 'S',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn offset(self) -> (isize, isize) {
        match self {
            Self::North => (-1, 0),
            Self::South => (1, 0),
            Self::East => (0, 1),
            Self::West => (0, -1),
        }
    }

    fn moves_to(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
}

pub fn parse_input(input: &str) -> Maze {
    let tiles = input
        .lines()
        .map(|line| line.chars().map(Tile::from_ch).collect())
        .collect::<Vec<Vec<Tile>>>();

    let i_len = tiles.len();
    let j_len = tiles[0].len();

    let mut start = None;

    for (i, row) in tiles.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            if *tile == Tile::Start {
                start = Some((i, j));
            }
        }
    }

    let start = start.unwrap();

    Maze {
        tiles,
        start,
        i_len,
        j_len,
    }
}

pub mod part1;
pub mod part2;
