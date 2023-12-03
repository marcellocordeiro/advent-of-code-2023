use crate::{EngineNumber, EngineSymbol};

fn is_gear(symbol: &EngineSymbol) -> bool {
    symbol.symbol == '*'
}

fn get_surrounding_numbers<'a>(
    symbol: &'a EngineSymbol,
    numbers: &'a [EngineNumber],
) -> Vec<&'a EngineNumber> {
    let row_start = symbol.line_index.saturating_sub(1);
    let row_end = symbol.line_index.saturating_add(1);

    let col_start = symbol.position.saturating_sub(1);
    let col_end = symbol.position.saturating_add(1);

    let row_range = row_start..=row_end;
    let col_range = col_start..=col_end;

    numbers
        .iter()
        .filter(move |n| {
            row_range.contains(&n.line_index) && col_range.clone().any(|i| n.range.contains(&i))
        })
        .collect()
}

pub fn result(numbers: &[EngineNumber], symbols: &[EngineSymbol]) -> i32 {
    symbols
        .iter()
        .filter(|s| is_gear(s))
        .fold(0, |acc, symbol| {
            let surrounding_numbers = get_surrounding_numbers(symbol, numbers);

            if surrounding_numbers.len() == 2 {
                let product = surrounding_numbers[0].number * surrounding_numbers[1].number;
                return acc + product;
            }

            acc
        })
}

#[cfg(test)]
mod tests {

    use crate::{convert_input, parse_engine, INPUT, SAMPLE};

    use super::*;

    #[test]
    fn test_all() {
        let lines = convert_input(SAMPLE);
        let (numbers, symbols) = parse_engine(&lines);

        let result = result(&numbers, &symbols);

        assert_eq!(result, 467835);
    }

    #[test]
    fn test_input() {
        let lines = convert_input(INPUT);
        let (numbers, symbols) = parse_engine(&lines);

        let result = result(&numbers, &symbols);

        assert_eq!(result, 80253814);
    }
}
