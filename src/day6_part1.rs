#![warn(clippy::pedantic)]
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq)]
struct Race {
    /// Time available in race in ms
    time: u32,
    /// Record distance traveled in mm
    distance: u32,
}

impl Race {
    /// Find all possible button press durations with their resulting distances
    fn all_button_presses(&self) -> impl Iterator<Item = (u32, u32)> + '_ {
        assert!(self.time != 0, "Race of zero time!");
        (1..=self.time).map(|duration| (duration, (self.time - duration) * duration))
    }

    fn count_faster(&self) -> usize {
        self.all_button_presses()
            .filter(|(_, new_distance)| *new_distance > self.distance)
            .count()
    }
}

fn parse_numbers(line: &str, suffix: &'static str) -> anyhow::Result<Vec<u32>> {
    if line.contains(suffix) {
        Ok(line[suffix.len()..]
            .split(' ')
            .flat_map(str::parse::<u32>)
            .collect())
    } else {
        Err(anyhow::anyhow!("Invalid Input!"))
    }
}

fn parse_contents(contents: &str) -> anyhow::Result<Vec<Race>> {
    let mut lines = contents.trim().lines();
    let times = parse_numbers(
        lines
            .next()
            .ok_or(anyhow::anyhow!("Invalid Input!"))?
            .trim(),
        "Time:",
    )?;
    let distances = parse_numbers(
        lines
            .next()
            .ok_or(anyhow::anyhow!("Invalid Input!"))?
            .trim(),
        "Distance:",
    )?;

    if times.len() == distances.len() {
        Ok(times
            .into_iter()
            .zip(distances)
            .map(|(time, distance)| Race { time, distance })
            .collect())
    } else {
        Err(anyhow::anyhow!("Invalid Input!"))
    }
}

fn main() {
    let contents = read_to_string("day6-input.txt").expect("Invalid Input!");
    let races = parse_contents(&contents).expect("Invalid Input!");
    let solution = races
        .iter()
        .map(Race::count_faster)
        .reduce(|a, b| a * b)
        .unwrap();
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
        let races = parse_contents(EXAMPLE_INPUT);
        assert_eq!(
            races.unwrap(),
            [
                Race {
                    time: 7,
                    distance: 9
                },
                Race {
                    time: 15,
                    distance: 40
                },
                Race {
                    time: 30,
                    distance: 200
                }
            ]
        );
    }

    #[test]
    fn test_all_button_presses() {
        let races = parse_contents(EXAMPLE_INPUT).unwrap();
        let first = races
            .iter()
            .map(|race| race.all_button_presses().collect::<Vec<_>>())
            .next()
            .unwrap();
        assert_eq!(
            first,
            [
                (1, 6,),
                (2, 10,),
                (3, 12,),
                (4, 12,),
                (5, 10,),
                (6, 6,),
                (7, 0,)
            ]
        );
    }

    #[test]
    fn test_count_faster() {
        let races = parse_contents(EXAMPLE_INPUT).unwrap();
        let faster = races.iter().map(Race::count_faster).collect::<Vec<_>>();
        assert_eq!(faster, [4, 8, 9]);
    }
}
