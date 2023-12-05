#![warn(clippy::pedantic)]
use std::{collections::HashMap, fs::read_to_string, ops::Range};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl Category {
    fn from_string(string: &str) -> Option<Self> {
        match string.trim() {
            "seed" => Some(Self::Seed),
            "soil" => Some(Self::Soil),
            "fertilizer" => Some(Self::Fertilizer),
            "water" => Some(Self::Water),
            "light" => Some(Self::Light),
            "temperature" => Some(Self::Temperature),
            "humidity" => Some(Self::Humidity),
            "location" => Some(Self::Location),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Mapping {
    /// Ranges with source to destination
    ranges: Vec<(Range<u64>, Range<u64>)>,
}

impl Mapping {
    fn lookup(&self, source: u64) -> u64 {
        for (i, j) in &self.ranges {
            if i.contains(&source) {
                let index_i = source - i.start;
                let destination = j.start + index_i;
                return destination;
            }
        }
        source
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    mappings: HashMap<(Category, Category), Mapping>,
}

impl Almanac {
    /// Lookup the corresponding value in the source category with a value,
    /// mapped to the destination value using the almanac.
    fn lookup(&self, source: Category, destination: Category, value: u64) -> anyhow::Result<u64> {
        // println!("Lookup: {:?} -> {:?} ({:?})", source, destination, value);
        // println!("Mappings: {:?}", self.mappings);
        let mut current = value;
        let mut source = source;
        loop {
            let ((_, new_destination), mapping) = self
                .mappings
                .iter()
                .find(|((i, _), _)| *i == source)
                .ok_or(anyhow::anyhow!(
                    "Invalid source! No mapping found for source {:?}!",
                    source
                ))?;

            // lookup next value in mapping:
            current = mapping.lookup(current);

            // println!("found mapping for source({:?}) -> {:?} mapping: {:?} (new value: {})", source, new_destination, mapping, current);

            if *new_destination == destination {
                return Ok(current);
            }

            // destination becomes new source
            source = *new_destination;
        }
    }
}

fn parse_contents(contents: &str) -> anyhow::Result<Almanac> {
    let contents = contents.trim();
    let mut lines = contents.lines();
    let first = lines.next().ok_or(anyhow::anyhow!("Invalid Input!"))?;
    if first.find("seeds: ") != Some(0) {
        return Err(anyhow::anyhow!("Invalid Input!"));
    }
    let seeds = first["seeds: ".len()..]
        .split(' ')
        .flat_map(str::parse::<u64>)
        .collect::<Vec<u64>>();

    let mut mappings = HashMap::new();
    let mut current_mapping_key: Option<(Category, Category)> = None;
    let mut current_mappings: Vec<(Range<u64>, Range<u64>)> = Vec::new();

    for line in lines {
        if let Some(rindex) = line.find(" map:") {
            if let Some(key) = current_mapping_key {
                mappings.insert(
                    key,
                    Mapping {
                        ranges: current_mappings.clone(),
                    },
                );
                current_mappings.clear();
            }

            let (source, destination) = line[0..rindex]
                .split_once("-to-")
                .ok_or(anyhow::anyhow!("Invalid Input!"))?;
            let source =
                Category::from_string(source).ok_or(anyhow::anyhow!("Invalid Source Category!"))?;
            let destination = Category::from_string(destination)
                .ok_or(anyhow::anyhow!("Invalid Destination Category!"))?;

            current_mapping_key = Some((source, destination));
        } else if !line.trim().is_empty() {
            let range = line.trim().split(' ').collect::<Vec<&str>>();
            if range.len() != 3 {
                return Err(anyhow::anyhow!("Invalid Range!"));
            }

            let destination_start =
                str::parse::<u64>(range[0]).map_err(|_| anyhow::anyhow!("Invalid Range!"))?;
            let source_start =
                str::parse::<u64>(range[1]).map_err(|_| anyhow::anyhow!("Invalid Range!"))?;
            let range_length =
                str::parse::<u64>(range[2]).map_err(|_| anyhow::anyhow!("Invalid Range!"))?;

            current_mappings.push((
                (source_start)..(source_start + range_length),
                (destination_start)..(destination_start + range_length),
            ));
        }
    }

    if let Some(key) = current_mapping_key {
        mappings.insert(
            key,
            Mapping {
                ranges: current_mappings.clone(),
            },
        );
        current_mappings.clear();
    }

    Ok(Almanac { seeds, mappings })
}

fn main() {
    let contents = read_to_string("day5-input.txt").expect("Invalid Input!");
    let almanac = parse_contents(&contents).expect("Invalid Input!");

    let locations = almanac
        .seeds
        .iter()
        .map(|seed| {
            almanac
                .lookup(Category::Seed, Category::Location, *seed)
                .expect("Invalid Seed Lookup!")
        })
        .collect::<Vec<u64>>();

    let min_location = locations.iter().min();

    println!("Locations: {locations:#?}");
    println!("Smallest Locations: {min_location:#?}");
}

#[cfg(test)]
mod tests {
    use crate::{parse_contents, Category, Mapping};

    const EXAMPLE_INPUT: &str = "
    seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48

    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15

    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4

    water-to-light map:
    88 18 7
    18 25 70

    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13

    temperature-to-humidity map:
    0 69 1
    1 0 69

    humidity-to-location map:
    60 56 37
    56 93 4
    ";

    #[test]
    fn test_mapping_lookup() {
        let mapping = Mapping {
            ranges: vec![(98..98 + 2, 50..50 + 2), (50..50 + 48, 52..52 + 48)],
        };
        assert_eq!(mapping.lookup(1), 1);

        assert_eq!(mapping.lookup(79), 81);
        assert_eq!(mapping.lookup(14), 14);
        assert_eq!(mapping.lookup(55), 57);
        assert_eq!(mapping.lookup(13), 13);

        assert_eq!(mapping.lookup(96), 98);
        assert_eq!(mapping.lookup(97), 99);
        assert_eq!(mapping.lookup(98), 50);
        assert_eq!(mapping.lookup(99), 51);
        assert_eq!(mapping.lookup(100), 100); // identity if no range!
    }

    #[test]
    fn test_parse_contents() {
        let almanac = parse_contents(EXAMPLE_INPUT).unwrap();
        assert_eq!(almanac.seeds, vec![79, 14, 55, 13]);
        assert_eq!(almanac.mappings.len(), 7);
    }

    #[test]
    fn test_almanac_lookup() {
        let almanac = parse_contents(EXAMPLE_INPUT).unwrap();
        assert_eq!(
            almanac.lookup(Category::Seed, Category::Soil, 79).unwrap(),
            81
        );
        assert_eq!(
            almanac
                .lookup(Category::Seed, Category::Fertilizer, 79)
                .unwrap(),
            81
        );
        assert_eq!(
            almanac.lookup(Category::Seed, Category::Water, 79).unwrap(),
            81
        );
        assert_eq!(
            almanac.lookup(Category::Seed, Category::Light, 79).unwrap(),
            74
        );
        assert_eq!(
            almanac
                .lookup(Category::Seed, Category::Temperature, 79)
                .unwrap(),
            78
        );
        assert_eq!(
            almanac
                .lookup(Category::Seed, Category::Humidity, 79)
                .unwrap(),
            78
        );
        assert_eq!(
            almanac
                .lookup(Category::Seed, Category::Location, 79)
                .unwrap(),
            82
        );
    }
}
