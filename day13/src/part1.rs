use crate::{find_column, find_row, parse_input};

pub fn result(input: &str) -> usize {
    let maps = parse_input(input);

    maps.iter()
        .map(|map| {
            if let Some(row) = find_row(map, None) {
                row * 100
            } else if let Some(column) = find_column(map, None) {
                column
            } else {
                panic!("Nothing was found");
            }
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

        assert_eq!(result, 405);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        assert_eq!(result, 30158);
    }
}
