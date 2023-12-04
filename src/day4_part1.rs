use std::{collections::HashSet, fs::read_to_string};

fn parse_numbers(numbers: &str) -> Vec<u8> {
    numbers
        .split(' ')
        .flat_map(str::parse::<u8>)
        .collect()
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

fn find_winning_numbers(cards: &[(Vec<u8>, Vec<u8>)]) -> Vec<Vec<u8>> {
    cards
        .iter()
        .map(|(winning, numbers)| {
            let winning: HashSet<&u8> = HashSet::from_iter(winning);
            let numbers: HashSet<&u8> = HashSet::from_iter(numbers);
            winning
                .intersection(&numbers)
                .map(|number| **number)
                .collect()
        })
        .collect()
}

fn calculate_score(winning: &[Vec<u8>]) -> u32 {
        winning.iter().map(|winners| {
            let score = winners.len() as i32;
            if score > 0 {
                2_i32.pow((score - 1) as u32) as u32
            } else {
                0
            }
        }).sum()

}

fn main() {
    let contents = read_to_string("day4-input.txt").expect("Invalid Input!");
    let cards = parse_input(&contents);
    let winning = find_winning_numbers(&cards);
    let total: u32 = calculate_score(&winning);

    println!("Total: {}", total);
}

#[cfg(test)]
mod tests {
    use crate::{find_winning_numbers, parse_input, calculate_score};

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
        let mut winning = find_winning_numbers(&cards);
        winning.iter_mut().for_each(|numbers| numbers.sort());
        assert_eq!(
            winning,
            vec![
                vec![17, 48, 83, 86],
                vec![32, 61],
                vec![1, 21],
                vec![84],
                vec![],
                vec![]
            ]
        );
    }

    #[test]
    fn test_calculate_score() {
        let cards = parse_input(EXAMPLE_INPUT);
        let winning = find_winning_numbers(&cards);
        let total = calculate_score(&winning);
        assert_eq!(total, 13);
    }
}
