use crate::{extrapolate, parse_input};

pub fn result(input: &str) -> i64 {
    let sequences = parse_input(input);

    sequences
        .iter()
        .map(|seq| seq.iter().rev().copied().collect::<Vec<_>>())
        .map(|seq| seq.last().unwrap() + extrapolate(&seq))
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

        assert_eq!(result, 2);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        assert_eq!(result, 905);
    }
}
