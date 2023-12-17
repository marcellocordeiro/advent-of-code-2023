use itertools::Itertools;

pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE_PART1: &str = include_str!("inputs/sample_part1.txt");
pub const SAMPLE1_PART2: &str = include_str!("inputs/sample1_part2.txt");
pub const SAMPLE2_PART2: &str = include_str!("inputs/sample2_part2.txt");
pub const SAMPLE3_PART2: &str = include_str!("inputs/sample3_part2.txt");

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
    let mut tiles = input
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

    let start_tile = get_start_tile(&tiles, start);

    tiles[start.0][start.1] = start_tile;

    Maze {
        tiles,
        start,
        i_len,
        j_len,
    }
}

fn get_start_tile(tiles: &[Vec<Tile>], start: (usize, usize)) -> Tile {
    let dirs = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ]
    .into_iter()
    .filter(|dir| {
        let next = {
            let offset = dir.offset();

            (
                start.0.saturating_add_signed(offset.0),
                start.1.saturating_add_signed(offset.1),
            )
        };

        let expect = dir.moves_to();

        let next_tile = tiles[next.0][next.1];

        let Tile::Connects(next_dirs) = next_tile else {
            return false;
        };

        next_dirs.contains(&expect)
    })
    .sorted()
    .collect_vec();

    Tile::Connects(dirs.try_into().unwrap())
}

pub mod part1;
pub mod part2;
