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
                let color = line.split_whitespace().collect::<Vec<&str>>()[2];

                let color = &color[2..(color.len() - 1)];

                let steps = &color[0..5];
                let direction = &color[5..];
                assert!(direction.len() == 1);

                (direction, steps)
            };

            let direction = Direction::from_hex_ch(direction.chars().next().unwrap());
            let steps = isize::from_str_radix(steps, 16).unwrap();

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

        assert_eq!(result, 952408144115);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        assert_eq!(result, 96556251590677);
    }
}
