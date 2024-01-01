use itertools::Itertools;

use crate::{expand_universe, parse_input};

pub fn result(input: &str) -> usize {
    let mut universe = parse_input(input);

    expand_universe(&mut universe, 2);

    universe
        .galaxies
        .iter()
        .tuple_combinations()
        .map(|(a, b)| a.i.abs_diff(b.i) + a.j.abs_diff(b.j))
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

        assert_eq!(result, 374);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        assert_eq!(result, 9623138);
    }
}
