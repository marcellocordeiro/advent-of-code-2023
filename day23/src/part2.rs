use std::collections::HashSet;

use colored::Colorize;

use crate::{parse_input, Grid};

pub fn result(input: &str) -> usize {
    let grid = parse_input(input);

    find_max(&grid)
}

fn find_max(grid: &Grid) -> usize {
    let mut visited = vec![Vec::new(); grid.len()];

    for row in &mut visited {
        *row = vec![false; grid[0].len()];
    }

    let start = (0, 1);
    let end = (grid.len() - 1, grid[0].len() - 2);

    find_max_impl(grid, start, &end, &mut visited, 0).unwrap()
}

fn find_max_impl(
    grid: &Grid,
    point: (usize, usize),
    end: &(usize, usize),
    visited: &mut Vec<Vec<bool>>,
    current: usize,
) -> Option<usize> {
    if visited[point.0][point.1] {
        return None;
    }

    if point == *end {
        return Some(current);
    }

    visited[point.0][point.1] = true;

    let max = [(-1, 0), (1, 0), (0, 1), (0, -1)]
        .into_iter()
        .filter_map(|offset| {
            let next_i = point.0.checked_add_signed(offset.0)?;
            let next_j = point.1.checked_add_signed(offset.1)?;

            Some((next_i, next_j))
        })
        .filter(|(i, j)| grid[*i][*j] != '#')
        .filter_map(|(i, j)| find_max_impl(grid, (i, j), end, visited, current + 1))
        .max();

    visited[point.0][point.1] = false;

    max
}

// Attempt at another algorithm, keeping it here for later.
#[allow(dead_code)]
fn get_intersections(grid: &Grid) {
    let mut intersections = HashSet::new();

    for i in 1..(grid.len() - 1) {
        for j in 1..(grid[0].len() - 1) {
            if grid[i][j] == '#' {
                continue;
            }

            let is_intersection = [(-1, 0), (1, 0), (0, 1), (0, -1)]
                .into_iter()
                .filter_map(|offset| {
                    let next_i = i.checked_add_signed(offset.0)?;
                    let next_j = j.checked_add_signed(offset.1)?;

                    Some((next_i, next_j))
                })
                .filter(|(i, j)| grid[*i][*j] != '#')
                .count()
                > 2;

            if is_intersection {
                intersections.insert((i, j));
            }
            //.collect();
        }
    }

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if intersections.contains(&(i, j)) {
                print!("{}", "X".red());
            } else {
                print!("{}", grid[i][j]);
            }
        }

        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let input = SAMPLE;

        let result = result(input);

        assert_eq!(result, 154);
    }

    #[ignore = "takes very long to compute. Refactor the algorithm. Actually rewrite the whole dang thing."]
    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        assert_eq!(result, 6418);
    }
}
