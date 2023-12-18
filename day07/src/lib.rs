pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

pub struct Hand {
    pub cards: String,
    pub bid: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

pub fn parse_input(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();

            let cards = cards.to_owned();
            let bid = bid.parse().unwrap();

            Hand { cards, bid }
        })
        .collect()
}

pub fn sort_hands<ToHandType>(
    hands: &mut [Hand],
    to_hand_type: ToHandType,
    card_strength: &'static [char; 13],
) where
    ToHandType: Fn(&str) -> HandType,
{
    hands.sort_by(|Hand { cards: a, .. }, Hand { cards: b, .. }| {
        let a_hand_type = to_hand_type(a);
        let b_hand_type = to_hand_type(b);

        if a_hand_type == b_hand_type {
            for (a_card, b_card) in a.chars().zip(b.chars()) {
                if a_card == b_card {
                    continue;
                }

                let a_strength = card_strength.iter().position(|ch| *ch == a_card).unwrap();
                let b_strength = card_strength.iter().position(|ch| *ch == b_card).unwrap();

                return a_strength.cmp(&b_strength).reverse();
            }

            unreachable!("Cards {a} and {b} are equal");
        }

        (a_hand_type).cmp(&b_hand_type)
    });
}

pub mod part1;
pub mod part2;
