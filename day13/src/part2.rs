use crate::{find_column, find_row, parse_input};

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
                    } else {
                        find_column(&new_map, regular_column_result)
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
