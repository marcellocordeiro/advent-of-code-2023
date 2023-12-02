use day1::INPUT;

fn result(lines: &[&str]) -> i32 {
    lines.iter().fold(0, |acc, line| {
        let first = line
            .chars()
            .find(|c| c.is_numeric())
            .expect("should find a match for the first digit");

        let last = line
            .chars()
            .rfind(|c| c.is_numeric())
            .expect("should find a match for the last digit");

        let number = format!("{first}{last}").parse::<i32>().unwrap();

        acc + number
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        let input = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        let expected_result = 142;

        let actual_result = result(&input);

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_input() {
        let expected_result = 56049;

        let lines = INPUT.trim().split('\n').collect::<Vec<&str>>();
        let actual_result = result(&lines);

        assert_eq!(actual_result, expected_result);
    }
}

fn main() {
    let lines = INPUT.trim().split('\n').collect::<Vec<&str>>();

    let result = result(&lines);

    println!("Result: {result}");
}
