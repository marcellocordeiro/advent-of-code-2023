use itertools::Itertools;
use pathfinding::directed::bfs::bfs;

use crate::{parse_input, possible_directions, Direction, Grid, Position, You};

pub fn result(input: &str) -> usize {
    let grid = parse_input(input);

    find_max(&grid)
}

fn next_positions(u: &You, grid: &Grid) -> Vec<You> {
    let possible = possible_directions(u);

    let next = possible.into_iter().filter_map(|direction| {
        let Position(i, j) = u.position;

        let offset = direction.offset();
        let new_i = i.checked_add_signed(offset.0)?;
        let new_j = j.checked_add_signed(offset.1)?;

        let row = grid.get(new_i)?;
        let ch = *row.get(new_j)?;

        let is_slope = matches!(ch, '^' | '>' | 'v' | '<');

        match (direction, ch) {
            (Direction::North, 'v') => return None,
            (Direction::East, '<') => return None,
            (Direction::South, '^') => return None,
            (Direction::West, '>') => return None,

            _ => {}
        }

        if ch != '.' && !is_slope {
            return None;
        }

        Some((
            is_slope,
            You {
                position: Position(new_i, new_j),
                direction,
                weigth: u.weigth + 1,
            },
        ))
    });

    let has_slope = next.clone().any(|(is_slope, _)| is_slope);

    next.into_iter()
        .filter(|(is_slope, _new_u)| if has_slope { *is_slope } else { true })
        .map(|(_, new_u)| new_u)
        .collect_vec()
}

fn find_max(grid: &Grid) -> usize {
    let start_u = You {
        position: Position(0, 1),
        direction: Direction::South,
        weigth: 0,
    };

    let end = Position(grid.len() - 1, grid[0].len() - 2);

    let successors = |u: &You| next_positions(u, grid);

    let mut steps = vec![];

    loop {
        let Some(results) = ({
            let success = |u: &You| u.position == end && !steps.contains(&u.weigth);

            bfs(&start_u, successors, success)
        }) else {
            break;
        };

        steps.push(results.len() - 1);
    }

    steps.into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let input = SAMPLE;

        let result = result(input);

        assert_eq!(result, 94);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        assert_eq!(result, 2294);
    }
}
