use itertools::Itertools;

use crate::{parse_input, Maze, Tile};

pub fn result(input: &str) -> i64 {
    let maze = parse_input(input);

    let lengths = (maze.i_len, maze.j_len);

    let mut marked_maze = (0..lengths.0)
        .map(|_| vec![-1_i64; lengths.1])
        .collect::<Vec<_>>();

    let coords = go_cycle(&maze, &mut marked_maze);

    // Another idea: https://en.wikipedia.org/wiki/Point_in_polygon

    // Area inside (Shoelace formula)
    // https://en.wikipedia.org/wiki/Shoelace_formula
    let mut area: i64 = 0;
    let n = coords.len() as i64;
    for w in coords.windows(2) {
        area += (w[0].0 * w[1].1) as i64;
        area -= (w[0].1 * w[1].0) as i64;
    }

    let area = area.abs() / 2;

    // Tiles inside (Pick's theorem)
    // https://en.wikipedia.org/wiki/Pick's_theorem
    area - (n / 2) + 1
}

fn go_cycle(maze: &Maze, marked_maze: &mut [Vec<i64>]) -> Vec<(usize, usize)> {
    let mut coords = Vec::new();

    let (i_start, j_start) = maze.start;

    marked_maze[i_start][j_start] = 0;

    let mut current_dir = match maze.tiles[i_start][j_start] {
        Tile::Connects(dir) => dir[0],
        _ => panic!("err"),
    };

    let mut current_position = (i_start, j_start);
    let mut next_position = {
        let offset = current_dir.offset();

        (
            maze.start.0.saturating_add_signed(offset.0),
            maze.start.1.saturating_add_signed(offset.1),
        )
    };

    coords.push(current_position);
    coords.push(next_position);

    while marked_maze[next_position.0][next_position.1] == -1 {
        let previous_value = marked_maze[current_position.0][current_position.1];

        current_position = next_position;

        marked_maze[next_position.0][next_position.1] = previous_value + 1;

        let next_tile = maze.tiles[next_position.0][next_position.1];

        current_dir = match next_tile {
            Tile::Connects(dirs) => dirs
                .into_iter()
                .filter(|dir| *dir != current_dir.moves_to())
                .exactly_one()
                .unwrap(),
            _ => panic!("err"),
        };

        next_position = {
            let offset = current_dir.offset();

            (
                current_position.0.saturating_add_signed(offset.0),
                current_position.1.saturating_add_signed(offset.1),
            )
        };

        coords.push(next_position);
    }

    coords
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE1_PART2, SAMPLE2_PART2, SAMPLE3_PART2};

    #[test]
    fn test_sample1() {
        let input = SAMPLE1_PART2;

        let result = result(input);

        assert_eq!(result, 4);
    }

    #[test]
    fn test_sample2() {
        let input = SAMPLE2_PART2;

        let result = result(input);

        assert_eq!(result, 8);
    }

    #[test]
    fn test_sample3() {
        let input = SAMPLE3_PART2;

        let result = result(input);

        assert_eq!(result, 10);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        assert_eq!(result, 305);
    }
}
