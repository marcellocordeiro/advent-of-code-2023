use crate::parse_input;

pub fn result(input: &str) -> usize {
    let mut platform = parse_input(input);

    for _ in 0..1_000_000_000 {
        platform.drop_north();
        platform.drop_west();
        platform.drop_south();
        platform.drop_east();
    }

    platform.total_load()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        // let input = SAMPLE;

        // let result = result(input);

        // assert_eq!(result, 136);
    }

    #[test]
    fn test_input() {
        // let input = INPUT;

        // let result = result(input);

        // assert_eq!(result, 108955);
    }
}
