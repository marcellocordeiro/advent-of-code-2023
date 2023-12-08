use crate::parse_input;
use num::Integer;

pub fn result(input: &str) -> usize {
    let map = parse_input(input);

    map.nodes
        .iter()
        .enumerate()
        .filter(|(_, node)| node.from.ends_with('A'))
        .map(|(position, _)| {
            let mut steps = 0;
            let mut current_node = position;

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

                    if map.nodes[current_node].from.ends_with('Z') {
                        return steps;
                    }
                }
            }
        })
        .reduce(|acc, step| acc.lcm(&step))
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
