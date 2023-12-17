use std::collections::HashSet;

pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

pub type Grid = Vec<Vec<char>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Beam {
    position: (usize, usize),
    direction: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
}

pub enum NextAction {
    Straight(Direction),
    Split([Direction; 2]),
}

pub fn next_action(grid: &Grid, beam: &Beam) -> NextAction {
    let (current_row, current_col) = beam.position;
    let current_tile = grid[current_row][current_col];

    match (beam.direction, current_tile) {
        (Direction::East | Direction::West, '|') => {
            NextAction::Split([Direction::North, Direction::South])
        }

        (Direction::South | Direction::North, '-') => {
            NextAction::Split([Direction::West, Direction::East])
        }

        (Direction::West, '/') => NextAction::Straight(Direction::South),
        (Direction::West, '\\') => NextAction::Straight(Direction::North),

        (Direction::East, '/') => NextAction::Straight(Direction::North),
        (Direction::East, '\\') => NextAction::Straight(Direction::South),

        (Direction::South, '/') => NextAction::Straight(Direction::West),
        (Direction::South, '\\') => NextAction::Straight(Direction::East),

        (Direction::North, '/') => NextAction::Straight(Direction::East),
        (Direction::North, '\\') => NextAction::Straight(Direction::West),

        _ => NextAction::Straight(beam.direction),
    }
}

pub fn next_beams(grid: &Grid, beam: &Beam) -> Vec<Beam> {
    let next_action = next_action(grid, beam);

    let row_len = grid.len();
    let col_len = grid[0].len();

    let (row_cur, col_cur) = beam.position;

    match next_action {
        NextAction::Straight(dir) => {
            let (row_offset, col_offset) = dir.offset();

            let Some(next_row) = row_cur.checked_add_signed(row_offset) else {
                return vec![];
            };

            let Some(next_col) = col_cur.checked_add_signed(col_offset) else {
                return vec![];
            };

            if next_row == row_len || next_col == col_len {
                return vec![];
            }

            vec![Beam {
                position: (next_row, next_col),
                direction: dir,
            }]
        }

        NextAction::Split(dirs) => dirs
            .iter()
            .filter_map(|dir| {
                let (row_offset, col_offset) = dir.offset();

                let Some(next_row) = row_cur.checked_add_signed(row_offset) else {
                    return None;
                };

                let Some(next_col) = col_cur.checked_add_signed(col_offset) else {
                    return None;
                };

                if next_row == row_len || next_col == col_len {
                    return None;
                }

                Some(Beam {
                    position: (next_row, next_col),
                    direction: *dir,
                })
            })
            .collect(),
    }
}

pub fn get_visited_count(grid: &Grid, first_beam: Beam) -> usize {
    let mut visited = HashSet::new();
    let mut history = HashSet::new();

    let mut beams = vec![first_beam];

    while !beams.is_empty() {
        beams = beams
            .into_iter()
            .flat_map(|beam| {
                if history.contains(&beam) {
                    return vec![];
                }

                history.insert(beam);
                visited.insert(beam.position);

                next_beams(grid, &beam)
            })
            .collect();
    }

    visited.len()
}

pub fn parse_input(input: &str) -> Grid {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect()
}

pub mod part1;
pub mod part2;
