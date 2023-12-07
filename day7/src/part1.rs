use crate::{parse_input, Hand, HandType};
use itertools::Itertools;
use std::collections::HashMap;

pub const CARD_STRENGTH: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

pub fn to_hand_type(cards: &str) -> HandType {
    let mut hash_map = HashMap::<char, usize>::new();

    cards.chars().for_each(|ch| {
        *hash_map.entry(ch).or_default() += 1;
    });

    let counts = hash_map.values().copied().sorted().collect::<Vec<_>>();

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

pub fn sort_hands(hands: &mut [Hand]) {
    hands.sort_by(|Hand { cards: a, .. }, Hand { cards: b, .. }| {
        let a_hand_type = to_hand_type(a);
        let b_hand_type = to_hand_type(b);

        if a_hand_type == b_hand_type {
            for (a_card, b_card) in a.chars().zip(b.chars()) {
                if a_card == b_card {
                    continue;
                }

                let a_strength = CARD_STRENGTH.iter().position(|ch| *ch == a_card).unwrap();
                let b_strength = CARD_STRENGTH.iter().position(|ch| *ch == b_card).unwrap();

                return a_strength.cmp(&b_strength).reverse();
            }

            unreachable!("Cards {a} and {b} are equal");
        }

        (a_hand_type).cmp(&b_hand_type)
    });
}

pub fn result(input: &str) -> usize {
    let mut hands = parse_input(input);

    sort_hands(&mut hands);

    hands
        .iter()
        .enumerate()
        .map(|(rank, Hand { bid, .. })| (rank + 1) * bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let input = SAMPLE;

        let result = result(input);

        assert_eq!(result, 6440);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        assert_eq!(result, 249_390_788);
    }
}
