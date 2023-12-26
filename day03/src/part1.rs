use crate::{get_surrounding_coordinates, EngineNumber, EngineSymbol};

pub fn result(numbers: &[EngineNumber], symbols: &[EngineSymbol]) -> i32 {
    numbers
        .iter()
        .filter_map(|n| has_surrounding_symbol(n, symbols).then_some(n.number))
        .sum()
}

fn has_surrounding_symbol(number: &EngineNumber, symbols: &[EngineSymbol]) -> bool {
    for column in number.column_range.clone() {
        for (row, column) in get_surrounding_coordinates((number.row, column)) {
            let found_symbol = symbols.iter().any(|s| s.row == row && s.column == column);

            if found_symbol {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parse_input, INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let (numbers, symbols) = parse_input(SAMPLE);

        let result = result(&numbers, &symbols);

        assert_eq!(result, 4361);
    }

    #[test]
    fn test_input() {
        let (numbers, symbols) = parse_input(INPUT);

        let result = result(&numbers, &symbols);

        assert_eq!(result, 530495);
    }
}
