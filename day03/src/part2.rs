use crate::{get_surrounding_coordinates, EngineNumber, EngineSymbol};

pub fn result(numbers: &[EngineNumber], symbols: &[EngineSymbol]) -> i32 {
    symbols
        .iter()
        .filter(|s| is_gear(s))
        .filter_map(|s| {
            let surrounding_numbers = get_surrounding_numbers(s, numbers);

            (surrounding_numbers.len() == 2)
                .then_some(surrounding_numbers.into_iter().product::<i32>())
        })
        .sum()
}

fn is_gear(symbol: &EngineSymbol) -> bool {
    symbol.symbol == '*'
}

fn get_surrounding_numbers(symbol: &EngineSymbol, numbers: &[EngineNumber]) -> Vec<i32> {
    let surrounding_coordinates = get_surrounding_coordinates((symbol.row, symbol.column));

    numbers
        .iter()
        .filter_map(|n| {
            surrounding_coordinates
                .iter()
                .any(|(row, column)| *row == n.row && n.column_range.contains(column))
                .then_some(n.number)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parse_input, INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let (numbers, symbols) = parse_input(SAMPLE);

        let result = result(&numbers, &symbols);

        assert_eq!(result, 467835);
    }

    #[test]
    fn test_input() {
        let (numbers, symbols) = parse_input(INPUT);

        let result = result(&numbers, &symbols);

        assert_eq!(result, 80253814);
    }
}
