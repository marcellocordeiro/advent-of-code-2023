fn part1(lines: &[&str]) -> i32 {
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

        let actual_result = part1(&input);

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let expected_result = 56049;

        let lines: Vec<&str> = input.trim().split('\n').collect();
        let actual_result = part1(&lines);

        assert_eq!(actual_result, expected_result);
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let lines: Vec<&str> = input.trim().split('\n').collect();

    let result = part1(&lines);

    println!("Result: {result}");
}
