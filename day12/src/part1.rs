use std::collections::HashMap;

use crate::parse_input;

pub fn result(input: &str) -> usize {
    let groups = parse_input(input);

    /* for group in groups {
        let p = permutations(group.conditions.to_owned(), 0, &group.ranges);

        println!("{p}");
    } */

    let mut cache = HashMap::new();

    groups
        .iter()
        .map(|group| permutations(group.conditions.to_owned(), 0, &group.ranges, &mut cache))
        .sum()
}

fn permutations<'a>(
    working_str: String,
    slice_start: usize,
    ranges: &'a [usize],
    cache: &mut HashMap<(String, usize, &'a [usize]), usize>,
) -> usize {
    let key = (working_str.clone(), slice_start, ranges);

    if let Some(count) = cache.get(&key) {
        return *count;
    }

    if slice_start == working_str.len() {
        if !is_correct(&working_str, ranges) {
            return 0;
        }

        println!("{working_str}");
        return 1;
    }

    let working_copy = working_str.clone();

    let count = match working_str.chars().nth(slice_start).unwrap() {
        '?' => {
            let mut perm1 = working_str.to_owned();
            perm1.replace_range(slice_start..slice_start + 1, "#");

            let mut perm2 = working_str.to_owned();
            perm2.replace_range(slice_start..slice_start + 1, ".");

            let a = permutations(perm1, slice_start + 1, ranges, cache);
            let b = permutations(perm2, slice_start + 1, ranges, cache);

            a + b
        }

        _ => permutations(working_str, slice_start + 1, ranges, cache),
    };

    cache.insert((working_copy, slice_start, ranges), count);

    count
}

fn is_correct(springs: &String, ranges: &[usize]) -> bool {
    let springs = springs.replace('.', " ");
    let springs_iter = springs.split_whitespace();

    if springs_iter.clone().count() != ranges.len() {
        return false;
    }

    springs_iter
        .zip(ranges)
        .all(|(group, range)| group.len() == *range)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let input = SAMPLE;

        let result = result(input);

        assert_eq!(result, 21);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        assert_eq!(result, 7633);
    }
}
