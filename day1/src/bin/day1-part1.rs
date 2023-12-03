use day1::INPUT;

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

fn result(lines: &[&str]) -> i32 {
    lines.iter().fold(0, |acc, line| acc + each_result(line))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_each() {
        let lines_with_result = [
            ("1abc2", 12),
            ("pqr3stu8vwx", 38),
            ("a1b2c3d4e5f", 15),
            ("treb7uchet", 77),
        ];

        for (line, expected_result) in lines_with_result {
            let actual_result = each_result(&line);

            assert_eq!(actual_result, expected_result);
        }
    }

    #[test]
    fn test_all() {
        let lines = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        let expected_result = 142;

        let actual_result = result(&lines);

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_input() {
        let lines = INPUT.trim().split('\n').collect::<Vec<&str>>();
        let expected_result = 56049;

        let actual_result = result(&lines);

        assert_eq!(actual_result, expected_result);
    }
}

fn main() {
    let lines = INPUT.trim().split('\n').collect::<Vec<&str>>();
    let result = result(&lines);

    println!("Result: {result}");
}
