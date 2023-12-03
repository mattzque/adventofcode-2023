use std::fs::read_to_string;

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
            self.data.get(i as usize).and_then(|row| row.get(j as usize).copied()).unwrap_or('.')
        }
    }

    fn has_symbol(&self, i: isize, j: isize) -> bool {
        let char = self.get(i, j);
        !char.is_ascii_digit() && char != '.'
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
    ADJ.iter().map(move |(oi, oj)| (i as isize + oi, j as isize + oj))
}

fn find_part_numbers(schematic: &Schematic) -> Vec<usize> {
    let mut numbers = Vec::new();

    let mut current = String::new();
    let mut is_part = false;
    for (i, j) in product_range(schematic.height, schematic.width) {
        let char = schematic.get(i as isize, j as isize);
        let is_number = char.is_ascii_digit();
        
        if is_number {
            current.push(char);
        } else {
            if !current.is_empty() && is_part {
                numbers.push(str::parse::<usize>(&current).unwrap());
            }
            is_part = false;
            current.clear();
        }

        // determine if it is a number by checking adjacent/diagonal cells
        // if it is marked as a part number and there is a number under the current cell
        if adjacents(i, j).any(|(i, j)| schematic.has_symbol(i, j)) && is_number {
            is_part = true;
        }
    }

    numbers
}

fn main() {
    let contents = read_to_string("day3-input.txt").expect("Invalid Input!");
    let schematic = parse_schematic(&contents).unwrap();
    let part_numbers = find_part_numbers(&schematic);

    println!("Sum of Part Numbers: {:?}", part_numbers.iter().sum::<usize>());
}


#[cfg(test)]
mod test {
    use crate::{parse_schematic, find_part_numbers};

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
    fn test_find_part_numbers() {
        let schematic = parse_schematic(TEST_SCHEMATIC).unwrap();
        let part_numbers = find_part_numbers(&schematic);
        assert_eq!(part_numbers, &[467, 35, 633, 617, 592, 755, 664, 598]);
    }
}