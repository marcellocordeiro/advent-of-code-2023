use crate::parse_input;

pub fn result(input: &str) -> usize {
    let guide = parse_input(input);

    let mut steps = 0;
    let mut current_node = guide.nodes.iter().find(|node| node.from == "AAA").unwrap();

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

            if current_node.from == "ZZZ" {
                return steps;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE1_PART1, SAMPLE2_PART1};

    #[test]
    fn test_sample1() {
        let input = SAMPLE1_PART1;

        let result = result(input);

        assert_eq!(result, 2);
    }

    #[test]
    fn test_sample2() {
        let input = SAMPLE2_PART1;

        let result = result(input);

        assert_eq!(result, 6);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        assert_eq!(result, 13207);
    }
}
