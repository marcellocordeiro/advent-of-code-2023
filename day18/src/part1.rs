use crate::{calculate_area, Direction, Entry};

pub fn result(input: &str) -> isize {
    let entries = parse_input(input);

    calculate_area(entries)
}

fn parse_input(input: &str) -> Vec<Entry> {
    input
        .lines()
        .map(|line| {
            let (direction, steps) = {
                let split = line.split_whitespace().collect::<Vec<&str>>();
                (split[0], split[1])
            };

            let direction = Direction::from_ch(direction.chars().next().unwrap());
            let steps = steps.parse().unwrap();

            Entry { direction, steps }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let input = SAMPLE;

        let result = result(input);

        assert_eq!(result, 62);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        assert_eq!(result, 50603);
    }
}
