use crate::parse_input;
use num::Integer;

pub fn result(input: &str) -> usize {
    let guide = parse_input(input);

    guide
        .nodes
        .iter()
        .filter(|node| node.from.ends_with('A'))
        .map(|starting_node| {
            let mut steps = 0;
            let mut current_node = starting_node;

            loop {
                for dir in guide.instructions.chars() {
                    let next_node = match dir {
                        'L' => &current_node.to_l,
                        'R' => &current_node.to_r,

                        _ => unreachable!("Invalid direction"),
                    };

                    current_node = guide
                        .nodes
                        .iter()
                        .find(|node| node.from == *next_node)
                        .unwrap();

                    steps += 1;

                    if current_node.from.ends_with('Z') {
                        return steps;
                    }
                }
            }
        })
        .reduce(|acc, steps| acc.lcm(&steps))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE3};

    #[test]
    fn test_sample3() {
        let input = SAMPLE3;

        let result = result(input);

        assert_eq!(result, 6);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        assert_eq!(result, 12_324_145_107_121);
    }
}
