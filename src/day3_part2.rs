use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug)]
struct Schematic {
    data: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Schematic {
    fn get(&self, i: isize, j: isize) -> char {
        if i < 0 || j < 0 || i > self.width as isize || j > self.height as isize {
            '.'
        } else {
            self.data
                .get(i as usize)
                .and_then(|row| row.get(j as usize).copied())
                .unwrap_or('.')
        }
    }

    fn has_symbol(&self, i: isize, j: isize, symbol: char) -> bool {
        self.get(i, j) == symbol
    }
}

fn parse_schematic(contents: &str) -> anyhow::Result<Schematic> {
    let data = contents
        .trim()
        .split_terminator('\n')
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let width = data
        .first()
        .ok_or(anyhow::anyhow!("Empty Schematic!"))?
        .len();
    let height = data.len();

    // validation, ensures that all rows have the same width
    if data.iter().all(|row| row.len() != width) {
        Err(anyhow::anyhow!("Invalid Schematic!"))
    } else {
        Ok(Schematic {
            data,
            width,
            height,
        })
    }
}

/// Cartesian product of the ranges 0..a and 0..b
fn product_range(a: usize, b: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..a).flat_map(move |i| (0..b).map(move |j| (i, j)))
}

fn adjacents(i: usize, j: usize) -> impl Iterator<Item = (isize, isize)> {
    const ADJ: &[(isize, isize)] = &[
        // left
        (-1, -1),
        (-1, 0),
        (-1, 1),
        // right
        (1, -1),
        (1, 0),
        (1, 1),
        // top
        (0, -1),
        // bottom
        (0, 1),
    ];
    ADJ.iter()
        .map(move |(oi, oj)| (i as isize + oi, j as isize + oj))
}

fn find_gear_ratios(schematic: &Schematic) -> Vec<usize> {
    const GEAR_SYMBOL: char = '*';

    // find gears with their marker symbol location:
    //  (number, (i, j))
    let mut gears = Vec::new();

    let mut current = String::new();
    let mut is_gear = false;
    let mut gear_mark_coord = (0, 0);
    for (i, j) in product_range(schematic.height, schematic.width) {
        let char = schematic.get(i as isize, j as isize);
        let is_number = char.is_ascii_digit();

        if is_number {
            current.push(char);
        } else {
            if !current.is_empty() && is_gear {
                gears.push((str::parse::<usize>(&current).unwrap(), gear_mark_coord));
            }
            is_gear = false;
            current.clear();
        }

        // determine if it is a number by checking adjacent/diagonal cells
        // if it is marked as a part number and there is a number under the current cell
        if is_number {
            if let Some(mark_coord) = adjacents(i, j).find_map(|(i, j)| {
                if schematic.has_symbol(i, j, GEAR_SYMBOL) {
                    Some((i as usize, j as usize))
                } else {
                    None
                }
            }) {
                is_gear = true;
                gear_mark_coord = mark_coord;
            }
        }
    }

    // group gears by their marker symbol keeping track of their location by index
    //  (number, (i, j)) -> {[(i, j)]: [(index, number)]}
    let mut mapping: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    for (number, gear_mark_coord) in gears {
        // if mapping.entry(gear_mark_coord).or_de()
        let gears = mapping.entry(gear_mark_coord).or_default();
        gears.push(number);
    }

    mapping
        .values()
        .filter(|gears| gears.len() == 2)
        .map(|gears| gears.iter().copied().reduce(|a, b| a * b).unwrap())
        .collect()
}

fn main() {
    let contents = read_to_string("day3-input.txt").expect("Invalid Input!");
    let schematic = parse_schematic(&contents).unwrap();
    let gear_ratios = find_gear_ratios(&schematic);

    println!(
        "Sum of Gear Ratios: {:?}",
        gear_ratios.iter().sum::<usize>()
    );
}

#[cfg(test)]
mod test {
    use crate::{find_gear_ratios, parse_schematic};

    const TEST_SCHEMATIC: &str = "
    467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..
    ";

    #[test]
    fn test_parse_schematic() {
        let schematic = parse_schematic(TEST_SCHEMATIC).unwrap();
        assert_eq!(schematic.width, 10);
        assert_eq!(schematic.height, 10);
        assert_eq!(schematic.data.len(), 10);
    }

    #[test]
    fn test_find_gear_ratios() {
        let schematic = parse_schematic(TEST_SCHEMATIC).unwrap();
        let mut gear_ratios = find_gear_ratios(&schematic);
        gear_ratios.sort();
        assert_eq!(gear_ratios, &[16345, 451490]);
    }
}
