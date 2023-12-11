use itertools::Itertools;

use crate::Galaxy;

pub fn result(input: &str) -> usize {
    let galaxies = parse_input(input);

    galaxies
        .iter()
        .tuple_combinations()
        .map(|(a, b)| a.i.abs_diff(b.i) + a.j.abs_diff(b.j))
        .sum()
}

fn parse_input(input: &str) -> Vec<Galaxy> {
    let universe = input.lines().map(str::to_owned).collect_vec();

    let empty_rows = universe
        .iter()
        .enumerate()
        .filter_map(|(i, row)| row.chars().all(|ch| ch == '.').then_some(i))
        .collect_vec();

    let empty_columns = {
        let mut empty = Vec::new();

        for j in 0..universe[0].len() {
            let mut all_equal = true;
            for row in &universe {
                if row.chars().nth(j).unwrap() != '.' {
                    all_equal = false;
                    break;
                }
            }

            if all_equal {
                empty.push(j);
            }
        }

        empty
    };

    let mut galaxies = universe
        .into_iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.chars()
                .enumerate()
                .filter_map(|(j, ch)| (ch == '#').then_some(Galaxy { i, j }))
                .collect_vec()
        })
        .collect_vec();

    let expanding_factor = 2;

    empty_rows.iter().enumerate().for_each(|(offset, row)| {
        let resolved_row = (offset * (expanding_factor - 1)) + row;

        galaxies
            .iter_mut()
            .filter(|g| g.i > resolved_row)
            .for_each(|g| g.i += expanding_factor - 1);
    });

    empty_columns
        .iter()
        .enumerate()
        .for_each(|(offset, column)| {
            let resolved_column = (offset * (expanding_factor - 1)) + column;

            galaxies
                .iter_mut()
                .filter(|g| g.j > resolved_column)
                .for_each(|g| g.j += expanding_factor - 1);
        });

    galaxies
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let input = SAMPLE;

        let result = result(input);

        assert_eq!(result, 374);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        assert_eq!(result, 9_623_138);
    }
}
