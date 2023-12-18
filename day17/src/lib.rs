use pathfinding::directed::dijkstra::dijkstra;

pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE1: &str = include_str!("inputs/sample1.txt");
pub const SAMPLE2: &str = include_str!("inputs/sample2.txt");

pub type Grid = Vec<Vec<usize>>;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Crucible {
    position: Position,
    direction: Direction,
    remaining: usize,
    weight: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position(usize, usize);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

fn possible_directions(
    c: &Crucible,
    min_steps: usize,
    max_steps: usize,
) -> Vec<(Direction, usize)> {
    let mut possible = vec![];

    if c.remaining <= (max_steps - min_steps) {
        possible.push((c.direction.turn_left(), max_steps));
        possible.push((c.direction.turn_right(), max_steps));
    }

    if c.remaining > 0 {
        possible.push((c.direction, c.remaining));
    }

    possible
}

fn next_crucibles(
    c: &Crucible,
    grid: &Grid,
    min_steps: usize,
    max_steps: usize,
) -> Vec<(Crucible, usize)> {
    let possible = possible_directions(c, min_steps, max_steps);

    possible
        .into_iter()
        .filter_map(|(direction, remaining)| {
            let Position(i, j) = c.position;

            let offset = direction.offset();
            let new_i = i.checked_add_signed(offset.0)?;
            let new_j = j.checked_add_signed(offset.1)?;

            let row = grid.get(new_i)?;
            let weight = *row.get(new_j)?;

            Some(Crucible {
                position: Position(new_i, new_j),
                direction,
                remaining: remaining - 1,
                weight,
            })
        })
        .map(|c| {
            let weight = c.weight;
            (c, weight)
        })
        .collect()
}

fn get_shortest_path(grid: &Grid, min_steps: usize, max_steps: usize) -> usize {
    let starting_crucibles = [
        Crucible {
            position: Position(0, 0),
            direction: Direction::East,
            remaining: max_steps,
            weight: 0,
        },
        Crucible {
            position: Position(0, 0),
            direction: Direction::South,
            remaining: max_steps,
            weight: 0,
        },
    ];

    let successors = |c: &Crucible| next_crucibles(c, grid, min_steps, max_steps);

    let end_position = Position(grid.len() - 1, grid[0].len() - 1);
    let success =
        |c: &Crucible| c.position == end_position && c.remaining <= (max_steps - min_steps);

    let results = starting_crucibles
        .into_iter()
        .map(|start| dijkstra(&start, successors, success).unwrap())
        .collect::<Vec<_>>();

    /*let visited = &results[0].0;
    let result = results[0].1;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if visited
                .iter()
                .find(|c| c.position == Position(i, j))
                .is_some()
            {
                print!("*");
            } else {
                print!(".");
            }
        }

        println!();
    }*/

    usize::min(results[0].1, results[1].1)
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
