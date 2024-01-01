use std::collections::HashMap;

use itertools::Itertools;

use crate::{parse_input, sort_hands, Hand, HandType};

const CARD_STRENGTH: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

pub fn result(input: &str) -> usize {
    let mut hands = parse_input(input);

    sort_hands(&mut hands, to_optimized_hand_type, &CARD_STRENGTH);

    hands
        .iter()
        .enumerate()
        .map(|(rank, Hand { bid, .. })| (rank + 1) * bid)
        .sum()
}

fn to_optimized_hand_type(cards: &str) -> HandType {
    let mut hash_map = HashMap::<char, usize>::new();

    cards.chars().for_each(|ch| {
        *hash_map.entry(ch).or_default() += 1;
    });

    let wildcard_count = hash_map.remove(&'J').unwrap_or(0);
    let mut counts = hash_map.values().copied().sorted().collect::<Vec<_>>();

    #[allow(clippy::option_if_let_else)]
    if let Some(last_ref) = counts.last_mut() {
        *last_ref += wildcard_count;
    } else {
        counts.push(wildcard_count);
    }

    match counts.as_slice() {
        [1, 1, 1, 1, 1] => HandType::HighCard,
        [1, 1, 1, 2] => HandType::OnePair,
        [1, 2, 2] => HandType::TwoPair,
        [1, 1, 3] => HandType::ThreeOfAKind,
        [2, 3] => HandType::FullHouse,
        [1, 4] => HandType::FourOfAKind,
        [5] => HandType::FiveOfAKind,

        _ => panic!("Invalid hand: {counts:?}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let input = SAMPLE;

        let result = result(input);

        assert_eq!(result, 5905);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        assert_eq!(result, 248750248);
    }
}
