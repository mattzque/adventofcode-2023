#![warn(clippy::pedantic)]
use std::fs::read_to_string;

// springs, groups
fn parse_content(content: &str) -> Vec<(String, Vec<u64>)> {
    content
        .trim()
        .lines()
        .map(|line| {
            let (springs, groups) = line.trim().split_once(' ').unwrap();
            let springs = springs.to_string();
            let groups = groups
                .split(',')
                .map(str::parse::<u64>)
                .map(Result::unwrap)
                .collect::<Vec<u64>>();
            (springs, groups)
        })
        .collect()
}

fn valid(chars: &[char], numbers: &[u64]) -> bool {
    let mut current = 0;
    let mut acc = Vec::new();
    for &ch in chars {
        if ch == '.' {
            if current > 0 {
                acc.push(current);
            }
            current = 0;
        } else if ch == '#' {
            current += 1;
        }
    }
    if current > 0 {
        acc.push(current);
    }
    acc == numbers
}

fn combinations(springs: &str, groups: &[u64]) -> u64 {
    let indices = springs
        .char_indices()
        .filter_map(|(i, ch)| if ch == '?' { Some(i) } else { None })
        .collect::<Vec<usize>>();
    let n_total = 1 << indices.len(); // 2^n
    let mut n_valid = 0;

    for n in 0..n_total {
        let mut chars = springs.chars().collect::<Vec<char>>();
        for (i, &index) in indices.iter().enumerate() {
            if n & (1 << i) == 0 {
                chars[index] = '.';
            } else {
                chars[index] = '#';
            }
        }

        if valid(&chars, groups) {
            n_valid += 1;
        }
    }

    n_valid
}

fn main() {
    let contents = read_to_string("day12-input.txt").expect("Invalid Input!");
    let rows = parse_content(&contents);
    let n_valids = rows
        .iter()
        .map(|(chars, numbers)| combinations(chars, numbers))
        .sum::<u64>();
    println!("Solution: {n_valids}");
}

#[cfg(test)]
mod tests {
    use crate::{combinations, parse_content};

    const EXAMPLE_INPUT: &str = "
    ???.### 1,1,3
    .??..??...?##. 1,1,3
    ?#?#?#?#?#?#?#? 1,3,1,6
    ????.#...#... 4,1,1
    ????.######..#####. 1,6,5
    ?###???????? 3,2,1
    ";

    #[test]
    fn test_from_contents() {
        let rows = parse_content(EXAMPLE_INPUT);
        let valids = rows
            .iter()
            .map(|(chars, numbers)| combinations(chars, numbers))
            .collect::<Vec<u64>>();
        assert_eq!(valids, &[1, 4, 1, 1, 4, 10]);
        assert_eq!(valids.iter().sum::<u64>(), 21);
    }
}
