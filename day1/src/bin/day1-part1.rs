fn part1(lines: &[&str]) -> i32 {
    lines.iter().fold(0, |acc, x| {
        let first = { x.chars().find(|c| c.is_numeric()) };
        let second = { x.chars().rfind(|c| c.is_numeric()) };

        if let (Some(first), Some(second)) = (first, second) {
            let number = format!("{first}{second}").parse::<i32>().unwrap();

            return acc + number;
        }

        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        let answer = part1(&input);

        assert_eq!(answer, 142);
    }
}

fn main() {
    let contents = include_str!("../input.txt");
    let lines: Vec<&str> = contents.trim().split('\n').collect();

    let answer = part1(&lines);

    println!("Answer: {answer}");
}
