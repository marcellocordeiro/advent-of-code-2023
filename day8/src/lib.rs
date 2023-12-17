pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE1_PART1: &str = include_str!("inputs/sample1_part1.txt");
pub const SAMPLE2_PART1: &str = include_str!("inputs/sample2_part1.txt");
pub const SAMPLE_PART2: &str = include_str!("inputs/sample_part2.txt");

pub struct Guide {
    pub instructions: String,
    pub nodes: Vec<Node>,
}

pub struct Node {
    from: String,
    to_l: String,
    to_r: String,
}

pub fn parse_input(input: &str) -> Guide {
    let (instructions, nodes) = input.trim().split_once("\n\n").unwrap();

    let instructions = instructions.to_owned();

    let nodes = nodes
        .lines()
        .map(|node| {
            let (key, value) = node.split_once(" = ").unwrap();

            let key = key.to_owned();
            let value = {
                let (l, r) = value[1..(value.len() - 1)].split_once(", ").unwrap();

                let l = l.to_owned();
                let r = r.to_owned();

                (l, r)
            };

            Node {
                from: key,
                to_l: value.0,
                to_r: value.1,
            }
        })
        .collect();

    Guide {
        instructions,
        nodes,
    }
}

pub mod part1;
pub mod part2;
