use crate::{get_visited_count, parse_input, Beam, Direction};

pub fn result(input: &str) -> usize {
    let grid = parse_input(input);

    let rows = grid.len();
    let cols = grid[0].len();

    let mut max = 0;

    for row in 0..rows {
        let beam = Beam {
            position: (row, 0),
            direction: Direction::East,
        };
        let result = get_visited_count(&grid, beam);

        max = max.max(result);

        let beam = Beam {
            position: (row, cols - 1),
            direction: Direction::West,
        };

        let result = get_visited_count(&grid, beam);

        max = max.max(result);
    }

    for col in 0..cols {
        let beam = Beam {
            position: (0, col),
            direction: Direction::South,
        };

        let result = get_visited_count(&grid, beam);

        max = max.max(result);

        let beam = Beam {
            position: (rows - 1, col),
            direction: Direction::North,
        };

        let result = get_visited_count(&grid, beam);

        max = max.max(result);
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let input = SAMPLE;

        let result = result(input);

        assert_eq!(result, 51);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        assert_eq!(result, 7716);
    }
}
