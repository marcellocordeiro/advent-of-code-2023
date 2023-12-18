use crate::{get_shortest_path, parse_input};

const MIN_STEPS: usize = 0;
const MAX_STEPS: usize = 3;

pub fn result(input: &str) -> usize {
    let grid = parse_input(input);

    get_shortest_path::<MIN_STEPS, MAX_STEPS>(&grid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE1, SAMPLE2};

    #[test]
    fn test_sample1() {
        let input = SAMPLE1;

        let result = result(input);

        assert_eq!(result, 102);
    }

    #[test]
    fn test_sample2() {
        let input = SAMPLE2;

        let result = result(input);

        assert_eq!(result, 59);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        assert_eq!(result, 1110);
    }
}
