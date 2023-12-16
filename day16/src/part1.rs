use crate::{get_visited_count, parse_input, Beam};

pub fn result(input: &str) -> usize {
    let grid = parse_input(input);

    let first_beam = Beam {
        position: (0, 0),
        direction: crate::Direction::East,
    };

    get_visited_count(&grid, first_beam)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let input = SAMPLE;

        let result = result(input);

        assert_eq!(result, 46);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        assert_eq!(result, 7472);
    }
}
