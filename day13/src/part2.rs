use itertools::Itertools;
use crate::{parse_input, Object};

pub fn result(input: &str) -> usize {
    let maps = parse_input(input);

    maps.into_iter()
        .map(|map| {
            let regular_row_result = find_row(&map, None).map(|x| x - 1);
            let regular_column_result = find_column(&map, None).map(|x| x - 1);

            for row in 0..map.len() {
                for column in 0..map[0].len() {
                    let new_map = {
                        let mut cloned = map.clone();
                        cloned[row][column] = cloned[row][column].flip();

                        cloned
                    };

                    let result = if let Some(row) = find_row(&new_map, regular_row_result) {
                        Some(row * 100)
                    } else if let Some(column) = find_column(&new_map, regular_column_result) {
                        Some(column)
                    } else {
                        None
                    };

                    if let Some(result) = result {
                        return result;
                    }
                }
            }

            panic!();
        })
        .sum()
}

fn find_row(map: &[Vec<Object>], ignore: Option<usize>) -> Option<usize> {
    let pair = (0..map.len())
        .tuple_windows()
        .find(|(up_start, down_start)| {
            if Some(*up_start) == ignore {
                return false;
            }

            // println!("Up start = {up_start} / Down start = {down_start}");
            (0..=*up_start)
                .rev()
                .zip(*down_start..map.len())
                .all(|(up, down)| map[up] == map[down])
        })?;

    Some(pair.0 + 1)
}

fn find_column(map: &[Vec<Object>], ignore: Option<usize>) -> Option<usize> {
    let row_size = map[0].len();

    let pair = (0..row_size)
        .tuple_windows()
        .find(|(left_start, right_start)| {
            if Some(*left_start) == ignore {
                return false;
            }

            (0..=*left_start)
                .rev()
                .zip(*right_start..row_size)
                .all(|(left, right)| {
                    let left_column_iter = map.iter().map(|row| row[left]);
                    let right_column_iter = map.iter().map(|row| row[right]);

                    left_column_iter.eq(right_column_iter)
                })
        })?;

    Some(pair.0 + 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let input = SAMPLE;

        let result = result(input);

        assert_eq!(result, 400);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        assert_eq!(result, 36474);
    }
}
