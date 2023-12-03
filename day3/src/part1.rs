use crate::{EngineNumber, EngineSymbol};

fn has_surrounding_symbol(number: &EngineNumber, symbols: &[EngineSymbol]) -> bool {
    for col_index in number.range.clone() {
        let row_start = number.line_index.saturating_sub(1);
        let row_end = number.line_index.saturating_add(1);

        let col_start = col_index.saturating_sub(1);
        let col_end = col_index.saturating_add(1);

        let row_range = row_start..=row_end;
        let col_range = col_start..=col_end;

        for col in col_range {
            for row in row_range.clone() {
                if symbols
                    .iter()
                    .any(|symbol| symbol.line_index == row && symbol.position == col)
                {
                    return true;
                }
            }
        }
    }

    false
}

pub fn result(numbers: &[EngineNumber], symbols: &[EngineSymbol]) -> i32 {
    numbers
        .iter()
        .filter_map(|n| has_surrounding_symbol(n, symbols).then_some(n.number))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parse_engine, INPUT, SAMPLE};
    use common::split_by_line;

    #[test]
    fn test_all() {
        let lines = split_by_line(SAMPLE);
        let (numbers, symbols) = parse_engine(&lines);

        let result = result(&numbers, &symbols);

        assert_eq!(result, 4361);
    }

    #[test]
    fn test_input() {
        let lines = split_by_line(INPUT);
        let (numbers, symbols) = parse_engine(&lines);

        let result = result(&numbers, &symbols);

        assert_eq!(result, 530495);
    }
}
