use day3::{convert_input, parse_engine, EngineNumber, EngineSymbol, INPUT};

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

fn part1(numbers: &[EngineNumber], symbols: &[EngineSymbol]) -> i32 {
    numbers.iter().fold(0, |acc, number| {
        acc + if has_surrounding_symbol(number, symbols) {
            number.number
        } else {
            0
        }
    })
}

fn main() {
    let lines = convert_input(INPUT);
    let (numbers, symbols) = parse_engine(&lines);

    let result = part1(&numbers, &symbols);

    println!("Result: {result}");
}

#[cfg(test)]
mod tests {

    use day3::SAMPLE;

    use super::*;

    #[test]
    fn test_all() {
        let lines = convert_input(SAMPLE);
        let (numbers, symbols) = parse_engine(&lines);

        let result = part1(&numbers, &symbols);

        assert_eq!(result, 4361);
    }

    #[test]
    fn test_input() {
        let lines = convert_input(INPUT);
        let (numbers, symbols) = parse_engine(&lines);

        let result = part1(&numbers, &symbols);

        assert_eq!(result, 530495);
    }
}
