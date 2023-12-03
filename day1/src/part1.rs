fn each_result(line: &str) -> i32 {
    let first = line
        .chars()
        .find(|c| c.is_numeric())
        .expect("should find a match for the first digit");

    let last = line
        .chars()
        .rfind(|c| c.is_numeric())
        .expect("should find a match for the last digit");

    format!("{first}{last}").parse::<i32>().unwrap()
}

pub fn result(lines: &[&str]) -> i32 {
    lines.iter().fold(0, |acc, line| acc + each_result(line))
}

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use common::split_by_line;

    use crate::{INPUT, SAMPLE_PART1};

    use super::*;

    #[test]
    fn test_each() {
        let lines = split_by_line(SAMPLE_PART1);
        let results = [12, 38, 15, 77];

        assert_eq!(lines.len(), results.len());

        let zipped = zip(lines, results);

        for (line, expected_result) in zipped {
            let actual_result = each_result(&line);

            assert_eq!(actual_result, expected_result);
        }
    }

    #[test]
    fn test_all() {
        let lines = split_by_line(SAMPLE_PART1);
        let actual_result = result(&lines);

        assert_eq!(actual_result, 142);
    }

    #[test]
    fn test_input() {
        let lines = split_by_line(INPUT);
        let actual_result = result(&lines);

        assert_eq!(actual_result, 56049);
    }
}
