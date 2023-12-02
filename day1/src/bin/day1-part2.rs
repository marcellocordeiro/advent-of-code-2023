const STR_NUMBERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn find_first(x: &str) -> Option<i32> {
    for i in 0..x.len() {
        let window = &x[i..];

        if window.starts_with(char::is_numeric) {
            let number = window.chars().next().unwrap().to_digit(10).unwrap();

            return Some(number as i32);
        }

        for (i, str_number) in STR_NUMBERS.iter().enumerate() {
            if window.starts_with(str_number) {
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

        for (i, str_number) in STR_NUMBERS.iter().enumerate() {
            if window.ends_with(str_number) {
                return Some(i as i32);
            }
        }
    }

    None
}

fn part2(lines: &[&str]) -> i32 {
    lines.iter().fold(0, |acc, x| {
        let first = find_first(x).expect("should find a match for the first digit");
        let last = find_last(x).expect("should find a match for the last digit");

        let number = (first * 10) + last;

        acc + number
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_each() {
        let input = [
            ("two1nine", 29),
            ("eightwothree", 83),
            ("abcone2threexyz", 13),
            ("xtwone3four", 24),
            ("4nineeightseven2", 42),
            ("zoneight234", 14),
            ("7pqrstsixteen", 76),
        ];

        for (line, expected_result) in input {
            let actual_result = part2(&[line]);

            assert_eq!(actual_result, expected_result, "for: {line}");
        }
    }

    #[test]
    fn test_all() {
        let input = [
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];

        let expected_result = 281;

        let actual_result = part2(&input);

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let expected_result = 54530;

        let lines: Vec<&str> = input.trim().split('\n').collect();

        let actual_result = part2(&lines);

        assert_eq!(actual_result, expected_result);
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let lines: Vec<&str> = input.trim().split('\n').collect();

    let result = part2(&lines);

    println!("Result: {result}");
}
