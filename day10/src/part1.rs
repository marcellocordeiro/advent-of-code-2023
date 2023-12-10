use itertools::Itertools;

use crate::{parse_input, Direction, Maze, Tile};

pub fn result(input: &str) -> usize {
    let maze = parse_input(input);

    let lengths = (maze.i_len, maze.j_len);

    let mut marked_maze = (0..lengths.0)
        .map(|_| vec![-1_i64; lengths.1])
        .collect::<Vec<_>>();

    let count = go_cycle(&maze, &mut marked_maze);

    (count + 1) / 2
}

fn go_cycle(maze: &Maze, marked_maze: &mut [Vec<i64>]) -> usize {
    let mut count = 0;
    let (i_start, j_start) = maze.start;

    marked_maze[i_start][j_start] = 0;

    let start_tile = get_start_tile(maze);

    let mut current_dir = match start_tile {
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

    while marked_maze[next_position.0][next_position.1] == -1 {
        count += 1;
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
            Tile::Start => break,
            Tile::Ground => panic!("err"),
        };

        next_position = {
            let offset = current_dir.offset();

            (
                current_position.0.saturating_add_signed(offset.0),
                current_position.1.saturating_add_signed(offset.1),
            )
        };
    }

    count
}

fn get_start_tile(maze: &Maze) -> Tile {
    let dirs = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ]
    .into_iter()
    .filter(|dir| {
        let next = {
            let offset = dir.offset();

            (
                maze.start.0.saturating_add_signed(offset.0),
                maze.start.1.saturating_add_signed(offset.1),
            )
        };

        let expect = dir.moves_to();

        let next_tile = maze.tiles[next.0][next.1];

        let Tile::Connects(next_dirs) = next_tile else {
            return false;
        };

        next_dirs.contains(&expect)
    })
    .sorted()
    .collect_vec();

    Tile::Connects(dirs.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let input = SAMPLE;

        let result = result(input);

        assert_eq!(result, 4);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        assert_eq!(result, 6831);
    }
}
