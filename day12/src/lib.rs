use std::collections::HashMap;

pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

pub struct Springs {
    conditions: String,
    ranges: Vec<usize>,
}

pub fn parse_input(input: &str) -> Vec<Springs> {
    input
        .lines()
        .map(|line| {
            let (conditions, ranges) = line.split_once(' ').unwrap();

            let conditions = conditions.to_owned();
            let ranges = ranges.split(',').map(|num| num.parse().unwrap()).collect();

            Springs { conditions, ranges }
        })
        .collect()
}

fn permutations(conditions: &str, ranges: &[usize]) -> usize {
    let mut cache = HashMap::new();

    permutations_impl(conditions, ranges, &mut cache)
}

fn permutations_impl<'a>(
    conditions: &str,
    ranges: &'a [usize],
    cache: &mut HashMap<(String, &'a [usize]), usize>,
) -> usize {
    let key = (conditions.to_owned(), ranges);

    if let Some(count) = cache.get(&key) {
        return *count;
    }

    let Some(current_range) = ranges.first().copied() else {
        return (!conditions.contains('#')) as usize;
    };

    let Some(current_ch) = conditions.chars().next() else {
        return 0;
    };

    let count = match current_ch {
        '?' => {
            let attempt = conditions.replacen('?', "#", 1);
            permutations_impl(&conditions[1..], ranges, cache)
                + permutations_impl(&attempt, ranges, cache)
        }

        '.' => permutations_impl(&conditions[1..], ranges, cache),

        '#' => {
            if conditions.len() < current_range || conditions[..current_range].contains('.') {
                0
            } else if conditions.len() == current_range {
                (ranges.len() == 1) as usize
            } else if conditions.chars().nth(current_range) == Some('#') {
                0
            } else {
                permutations_impl(&conditions[current_range + 1..], &ranges[1..], cache)
            }
            /*let Some(with_range) = conditions.get(..current_range) else {
                break 'lab 0;
            };

            if conditions.chars().nth(current_range) == Some('#') {
                break 'lab 0;
            }

            if with_range.contains('.') {
                break 'lab 0;
            }

            let Some(new_range) = conditions.get((current_range + 1)..) else {
                break 'lab 0;
            };

            permutations(new_range, &ranges[1..], cache)*/
        }

        _ => panic!(),
    };

    cache.insert(key, count);

    count
}

pub mod part1;
pub mod part2;
