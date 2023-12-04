use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs::read_to_string,
};

fn parse_numbers(numbers: &str) -> Vec<u8> {
    numbers.split(' ').flat_map(str::parse::<u8>).collect()
}

fn parse_input(contents: &str) -> Vec<(Vec<u8>, Vec<u8>)> {
    contents
        .trim()
        .lines()
        .map(|line| {
            let lindex = line.find(':').expect("Invalid Input!");
            let (winning, numbers) = line[lindex + 1..].split_once('|').expect("Invalid Input!");
            (parse_numbers(winning), parse_numbers(numbers))
        })
        .collect()
}

/// For each card return the index/number of the card and the number of winning numbers.
fn find_winning_numbers(cards: &[(Vec<u8>, Vec<u8>)]) -> Vec<(usize, usize)> {
    cards
        .iter()
        .enumerate()
        .map(|(index, (winning, numbers))| {
            let winning: HashSet<&u8> = HashSet::from_iter(winning);
            let numbers: HashSet<&u8> = HashSet::from_iter(numbers);
            (index, winning.intersection(&numbers).count())
        })
        .collect()
}

fn calculate_won_cards(scores: &[(usize, usize)]) -> usize {
    // card index -> number of cards
    let mut cards: HashMap<usize, usize> = HashMap::new();
    let scores: BTreeMap<usize, usize> = BTreeMap::from_iter(scores.iter().copied());
    let mut current = scores.keys().copied().collect::<Vec<usize>>();

    while !current.is_empty() {
        let mut next_cards = Vec::new();
        for index in current {
            *cards.entry(index).or_default() += 1;

            let score = scores.get(&index).cloned().unwrap_or(0);
            // you win one copy of the next <score> cards...
            (index..index + score).for_each(|won_card_index| {
                if won_card_index + 1 < scores.len() {
                    next_cards.push(won_card_index + 1);
                }
            });
        }
        current = next_cards;
    }

    cards.values().sum()
}

fn main() {
    let contents = read_to_string("day4-input.txt").expect("Invalid Input!");
    let cards = parse_input(&contents);
    let winning = find_winning_numbers(&cards);
    let num = calculate_won_cards(&winning);
    println!("Total Cards: {}", num);
}

#[cfg(test)]
mod tests {
    use crate::{calculate_won_cards, find_winning_numbers, parse_input};

    const EXAMPLE_INPUT: &str = "
    Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    ";

    #[test]
    fn test_parse_input() {
        let cards = parse_input(EXAMPLE_INPUT);
        assert_eq!(
            cards,
            vec![
                (vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53]),
                (
                    vec![13, 32, 20, 16, 61],
                    vec![61, 30, 68, 82, 17, 32, 24, 19]
                ),
                (vec![1, 21, 53, 59, 44], vec![69, 82, 63, 72, 16, 21, 14, 1]),
                (
                    vec![41, 92, 73, 84, 69],
                    vec![59, 84, 76, 51, 58, 5, 54, 83]
                ),
                (
                    vec![87, 83, 26, 28, 32],
                    vec![88, 30, 70, 12, 93, 22, 82, 36]
                ),
                (
                    vec![31, 18, 13, 56, 72],
                    vec![74, 77, 10, 23, 35, 67, 36, 11]
                )
            ]
        );
    }

    #[test]
    fn test_find_winning_numbers() {
        let cards = parse_input(EXAMPLE_INPUT);
        let winning = find_winning_numbers(&cards);
        assert_eq!(
            winning,
            vec![(0, 4), (1, 2), (2, 2), (3, 1), (4, 0), (5, 0),]
        );
    }

    #[test]
    fn test_calculate_won_cards() {
        let cards = parse_input(EXAMPLE_INPUT);
        let winning = find_winning_numbers(&cards);
        let num = calculate_won_cards(&winning);
        assert_eq!(num, 30);
    }
}
