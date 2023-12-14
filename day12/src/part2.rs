use std::collections::HashMap;

use crate::parse_input;

pub fn result(input: &str) -> usize {
    //return 0;
    let groups = parse_input(input);

    /* for group in groups {
        let p = permutations(group.conditions.to_owned(), 0, &group.ranges);

        println!("{p}");
    } */

    groups
        .iter()
        .map(|group| {
            let mut cache = HashMap::new();

            let new_cond = group.conditions.clone()
                + "?"
                + &group.conditions.clone()
                + "?"
                + &group.conditions.clone()
                + "?"
                + &group.conditions.clone()
                + "?"
                + &group.conditions.clone();

            let new_ranges = group.ranges.repeat(5);
            // permutations(&new_cond, &new_ranges, &mut cache)
            permutations(&new_cond, &new_ranges, &mut cache)
        })
        .sum()
}

fn permutations<'a>(
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
            permutations(&conditions[1..], ranges, cache) + permutations(&attempt, ranges, cache)
        }

        '.' => permutations(&conditions[1..], ranges, cache),

        '#' => {
            if conditions.len() < current_range || conditions[..current_range].contains('.') {
                0
            } else if conditions.len() == current_range {
                (ranges.len() == 1) as usize
            } else if conditions.chars().nth(current_range) == Some('#') {
                0
            } else {
                permutations(&conditions[current_range + 1..], &ranges[1..], cache)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let input = SAMPLE;

        let result = result(input);

        assert_eq!(result, 525152);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        assert_eq!(result, 23903579139437);
    }
}
