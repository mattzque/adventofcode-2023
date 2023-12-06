#![warn(clippy::pedantic)]
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq)]
struct Race {
    /// Time available in race in ms
    time: u64,
    /// Record distance traveled in mm
    distance: u64,
}

impl Race {
    fn count_faster_bounds(&self) -> u64 {
        let range = 1..=self.time;
        let find_faster = |duration: &u64| {
            let new_distance = (self.time - duration) * duration;
            new_distance > self.distance
        };

        let lower = range.clone().find(find_faster).unwrap();
        let upper = range.rev().find(find_faster).unwrap();

        upper - lower + 1
    }
}

fn parse_number(line: &str, suffix: &'static str) -> anyhow::Result<u64> {
    if line.contains(suffix) {
        let number = line[suffix.len()..].replace(' ', "");
        let number =
            str::parse::<u64>(&number).map_err(|_| anyhow::anyhow!("Error parsing number"))?;
        Ok(number)
    } else {
        Err(anyhow::anyhow!("Invalid Input!"))
    }
}

fn parse_contents(contents: &str) -> anyhow::Result<Race> {
    let mut lines = contents.trim().lines();
    let time = parse_number(
        lines
            .next()
            .ok_or(anyhow::anyhow!("Invalid Input!"))?
            .trim(),
        "Time:",
    )?;
    let distance = parse_number(
        lines
            .next()
            .ok_or(anyhow::anyhow!("Invalid Input!"))?
            .trim(),
        "Distance:",
    )?;

    Ok(Race { time, distance })
}

fn main() {
    let contents = read_to_string("day6-input.txt").expect("Invalid Input!");
    let race = parse_contents(&contents).expect("Invalid Input!");
    let solution = race.count_faster_bounds();
    println!("Solution: {solution}");
}

#[cfg(test)]
mod tests {
    use crate::{parse_contents, Race};

    const EXAMPLE_INPUT: &str = "
    Time:      7  15   30
    Distance:  9  40  200
    ";

    #[test]
    fn test_parse_contents() {
        let race = parse_contents(EXAMPLE_INPUT).unwrap();
        assert_eq!(
            race,
            Race {
                time: 71530,
                distance: 940_200
            }
        );
    }

    #[test]
    fn test_count_faster_bounds() {
        let race = parse_contents(EXAMPLE_INPUT).unwrap();
        assert_eq!(race.count_faster_bounds(), 71503);
    }
}
