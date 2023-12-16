use crate::{hash, parse_input};

pub fn result(input: &str) -> usize {
    let items = parse_input(input);
    items.into_iter().map(|item| hash(item) as usize).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_pre_sample() {
        let input = "HASH";

        let result = result(input);

        assert_eq!(result, 52);
    }

    #[test]
    fn test_sample() {
        let input = SAMPLE;

        let result = result(input);

        assert_eq!(result, 1320);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        assert_eq!(result, 522547);
    }
}
