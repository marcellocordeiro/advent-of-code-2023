use std::collections::HashMap;

use crate::{hash, parse_input};

pub fn result(input: &str) -> usize {
    let items = parse_input(input);
    let mut boxes = HashMap::<u8, Vec<(&str, usize)>>::new();

    for item in items {
        if let Some((label, focal_length)) = item.split_once('=') {
            let hash = hash(label);
            let focal_length = focal_length.parse::<usize>().unwrap();

            let values = boxes.entry(hash).or_default();

            if let Some(current_focal_length) = values
                .iter_mut()
                .find(|item| item.0 == label)
                .map(|item| &mut item.1)
            {
                *current_focal_length = focal_length;
            } else {
                values.push((label, focal_length));
            }
        } else if let Some(label) = item.strip_suffix('-') {
            let hash = hash(label);

            let values = boxes.get_mut(&hash);

            if let Some(values) = values {
                if let Some(index) = values.iter().position(|item| item.0 == label) {
                    values.remove(index);
                }
            }
        } else {
            panic!("Neither = or -");
        }
    }

    dbg!(&boxes);

    boxes
        .into_iter()
        .map(|(k, v)| {
            v.iter()
                .enumerate()
                .map(|(i, val)| ((k as usize) + 1) * (i + 1) * val.1)
                .sum::<usize>()
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let input = SAMPLE;

        let result = result(input);

        assert_eq!(result, 145);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        assert_eq!(result, 229271);
    }
}
