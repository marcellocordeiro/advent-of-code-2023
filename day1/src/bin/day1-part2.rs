use day1::INPUT;

const DIGITS_STR: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn find_first(x: &str) -> Option<i32> {
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

pub fn find_last(x: &str) -> Option<i32> {
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

fn each_result(line: &str) -> i32 {
    let first = find_first(line).expect("should find a match for the first digit");
    let last = find_last(line).expect("should find a match for the last digit");

    (first * 10) + last
}

fn result(lines: &[&str]) -> i32 {
    lines.iter().fold(0, |acc, line| acc + each_result(line))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_each() {
        let lines_with_result = [
            ("two1nine", 29),
            ("eightwothree", 83),
            ("abcone2threexyz", 13),
            ("xtwone3four", 24),
            ("4nineeightseven2", 42),
            ("zoneight234", 14),
            ("7pqrstsixteen", 76),
        ];

        for (line, expected_result) in lines_with_result {
            let actual_result = each_result(line);

            assert_eq!(actual_result, expected_result, "for: {line}");
        }
    }

    #[test]
    fn test_all() {
        let lines = [
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];
        let expected_result = 281;

        let actual_result = result(&lines);

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_input() {
        let lines = INPUT.trim().split('\n').collect::<Vec<&str>>();
        let expected_result = 54530;

        let actual_result = result(&lines);

        assert_eq!(actual_result, expected_result);
    }
}

fn main() {
    let lines = INPUT.trim().split('\n').collect::<Vec<&str>>();
    let result = result(&lines);

    println!("Result: {result}");
}
