use itertools::Itertools;

pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

#[derive(Debug)]
struct Brick {
    id: usize,
    from: (usize, usize, usize),
    to: (usize, usize, usize),
}

impl Brick {
    fn is_supporting(&self, other: &Self) -> bool {
        (self.to.2 + 1) == other.from.2
            && ranges_overlap((self.from.0, self.to.0), (other.from.0, other.to.0))
            && ranges_overlap((self.from.1, self.to.1), (other.from.1, other.to.1))
    }

    fn supporting(&self, others: &[Self]) -> Vec<usize> {
        others
            .iter()
            .enumerate()
            .filter_map(|(index, b)| b.is_supporting(self).then_some(index))
            .collect_vec()
    }
}

fn ranges_overlap((start_a, end_a): (usize, usize), (start_b, end_b): (usize, usize)) -> bool {
    assert!(start_a <= end_a);
    assert!(start_b <= end_b);

    start_a <= end_b && start_b <= end_a
}

fn parse_input(input: &str) -> Vec<Brick> {
    input
        .lines()
        .enumerate()
        .map(|(id, line)| {
            let (from, to) = line.split_once('~').unwrap();

            let from = {
                let (x, y, z) = from
                    .split(',')
                    .map(|x| x.parse().unwrap())
                    .collect_tuple()
                    .unwrap();

                (x, y, z)
            };

            let to = {
                let (x, y, z) = to
                    .split(',')
                    .map(|x| x.parse().unwrap())
                    .collect_tuple()
                    .unwrap();

                (x, y, z)
            };

            Brick { id, from, to }
        })
        .sorted_by(|a, b| a.from.2.cmp(&b.from.2))
        .collect()
}

pub mod part1;
pub mod part2;
