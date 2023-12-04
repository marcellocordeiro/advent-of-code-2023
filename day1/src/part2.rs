pub fn result(lines: &[&str]) -> i32 {
    lines.iter().map(|line| each_result(line)).sum()
}

fn each_result(line: &str) -> i32 {
    let first = find_first(line).expect("should find a match for the first digit");
    let last = find_last(line).expect("should find a match for the last digit");

    (first * 10) + last
}

const DIGITS_STR: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn find_first(x: &str) -> Option<i32> {
    for i in 0..x.len() {
        let window = &x[i..];

        if window.starts_with(char::is_numeric) {
            let number = window.chars().next().unwrap().to_digit(10).unwrap();

            return Some(number as i32);
        }

        for (i, digit_str) in DIGITS_STR.iter().enumerate() {
            if window.starts_with(digit_str) {
                return Some(i as i32);
            }
        }
    }

    None
}

fn find_last(x: &str) -> Option<i32> {
    for i in (0..x.len()).rev() {
        let window = &x[0..=i];

        if window.ends_with(char::is_numeric) {
            let number = window.chars().last().unwrap().to_digit(10).unwrap();

            return Some(number as i32);
        }

        for (i, digit_str) in DIGITS_STR.iter().enumerate() {
            if window.ends_with(digit_str) {
                return Some(i as i32);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE_PART2};
    use common::split_by_line;

    #[test]
    fn test_each() {
        let lines = split_by_line(SAMPLE_PART2);
        let results = [29, 83, 13, 24, 42, 14, 76];

        assert_eq!(lines.len(), results.len());

        for (line, expected_result) in lines.into_iter().zip(results) {
            let actual_result = each_result(line);

            assert_eq!(actual_result, expected_result, "for: {line}");
        }
    }

    #[test]
    fn test_all() {
        let lines = split_by_line(SAMPLE_PART2);
        let actual_result = result(&lines);

        assert_eq!(actual_result, 281);
    }

    #[test]
    fn test_input() {
        let lines = split_by_line(INPUT);
        let actual_result = result(&lines);

        assert_eq!(actual_result, 54530);
    }
}
