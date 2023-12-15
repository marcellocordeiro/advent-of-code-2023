use crate::{parse_input, permutations};

pub fn result(input: &str) -> usize {
    let groups = parse_input(input);

    groups
        .iter()
        .map(|group| permutations(&group.conditions, &group.ranges))
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

        assert_eq!(result, 21);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        assert_eq!(result, 7633);
    }
}
