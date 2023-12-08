use crate::parse_input;

pub fn result(input: &str) -> i32 {
    let map = parse_input(input);

    let mut steps = 0;
    let mut current_node = map
        .nodes
        .iter()
        .position(|node| node.from == "AAA")
        .unwrap();

    loop {
        for dir in map.instructions.chars() {
            let next_node = match dir {
                'L' => &map.nodes[current_node].to_l,
                'R' => &map.nodes[current_node].to_r,

                _ => unreachable!(),
            };

            current_node = map
                .nodes
                .iter()
                .position(|node| node.from == *next_node)
                .unwrap();

            steps += 1;

            if map.nodes[current_node].from == "ZZZ" {
                return steps;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE1, SAMPLE2};

    #[test]
    fn test_sample1() {
        let input = SAMPLE1;

        let result = result(input);

        assert_eq!(result, 2);
    }

    #[test]
    fn test_sample2() {
        let input = SAMPLE2;

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
