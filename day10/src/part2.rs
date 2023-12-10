use crate::parse_input;

pub fn result(input: &str) -> i32 {
    let _ = parse_input(input);

    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let input = SAMPLE;

        let result = result(input);

        // assert_eq!(result, 114);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        // assert_eq!(result, 1_901_217_887);
    }
}
