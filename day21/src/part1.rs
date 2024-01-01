use std::collections::HashSet;

use crate::{get_surrounding, parse_input, Grid};

pub fn result(input: &str) -> usize {
    let grid = parse_input(input);

    possible_positions_count::<64>(&grid)
}

fn possible_positions_count<const MAX_STEPS: usize>(grid: &Grid) -> usize {
    let start = (0..grid.len())
        .find_map(|i| (0..grid[0].len()).find_map(|j| (grid[i][j] == 'S').then_some((i, j))))
        .unwrap();

    let mut taken_positions = HashSet::new();
    taken_positions.insert(start);

    for _ in 0..MAX_STEPS {
        taken_positions = taken_positions
            .into_iter()
            .flat_map(|pos| get_surrounding(grid, pos))
            .collect();

        /* for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if taken_positions.contains(&(i, j)) {
                    print!("O");
                } else {
                    print!("{}", grid[i][j]);
                }
            }

            println!();
        }

        println!(); */
    }

    taken_positions.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let input = SAMPLE;
        let grid = parse_input(input);

        let result = possible_positions_count::<6>(&grid);

        assert_eq!(result, 16);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        assert_eq!(result, 3743);
    }
}
