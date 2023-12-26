pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

type Grid = Vec<Vec<char>>;

fn get_surrounding(grid: &Grid, (i, j): (usize, usize)) -> Vec<(usize, usize)> {
    [(-1, 0), (1, 0), (0, 1), (0, -1)]
        .into_iter()
        .filter_map(|offset| {
            let next_i = i.checked_add_signed(offset.0)?;
            let next_j = j.checked_add_signed(offset.1)?;

            let row = grid.get(next_i)?;
            let ch = row.get(next_j)?;

            (*ch != '#').then_some((next_i, next_j))
        })
        .collect()
}

fn parse_input(input: &str) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub mod part1;
pub mod part2;
