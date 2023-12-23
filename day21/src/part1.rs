use std::collections::{HashSet, VecDeque};

use crate::{get_surrounding, parse_input};

pub fn result(input: &str) -> usize {
    let grid = parse_input(input);

    let start = (0..grid.len())
        .find_map(|i| (0..grid[0].len()).find_map(|j| (grid[i][j] == 'S').then_some((i, j))))
        .unwrap();

    //for _ in 0..6 {
    let mut queue = VecDeque::new();
    queue.push_back(start);

    let mut taken_positions = HashSet::new();
    taken_positions.insert(start);

    let mut count = 0;

    while let Some(pos) = queue.pop_front() {
        let possible_positions = get_surrounding(pos);

        possible_positions
            .into_iter()
            .filter_map(|next| {
                let row = grid.get(next.0)?;
                let ch = row.get(next.1)?;

                (*ch == '.' || *ch == 'S').then_some(next)
            })
            .for_each(|position| {
                queue.push_back(position);
                taken_positions.insert(position);
            });

        count += 1;

        if count == 10 {
            break;
        }
    }

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if taken_positions.contains(&(i, j)) {
                print!("O");
            } else {
                print!("{}", grid[i][j]);
            }
        }

        println!();
    }

    println!();
    //}

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let input = SAMPLE;

        let result = result(input);

        //assert_eq!(result, 19114);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        // assert_eq!(result, 353046);
    }
}
