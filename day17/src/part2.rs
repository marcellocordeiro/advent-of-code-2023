use crate::{parse_input, Direction, Grid, Position};
use pathfinding::directed::dijkstra::dijkstra;

pub fn result(input: &str) -> usize {
    let grid = parse_input(input);

    get_shortest_path(&grid)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Crucible {
    position: Position,
    direction: Direction,
    remaining: usize,
    weigth: usize,
}

impl Crucible {
    fn next_crucibles(&self, grid: &Grid) -> Vec<(Self, usize)> {
        let possible = if self.remaining <= 6 {
            vec![
                (self.direction.turn_left(), 10),
                (self.direction.turn_right(), 10),
                (self.direction, self.remaining),
            ]
        } else {
            vec![(self.direction, self.remaining)]
        };

        possible
            .into_iter()
            .filter(|(_, remaining)| *remaining > 0)
            .filter_map(|(dir, remaining)| {
                let Position(i, j) = self.position;

                let offset = dir.offset();
                let new_i = i.checked_add_signed(offset.0)?;
                let new_j = j.checked_add_signed(offset.1)?;

                let row = grid.get(new_i)?;
                let cost = *row.get(new_j)?;

                Some(Self {
                    position: Position(new_i, new_j),
                    direction: dir,
                    remaining: remaining - 1,
                    weigth: cost,
                })
            })
            .map(|crucible| {
                let weight = crucible.weigth;
                (crucible, weight)
            })
            .collect()
    }
}

pub fn get_shortest_path(grid: &Grid) -> usize {
    let start_crucible_1 = Crucible {
        position: Position(0, 0),
        direction: Direction::East,
        remaining: 10,
        weigth: 0,
    };

    let start_crucible_2 = Crucible {
        position: Position(0, 0),
        direction: Direction::South,
        remaining: 10,
        weigth: 0,
    };

    let end_position = Position(grid.len() - 1, grid[0].len() - 1);

    let results = [start_crucible_1, start_crucible_2]
        .into_iter()
        .map(|c| {
            dijkstra(
                &c,
                |c| c.next_crucibles(grid),
                |c| c.position == end_position,
            )
            .unwrap()
        })
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

        assert_eq!(result, 1294);
    }
}
