#![warn(clippy::pedantic)]
use std::{collections::HashMap, fs::read_to_string};

// springs, groups
fn parse_content(content: &str, n_repititions: usize) -> Vec<(String, Vec<u64>)> {
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

            let mut springs_ = vec![springs.clone()];
            let mut groups_ = groups.clone();
            if n_repititions > 0 {
                for _ in 0..n_repititions {
                    springs_.push(springs.clone());
                    groups_.extend_from_slice(&groups.clone());
                }
            }

            (springs_.join("?"), groups_)
        })
        .collect()
}

fn combinations(springs: &str, groups: &[u64]) -> u64 {
    fn fun(
        springs: &str,
        remaining_groups: &[u64],
        current_group: u64,
        cache: &mut HashMap<String, u64>,
    ) -> u64 {
        let key = format!("{springs} {remaining_groups:?} {current_group:?}");
        if let Some(&count) = cache.get(&key) {
            return count;
        }

        if springs.is_empty() {
            if (current_group == 0 && remaining_groups.is_empty())
                || (remaining_groups.len() == 1 && current_group == remaining_groups[0])
            {
                cache.insert(key, 1);
                return 1;
            }
            cache.insert(key, 0);
            return 0;
        }

        if (!remaining_groups.is_empty() && current_group > remaining_groups[0])
            || (remaining_groups.is_empty() && current_group > 0)
        {
            cache.insert(key, 0);
            return 0;
        }

        let ch: char = springs.chars().next().unwrap();
        let mut n: u64 = 0;

        if ch == '#' || ch == '?' {
            n += fun(&springs[1..], remaining_groups, current_group + 1, cache);
        }

        if ch == '.' || ch == '?' {
            if !remaining_groups.is_empty() && current_group == remaining_groups[0] {
                n += fun(&springs[1..], &remaining_groups[1..], 0, cache);
            } else if current_group == 0 {
                n += fun(&springs[1..], remaining_groups, 0, cache);
            }
        }

        cache.insert(key, n);
        n
    }

    let mut cache = HashMap::new();
    fun(springs, groups, 0, &mut cache)
}

fn main() {
    let contents = read_to_string("day12-input.txt").expect("Invalid Input!");
    let rows = parse_content(&contents, 4);
        let valids = rows
            .iter()
            .map(|(springs, groups)| combinations(springs, groups))
            .sum::<u64>();
        println!("Solution: {valids}");
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
        let lines = parse_content(EXAMPLE_INPUT, 0);
        let valids = lines
            .iter()
            .map(|(springs, groups)| combinations(springs, groups))
            .collect::<Vec<u64>>();
        assert_eq!(valids, &[1, 4, 1, 1, 4, 10]);
        assert_eq!(valids.iter().sum::<u64>(), 21);

        let lines = parse_content(EXAMPLE_INPUT, 4);
        let valids = lines
            .iter()
            .map(|(springs, groups)| combinations(springs, groups))
            .collect::<Vec<u64>>();
        assert_eq!(valids, &[1, 16384, 1, 16, 2500, 506_250]);
        assert_eq!(valids.iter().sum::<u64>(), 525_152);
    }
}
