use crate::parse_input;

pub fn result(input: &str) -> i32 {
    let _ = parse_input(input);

    0 // todo!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE1_PART2, SAMPLE2_PART2};

    #[test]
    fn test_sample1() {
        let input = SAMPLE1_PART2;

        let _ = result(input);

        // assert_eq!(result, 114);
    }

    #[test]
    fn test_sample2() {
        let input = SAMPLE2_PART2;

        let _ = result(input);

        // assert_eq!(result, 114);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let _ = result(input);

        // assert_eq!(result, 1_901_217_887);
    }
}
