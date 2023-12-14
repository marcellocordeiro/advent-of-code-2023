use crate::{parse_input, Platform};

pub fn result(input: &str) -> usize {
    let mut platform = parse_input(input);
    platform.drop_north();

    Platform::total_load(&platform.map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let input = SAMPLE;

        let result = result(input);

        assert_eq!(result, 136);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        assert_eq!(result, 108955);
    }
}
