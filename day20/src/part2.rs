pub fn result(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE1, SAMPLE2};

    #[test]
    fn test_sample1() {
        let input = SAMPLE1;

        let _result = result(input);

        // assert_eq!(result, 19114);
    }

    #[test]
    fn test_sample2() {
        let input = SAMPLE2;

        let _result = result(input);

        // assert_eq!(result, 19114);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let _result = result(input);

        // assert_eq!(result, 353046);
    }
}
