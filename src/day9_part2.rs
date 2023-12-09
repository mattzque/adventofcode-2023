#![warn(clippy::pedantic)]
use std::fs::read_to_string;

fn parse_contents(contents: &str) -> Vec<Vec<i64>> {
    contents
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .split(' ')
                .map(|number| str::parse::<i64>(number).expect("Invalid Number!"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn sliding_windows<T, const N: usize>(list: &[T]) -> impl Iterator<Item = &[T]> {
    (0..=list.len() - N).map(move |index| &list[index..index + N])
}

fn differences(numbers: &[Vec<i64>]) -> Vec<Vec<Vec<i64>>> {
    numbers
        .iter()
        .map(|numbers| {
            let mut differences = Vec::new();
            let mut current = numbers.clone();
            loop {
                current = sliding_windows::<_, 2>(&current)
                    .map(|pair| pair[1] - pair[0])
                    .collect::<Vec<_>>();
                if current.iter().sum::<i64>() == 0 {
                    break;
                }
                differences.push(current.clone());
            }
            differences
        })
        .collect::<Vec<_>>()
}

fn find_previous_numbers(numbers: &[Vec<i64>], diffs: &[Vec<Vec<i64>>]) -> Vec<i64> {
    numbers
        .iter()
        .enumerate()
        .map(|(i, numbers)| {
            let mut num = 0;
            diffs[i].iter().rev().for_each(|diffs| {
                let first = diffs.first().unwrap();
                num = first - num;
            });
            numbers.first().unwrap() - num
        })
        .collect::<Vec<_>>()
}

fn main() {
    let contents = read_to_string("day9-input.txt").expect("Invalid Input!");
    let numbers = parse_contents(&contents);
    let diffs = differences(&numbers);
    let previous_numbers = find_previous_numbers(&numbers, &diffs);
    println!("Solution: {}", previous_numbers.iter().sum::<i64>());
}

#[cfg(test)]
mod tests {
    use crate::{differences, find_previous_numbers, parse_contents};

    const EXAMPLE_INPUT: &str = "
    0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45
    ";

    #[test]
    fn test_parse_contents() {
        let numbers = parse_contents(EXAMPLE_INPUT);
        assert_eq!(numbers.len(), 3);
        assert_eq!(numbers.first().unwrap(), &[0, 3, 6, 9, 12, 15]);
    }

    #[test]
    fn test_differences() {
        let numbers = parse_contents(EXAMPLE_INPUT);
        let diffs = differences(&numbers);
        assert_eq!(diffs.len(), 3);
        assert_eq!(diffs.get(0).unwrap(), &vec![[3, 3, 3, 3, 3]]);
        assert_eq!(diffs.get(1).unwrap(), &vec![vec![2, 3, 4, 5, 6], vec![1, 1, 1, 1]]);
    }

    #[test]
    fn test_find_previous_numbers() {
        let numbers = parse_contents(EXAMPLE_INPUT);
        let diffs = differences(&numbers);
        let previous_numbers = find_previous_numbers(&numbers, &diffs);
        assert_eq!(previous_numbers, [-3, 0, 5]);
    }
}
