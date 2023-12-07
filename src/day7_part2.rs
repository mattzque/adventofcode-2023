#![warn(clippy::pedantic)]
use std::{collections::HashMap, fs::read_to_string};

#[allow(clippy::enum_variant_names)]
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
enum Kind {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    #[default]
    HighCard,
}

impl Kind {
    const ORDER: &'static [Self] = &[
        Self::FiveOfAKind,
        Self::FourOfAKind,
        Self::FullHouse,
        Self::ThreeOfAKind,
        Self::TwoPair,
        Self::OnePair,
        Self::HighCard,
    ];

    fn from_counts(counts: &[u32]) -> Self {
        if counts.len() == 1 {
            Self::FiveOfAKind
        } else if counts.len() == 2 && counts == [4, 1] {
            Self::FourOfAKind
        } else if counts.len() == 2 && counts == [3, 2] {
            Self::FullHouse
        } else if counts.len() == 3 && counts == [3, 1, 1] {
            Self::ThreeOfAKind
        } else if counts.len() == 3 && counts == [2, 2, 1] {
            Self::TwoPair
        } else if counts.len() == 4 && counts == [2, 1, 1, 1] {
            Self::OnePair
        } else {
            Self::HighCard
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Hand {
    cards: [char; 5],
    kind: Kind,
}

impl Hand {
    const ORDER: [char; 13] = [
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ];
}

fn parse_hand(hand: &str) -> Hand {
    assert_eq!(hand.len(), 5);
    let mut counts: HashMap<char, u32> = HashMap::new();
    let mut cards = ['X'; 5];
    let mut jokers = 0;
    for (i, card) in hand.char_indices() {
        cards[i] = card;
        if card == 'J' {
            jokers += 1;
        } else {
            *counts.entry(card).or_default() += 1;
        }
    }
    let mut counts = counts.into_values().collect::<Vec<u32>>();
    counts.sort_unstable();
    counts.reverse();
    if counts.is_empty() {
        counts.push(jokers);
    } else {
        counts[0] += jokers;
    }
    let kind = Kind::from_counts(&counts);
    Hand { cards, kind }
}

fn parse_contents(contents: &str) -> impl Iterator<Item = (Hand, u32)> + '_ {
    contents.trim().lines().map(|line| {
        let (hand, bid) = line.trim().split_once(' ').expect("Invalid Input!");
        (
            parse_hand(hand),
            str::parse::<u32>(bid).expect("Invalid Bid Amount!"),
        )
    })
}

fn sort_hands(hands: &mut [(Hand, u32)]) {
    hands.sort_by_key(|(hand, _)| {
        let kind = Kind::ORDER
            .iter()
            .position(|kind| *kind == hand.kind)
            .unwrap();
        let cards = hand
            .cards
            .iter()
            .map(|card| {
                Hand::ORDER
                    .iter()
                    .position(|card_order| card_order == card)
                    .unwrap()
            })
            .collect::<Vec<_>>();

        (kind, cards)
    });
    hands.reverse();
}

fn calculate_winnings(hands: &[(Hand, u32)]) -> u32 {
    hands
        .iter()
        .enumerate()
        .map(|(index, (_, bid))| {
            let rank = u32::try_from(index).expect("rank cast to u32 error") + 1;
            bid * rank
        })
        .sum()
}

fn main() {
    let contents = read_to_string("day7-input.txt").expect("Invalid Input!");
    let mut cards = parse_contents(&contents).collect::<Vec<_>>();
    sort_hands(&mut cards);
    let winnings = calculate_winnings(&cards);
    println!("Winnings: {winnings}");
}

#[cfg(test)]
mod tests {
    use crate::{calculate_winnings, parse_contents, sort_hands, Hand, Kind};

    const EXAMPLE_INPUT: &str = "
    32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483
    ";

    #[test]
    fn test_parse_contents() {
        let cards = parse_contents(EXAMPLE_INPUT).collect::<Vec<(Hand, u32)>>();
        assert_eq!(
            cards
                .into_iter()
                .map(|(hand, bid)| (hand.kind, bid))
                .collect::<Vec<_>>(),
            &[
                (Kind::OnePair, 765),
                (Kind::FourOfAKind, 684),
                (Kind::TwoPair, 28),
                (Kind::FourOfAKind, 220),
                (Kind::FourOfAKind, 483),
            ]
        );
    }

    #[test]
    fn test_sort_hands() {
        let mut cards = parse_contents(EXAMPLE_INPUT).collect::<Vec<(Hand, u32)>>();
        sort_hands(&mut cards);
        assert_eq!(
            cards.iter().map(|(_, bid)| *bid).collect::<Vec<_>>(),
            &[765, 28, 684, 483, 220]
        );
    }

    #[test]
    fn test_calculate_winnings() {
        let mut cards = parse_contents(EXAMPLE_INPUT).collect::<Vec<(Hand, u32)>>();
        sort_hands(&mut cards);
        let winnings = calculate_winnings(&cards);
        assert_eq!(winnings, 5905);
    }
}
