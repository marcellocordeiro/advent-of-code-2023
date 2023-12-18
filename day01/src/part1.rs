pub fn result(lines: &[&str]) -> i32 {
    lines.iter().map(|line| each_result(line)).sum()
}

fn each_result(line: &str) -> i32 {
    let first = line
        .chars()
        .find(|ch| ch.is_numeric())
        .expect("should find a match for the first digit");

    let last = line
        .chars()
        .rfind(|ch| ch.is_numeric())
        .expect("should find a match for the last digit");

    format!("{first}{last}").parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parse_input, INPUT, SAMPLE_PART1};

    #[test]
    fn test_each_sample_line() {
        let lines = SAMPLE_PART1.lines();
        let results = [12, 38, 15, 77];

        assert_eq!(lines.clone().count(), results.len());

        for (line, expected_result) in lines.zip(results) {
            let actual_result = each_result(line);

            assert_eq!(actual_result, expected_result, "for {line}");
        }
    }

    #[test]
    fn test_sample() {
        let lines = parse_input(SAMPLE_PART1);
        let actual_result = result(&lines);

        assert_eq!(actual_result, 142);
    }

    #[test]
    fn test_input() {
        let lines = parse_input(INPUT);
        let actual_result = result(&lines);

        assert_eq!(actual_result, 56049);
    }
}
