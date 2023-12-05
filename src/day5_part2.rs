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
    fn lookup_range(&self, source: Range<u64>) -> Vec<Range<u64>> {
        let mut debug_source_ranges = Vec::new();
        let mut ranges = Vec::new();

        let mut start = source.start;
        let end = source.end;

        let mut lookup = self.ranges.clone();
        lookup.sort_by(|(i, _), (j, _)| j.start.partial_cmp(&i.start).unwrap());

        while let Some((source, destination)) = lookup.pop() {
            let i = start.max(source.start);
            let j = end.min(source.end);

            if i < j {
                // insert identity range for everything before the matching range
                if i > start {
                    debug_source_ranges.push(start..i);
                    ranges.push(start..i);
                }

                // map within destination

                ranges.push(
                    (destination.start
                        + (i64::try_from(source.start)
                            .expect("error casting source.start to i64!")
                            - i64::try_from(i).expect("error casting i to i64!"))
                        .unsigned_abs())
                        ..(destination.start
                            + (i64::try_from(j).expect("error casting j to i64!")
                                - i64::try_from(source.start)
                                    .expect("error casting source.start to i64!"))
                            .unsigned_abs()),
                );
                debug_source_ranges.push(i..j);

                // reset start to end of current range
                start = j;
            }
        }

        if ranges.is_empty() || start < end {
            debug_source_ranges.push(start..end);
            ranges.push(start..end);
        }

        // sanity checking
        let length = source.end - source.start;
        assert_eq!(
            debug_source_ranges
                .iter()
                .map(|range| range.end - range.start)
                .sum::<u64>(),
            length,
            "Invalid source mapping range length!"
        );
        assert_eq!(
            ranges
                .iter()
                .map(|range| range.end - range.start)
                .sum::<u64>(),
            length,
            "Invalid mapped ranges total length!"
        );

        ranges
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<Range<u64>>,
    mappings: HashMap<(Category, Category), Mapping>,
}

impl Almanac {
    /// Lookup the corresponding value in the source category with a value range,
    /// mapped to the destination value using the almanac.
    fn lookup_range(
        &self,
        source: Category,
        destination: Category,
        range: Range<u64>,
    ) -> anyhow::Result<Vec<Range<u64>>> {
        let mut current = vec![range];
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
            current = current
                .iter()
                .flat_map(|range| mapping.lookup_range(range.clone()))
                .collect();

            if *new_destination == destination {
                return Ok(current);
            }

            // destination becomes new source
            source = *new_destination;
        }
    }
}

fn create_pairs_from_iter<T>(
    mut iterator: impl Iterator<Item = T>,
) -> impl Iterator<Item = (T, T)> {
    std::iter::from_fn(move || {
        if let (Some(first), Some(second)) = (iterator.next(), iterator.next()) {
            Some((first, second))
        } else {
            None
        }
    })
}

fn parse_contents(contents: &str) -> anyhow::Result<Almanac> {
    let contents = contents.trim();
    let mut lines = contents.lines();
    let first = lines.next().ok_or(anyhow::anyhow!("Invalid Input!"))?;
    if first.find("seeds: ") != Some(0) {
        return Err(anyhow::anyhow!("Invalid Input!"));
    }
    let seeds = create_pairs_from_iter(
        first["seeds: ".len()..]
            .split(' ')
            .flat_map(str::parse::<u64>),
    )
    .map(|(start, length)| start..start + length)
    .collect::<Vec<Range<u64>>>();

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
        .flat_map(|range| {
            almanac.lookup_range(Category::Seed, Category::Location, range.clone()).expect("Invalid Seed Lookup!")
        })
        .collect::<Vec<Range<u64>>>();

    let min_location = locations.iter().min_by_key(|range| range.start).unwrap();

    // println!("Locations: {locations:#?}");
    println!("Smallest Range Start: {:#?}", min_location.start);
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
    fn test_mapping_lookup_range() {
        let mapping = Mapping {
            ranges: vec![(98..98 + 2, 50..50 + 2), (50..50 + 48, 52..52 + 48)],
        };

        let lookup_with_range = |source: u64| {
            #[allow(clippy::range_plus_one)]
            let ranges = mapping.lookup_range(source..source + 1);
            assert_eq!(ranges.len(), 1);
            let values = ranges.first().cloned().unwrap().collect::<Vec<_>>();
            assert_eq!(values.len(), 1);
            *values.first().unwrap()
        };

        assert_eq!(lookup_with_range(79), 81);

        assert_eq!(lookup_with_range(1), 1);

        assert_eq!(lookup_with_range(79), 81);
        assert_eq!(lookup_with_range(14), 14);
        assert_eq!(lookup_with_range(55), 57);
        assert_eq!(lookup_with_range(13), 13);

        assert_eq!(lookup_with_range(96), 98);
        assert_eq!(lookup_with_range(97), 99);
        assert_eq!(lookup_with_range(98), 50);
        assert_eq!(lookup_with_range(99), 51);
        assert_eq!(lookup_with_range(100), 100); // identity if no range!

        let mapping = Mapping {
            ranges: vec![(5..10, 20..25)],
        };
        mapping.lookup_range(1..30);
    }

    #[test]
    fn test_parse_contents() {
        let almanac = parse_contents(EXAMPLE_INPUT).unwrap();
        assert_eq!(almanac.seeds, vec![79..79 + 14, 55..55 + 13]);
        assert_eq!(almanac.mappings.len(), 7);
    }

    #[test]
    fn test_almanac_lookup() {
        let almanac = parse_contents(EXAMPLE_INPUT).unwrap();
        let lookup_with_value = |source: Category, destination: Category, value: u64| {
            #[allow(clippy::range_plus_one)]
            let range = value..(value + 1);
            let ranges = almanac.lookup_range(source, destination, range).unwrap();
            assert_eq!(ranges.len(), 1);
            let values = ranges.first().cloned().unwrap().collect::<Vec<_>>();
            assert_eq!(values.len(), 1);
            *values.first().unwrap()
        };

        assert_eq!(lookup_with_value(Category::Seed, Category::Soil, 79), 81);
        assert_eq!(
            lookup_with_value(Category::Seed, Category::Fertilizer, 79),
            81
        );
        assert_eq!(lookup_with_value(Category::Seed, Category::Water, 79), 81);
        assert_eq!(lookup_with_value(Category::Seed, Category::Light, 79), 74);
        assert_eq!(
            lookup_with_value(Category::Seed, Category::Temperature, 79),
            78
        );
        assert_eq!(
            lookup_with_value(Category::Seed, Category::Humidity, 79),
            78
        );
        assert_eq!(
            lookup_with_value(Category::Seed, Category::Location, 79),
            82
        );
    }
}
