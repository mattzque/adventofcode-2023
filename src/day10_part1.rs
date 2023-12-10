use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
};

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

type MapCoord = (isize, isize);

fn product_iter(width: usize, height: usize) -> impl Iterator<Item = MapCoord> {
    (0..width).flat_map(move |x| (0..height).map(move |y| (x as isize, y as isize)))
}

impl Map {
    const NORTH: MapCoord = (0, -1);
    const SOUTH: MapCoord = (0, 1);
    const WEST: MapCoord = (-1, 0);
    const EAST: MapCoord = (1, 0);
    const NEIGHBORS: &'static [MapCoord] = &[Self::EAST, Self::WEST, Self::NORTH, Self::SOUTH];
    const PIPES: &'static [(char, (MapCoord, MapCoord))] = &[
        // connecting north and south
        ('|', (Self::NORTH, Self::SOUTH)),
        // east and west
        ('-', (Self::EAST, Self::WEST)),
        // north and east
        ('L', (Self::NORTH, Self::EAST)),
        // north and west
        ('J', (Self::NORTH, Self::WEST)),
        //  south and west
        ('7', (Self::SOUTH, Self::WEST)),
        // south and east
        ('F', (Self::SOUTH, Self::EAST)),
    ];

    fn from_contents(contents: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let tiles = contents
            .trim()
            .lines()
            .map(|line| {
                height += 1;
                let row = line.trim().chars().collect::<Vec<_>>();
                width = row.len();
                row
            })
            .collect::<Vec<_>>();
        Self {
            tiles,
            width,
            height,
        }
    }

    fn get(&self, tile: MapCoord) -> Option<char> {
        let (x, y) = tile;

        let y = usize::try_from(y).ok()?; // .expect("invalid x-coordinate")
        let x = usize::try_from(x).ok()?; // .expect("invalid y-coordinate")

        self.tiles.get(y).and_then(|row| row.get(x).copied())
    }

    fn find_char(&self, needle: char) -> Option<(isize, isize)> {
        product_iter(self.width, self.height).find(|(x, y)| self.get((*x, *y)).unwrap() == needle)
    }

    /// Returns a list of tile coordinates which connect to the given tile.
    fn find_connected(&self, tile: (isize, isize)) -> Vec<MapCoord> {
        let mut connected = Vec::new();
        let (x, y) = tile;
        for (ox, oy) in Self::NEIGHBORS.iter() {
            // inverse offset:
            let offset = (
                if *ox == 1 { -1 } else { isize::from(*ox == -1) },
                if *oy == 1 { -1 } else { isize::from(*oy == -1) },
            );
            let tile = (x + ox, y + oy);
            let Some(ch) = self.get(tile) else {
                continue;
            };

            // check if this neighbor tile points into the given tile
            if let Some((_, (a, b))) = Self::PIPES.iter().find(|(tile, _)| *tile == ch) {
                if *a == offset || *b == offset {
                    connected.push(tile);
                }
            }
        }
        connected
    }

    // find max steps
    fn traverse(&self) -> usize {
        let start = self.find_char('S').expect("No start tile found!");

        // queue with all neighbors connecting to start (coordinate, number-of-steps)
        let mut q = self
            .find_connected(start)
            .iter()
            .map(|coord| (*coord, 1))
            .collect::<VecDeque<_>>();
        let mut visited = HashSet::new();
        let mut max_steps = 0;

        while let Some((current, steps)) = q.pop_back() {
            if steps > max_steps {
                max_steps = steps;
            }
            visited.insert(current);
            let Some(ch) = self.get(current) else {
                continue;
            };
            if let Some((_, (a, b))) = Self::PIPES.iter().find(|(pipe, _)| *pipe == ch) {
                // a and b are offsets indicating the pipe direction
                // get the absolute coordinates:
                let a = (current.0 + a.0, current.1 + a.1);
                let b = (current.0 + b.0, current.1 + b.1);

                if !visited.contains(&a) {
                    q.push_front((a, steps + 1));
                }
                if !visited.contains(&b) {
                    q.push_front((b, steps + 1));
                }
            }
        }

        max_steps
    }
}

fn main() {
    let contents = read_to_string("day10-input.txt").expect("Invalid Input!");
    let map = Map::from_contents(&contents);
    let max_path = map.traverse();
    println!("Solution: {max_path}");
}

#[cfg(test)]
mod tests {
    use crate::Map;

    const EXAMPLE_INPUT_1: &str = "
    .....
    |S-7.
    .|.|.
    .L-J.
    .....
    ";
    const EXAMPLE_INPUT_2: &str = "
    -L|F7
    7S-7|
    L|7||
    -L-J|
    L|-JF
    ";
    const EXAMPLE_INPUT_3: &str = "
    ..F7.
    .FJ|.
    SJ.L7
    |F--J
    LJ...
    ";

    #[test]
    fn test_from_contents() {
        let map = Map::from_contents(EXAMPLE_INPUT_1);
        assert_eq!(map.traverse(), 4);
        let map = Map::from_contents(EXAMPLE_INPUT_2);
        assert_eq!(map.traverse(), 4);
        let map = Map::from_contents(EXAMPLE_INPUT_3);
        assert_eq!(map.traverse(), 8);
    }
}
