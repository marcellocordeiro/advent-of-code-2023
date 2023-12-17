use itertools::Itertools;

pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

pub struct Universe {
    galaxies: Vec<Galaxy>,
    i_len: usize,
    j_len: usize,
}

pub struct Galaxy {
    i: usize,
    j: usize,
}

pub fn parse_input(input: &str) -> Universe {
    let lines = input.lines().collect_vec();

    let i_len = lines.len();
    let j_len = lines[0].len();

    let galaxies = lines
        .into_iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.chars()
                .enumerate()
                .filter_map(|(j, ch)| (ch == '#').then_some(Galaxy { i, j }))
                .collect_vec()
        })
        .collect_vec();

    Universe {
        galaxies,
        i_len,
        j_len,
    }
}

pub fn expand_universe(universe: &mut Universe, expanding_factor: usize) {
    let empty_rows = (0..universe.i_len)
        .filter(|i| !universe.galaxies.iter().any(|g| g.i == *i))
        .collect_vec();

    let empty_columns = (0..universe.j_len)
        .filter(|j| !universe.galaxies.iter().any(|g| g.j == *j))
        .collect_vec();

    empty_rows.iter().enumerate().for_each(|(offset, row)| {
        let resolved_row = (offset * (expanding_factor - 1)) + row;

        universe
            .galaxies
            .iter_mut()
            .filter(|g| g.i > resolved_row)
            .for_each(|g| g.i += expanding_factor - 1);
    });

    empty_columns
        .iter()
        .enumerate()
        .for_each(|(offset, column)| {
            let resolved_column = (offset * (expanding_factor - 1)) + column;

            universe
                .galaxies
                .iter_mut()
                .filter(|g| g.j > resolved_column)
                .for_each(|g| g.j += expanding_factor - 1);
        });
}

pub mod part1;
pub mod part2;
