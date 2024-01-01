use std::collections::BTreeMap;

use crate::{number_matches_count, Card};

pub fn result(cards: &[Card]) -> i32 {
    card_count(cards).values().sum()
}

fn card_count(cards: &[Card]) -> BTreeMap<i32, i32> {
    let mut map = BTreeMap::new();

    for card in cards {
        *map.entry(card.id).or_default() += 1;

        let current_card_copies = *map.entry(card.id).or_default();
        let matches_count = number_matches_count(card) as i32;

        let new_copy_ids = (card.id + 1)..((card.id + 1) + matches_count);

        for copy_id in new_copy_ids {
            *map.entry(copy_id).or_default() += current_card_copies;
        }
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parse_input, INPUT, SAMPLE};

    #[test]
    fn test_sample_card_count() {
        let cards = parse_input(SAMPLE);
        let results = [8, 2, 2, 1, 0, 0];

        assert_eq!(cards.len(), results.len());

        let map = card_count(&cards);

        assert_eq!(
            map.into_iter().collect::<Vec<_>>(),
            [(1, 1), (2, 2), (3, 4), (4, 8), (5, 14), (6, 1)]
        );
    }

    #[test]
    fn test_sample() {
        let cards = parse_input(SAMPLE);
        let result = result(&cards);

        assert_eq!(result, 30);
    }

    #[test]
    fn test_input() {
        let cards = parse_input(INPUT);

        let result = result(&cards);

        assert_eq!(result, 6283755);
    }
}
