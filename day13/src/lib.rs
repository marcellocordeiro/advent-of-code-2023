use itertools::Itertools;

pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Object {
    Ash,
    Rock,
}

impl Object {
    #[must_use]
    fn from_ch(ch: char) -> Self {
        match ch {
            '.' => Self::Ash,
            '#' => Self::Rock,

            _ => panic!(),
        }
    }

    #[must_use]
    pub fn flip(self) -> Self {
        match self {
            Self::Ash => Self::Rock,
            Self::Rock => Self::Ash,
        }
    }
}

pub fn find_row(map: &[Vec<Object>], ignore: Option<usize>) -> Option<usize> {
    let pair = (0..map.len())
        .tuple_windows()
        .find(|(up_start, down_start)| {
            if Some(*up_start) == ignore {
                return false;
            }

            (0..=*up_start)
                .rev()
                .zip(*down_start..map.len())
                .all(|(up, down)| map[up] == map[down])
        })?;

    Some(pair.0 + 1)
}

pub fn find_column(map: &[Vec<Object>], ignore: Option<usize>) -> Option<usize> {
    let row_size = map[0].len();

    let pair = (0..row_size)
        .tuple_windows()
        .find(|(left_start, right_start)| {
            if Some(*left_start) == ignore {
                return false;
            }

            (0..=*left_start)
                .rev()
                .zip(*right_start..row_size)
                .all(|(left, right)| {
                    let left_column_iter = map.iter().map(|row| row[left]);
                    let right_column_iter = map.iter().map(|row| row[right]);

                    left_column_iter.eq(right_column_iter)
                })
        })?;

    Some(pair.0 + 1)
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
