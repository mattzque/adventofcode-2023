// #![warn(clippy::pedantic)]
use core::fmt;
use std::{
    collections::{HashSet, VecDeque},
    fmt::{Debug, Formatter},
    fs::read_to_string,
};

type GridCoord = (isize, isize);

// cartesian product of two ranges from 0 to width and height
fn product_iter(width: usize, height: usize) -> impl Iterator<Item = GridCoord> {
    (0..width).flat_map(move |x| (0..height).map(move |y| (x as isize, y as isize)))
}

const NORTH: GridCoord = (0, -1);
const SOUTH: GridCoord = (0, 1);
const WEST: GridCoord = (-1, 0);
const EAST: GridCoord = (1, 0);
const NEIGHBORS: &[GridCoord] = &[EAST, WEST, NORTH, SOUTH];
const PIPES: &[(char, (GridCoord, GridCoord))] = &[
    // connecting north and south
    ('|', (NORTH, SOUTH)),
    // east and west
    ('-', (EAST, WEST)),
    // north and east
    ('L', (NORTH, EAST)),
    // north and west
    ('J', (NORTH, WEST)),
    //  south and west
    ('7', (SOUTH, WEST)),
    // south and east
    ('F', (SOUTH, EAST)),
];

struct Grid {
    rows: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn from_contents(contents: &str) -> Self {
        let rows: Vec<Vec<char>> = contents
            .trim()
            .lines()
            .map(|line| line.trim().chars().collect())
            .collect();

        let width = rows[0].len();
        let height = rows.len();

        Self {
            rows,
            width,
            height,
        }
    }

    fn get(&self, x: isize, y: isize) -> Option<char> {
        if x < 0 || y < 0 {
            None
        } else {
            self.rows
                .get(y as usize)
                .and_then(|row| row.get(x as usize).copied())
        }
    }

    fn is_any(&self, x: isize, y: isize, any: &[char]) -> bool {
        if let Some(ch) = self.get(x, y) {
            any.iter().any(|c| *c == ch)
        } else {
            false
        }
    }

    fn set(&mut self, x: isize, y: isize, ch: char) {
        self.rows[y as usize][x as usize] = ch;
    }

    fn set_connecting_cells(&mut self) {
        for (x, y) in product_iter(self.width, self.height) {
            if self.get(x, y) == Some(' ') {
                if self.is_any(x, y - 1, &['S', '|', '7', 'F'])
                    && self.is_any(x, y + 1, &['S', '|', 'L', 'J'])
                {
                    self.set(x, y, '|');
                }
                if self.is_any(x - 1, y, &['S', '-', 'L', 'F'])
                    && self.is_any(x + 1, y, &['S', '-', 'J', '7'])
                {
                    self.set(x, y, '-');
                }
            }
        }
    }

    fn find_cell(&self, ch: char) -> Option<(isize, isize)> {
        for (x, y) in product_iter(self.width, self.height) {
            if self.get(x, y) == Some(ch) {
                return Some((x, y));
            }
        }
        None
    }

    fn count_cells(&self, ch: char) -> usize {
        let mut num = 0;
        for (x, y) in product_iter(self.width, self.height) {
            if self.get(x, y) == Some(ch) {
                num += 1;
            }
        }
        num
    }

    fn flood_fill_cell(&mut self, x: isize, y: isize, source: char, replace: char) -> bool {
        let mut q = vec![(x, y)];
        let mut touch_border = false;

        while let Some((x, y)) = q.pop() {
            if let Some(ch) = self.get(x, y) {
                if ch == ' ' || ch == source {
                    if x == 0
                        || x == (self.width - 1) as isize
                        || y == 0
                        || y == (self.height - 1) as isize
                    {
                        touch_border = true;
                    }

                    self.set(x, y, replace);
                    q.extend_from_slice(&[(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]);
                }
            }
        }

        touch_border
    }

    fn flood_fill_cells(&mut self) {
        while let Some((x, y)) = self.find_cell('.') {
            // first flood fill just to get the touch_border state:
            if self.flood_fill_cell(x, y, '.', 'I') {
                // if it is the border, repeat the process but replace it with O
                // perhaps its faster to keep track of all the cells and in the end
                // just set all of them to O
                self.flood_fill_cell(x, y, 'I', 'O');
            }
        }
    }

    fn with_inbetween_cells(&self) -> Self {
        let w = self.width * 2;
        let h = self.height * 2;
        let mut rows = vec![vec![' '; w]; h];

        for (x, y) in product_iter(w, h) {
            if (x % 2) == 0 && (y % 2) == 0 {
                let gx = (x as f32 / 2.0).floor() as isize;
                let gy = (y as f32 / 2.0).floor() as isize;
                if let Some(ch) = self.get(gx, gy) {
                    rows[y as usize][x as usize] = ch;
                }
            }
        }

        Self {
            rows,
            width: w,
            height: h,
        }
    }

    // the inverse -> shrink back to half size
    fn shrink_grid(&self) -> Self {
        let w = self.width / 2;
        let h = self.height / 2;
        let mut rows = vec![vec![' '; w]; h];
        for (x, y) in product_iter(w, h) {
            // rows[y as usize][x as usize] = '#';
            // if (x % 2) == 0 && (y % 2) == 0 {

            let gx = (x as f32 * 2.0).floor() as isize;
            let gy = (y as f32 * 2.0).floor() as isize;
            if let Some(ch) = self.get(gx, gy) {
                rows[y as usize][x as usize] = ch;
            }
            // }
        }

        Self {
            rows,
            width: w,
            height: h,
        }
    }

    /// Returns a list of tile coordinates which connect to the given tile.
    fn find_connected(&self, tile: (isize, isize)) -> Vec<GridCoord> {
        let mut connected = Vec::new();
        let (x, y) = tile;
        for (ox, oy) in NEIGHBORS.iter() {
            // inverse offset:
            let offset = (
                if *ox == 1 { -1 } else { isize::from(*ox == -1) },
                if *oy == 1 { -1 } else { isize::from(*oy == -1) },
            );
            let tile = (x + ox, y + oy);
            let Some(ch) = self.get(tile.0, tile.1) else {
                continue;
            };

            // check if this neighbor tile points into the given tile
            if let Some((_, (a, b))) = PIPES.iter().find(|(tile, _)| *tile == ch) {
                if *a == offset || *b == offset {
                    connected.push(tile);
                }
            }
        }
        connected
    }

    // find max steps
    fn traverse(&self) -> (usize, HashSet<(isize, isize)>) {
        let start = self.find_cell('S').expect("No start tile found!");

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
            let Some(ch) = self.get(current.0, current.1) else {
                continue;
            };
            if let Some((_, (a, b))) = PIPES.iter().find(|(pipe, _)| *pipe == ch) {
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

        (max_steps, visited)
    }

    // any tiles that are not part of the main loop get replaced by ground tile
    fn set_junk_pipes(&mut self) {
        let (_, tiles) = self.traverse();
        for (x, y) in product_iter(self.width, self.height) {
            if !tiles.contains(&(x, y)) {
                if let Some(ch) = self.get(x, y) {
                    if ch != ' ' {
                        self.set(x, y, '.');
                    }
                }
            }
        }
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "(grid: {}x{})", self.width, self.height)?;
        for row in &self.rows {
            for ch in row {
                write!(f, "{ch}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let contents = read_to_string("day10-input.txt").expect("Invalid Input!");
    let grid = Grid::from_contents(&contents);
    // doubles the size of the grid, filling space inbetween cells with space (' ')
    let mut grid = grid.with_inbetween_cells();

    // for each space, figure out connecting pipes on either side (top/bottom, left/right)
    // and fill with | or - pipe: e.g. "- -" becomes "---" etc.
    grid.set_connecting_cells();

    // traverse the main loop, then for every pipe that isn't part of the main loop replace it by '.' (ground tile)
    grid.set_junk_pipes();

    // pick any ground tile ('.') and flood fill it, if it touches the borders replace it by I otherwise O
    grid.flood_fill_cells();

    // shrink the grid back down to half its size (the inverse of with_inbetween_cells)
    let grid = grid.shrink_grid();

    // count all cells that are "I" cells
    let num = grid.count_cells('I');

    // this is the answer: (phew!!! that was hard, I think a graph may have been easier? i dunno)
    println!("num: {}", num);
}

#[cfg(test)]
mod tests {
    use crate::Grid;

    const EXAMPLE_INPUT_5: &str = "
    .F----7F7F7F7F-7....
    .|F--7||||||||FJ....
    .||.FJ||||||||L7....
    FJL7L7LJLJ||LJ.L-7..
    L--J.L7...LJS7F-7L7.
    ....F-J..F7FJ|L7L7L7
    ....L7.F7||L7|.L7L7|
    .....|FJLJ|FJ|F7|.LJ
    ....FJL-7.||.||||...
    L...L---J.LJ.LJLJ...
    ";
    const EXAMPLE_INPUT_6: &str = "
    FF7FSF7F7F7F7F7F---7
    L|LJ||||||||||||F--J
    FL-7LJLJ||||||LJL-77
    F--JF--7||LJLJ7F7FJ-
    L---JF-JLJ.||-FJLJJ7
    |F|F-JF---7F7-L7L|7|
    |FFJF7L7F-JF7|JL---7
    7-L-JL7||F7|L7F-7F7|
    L.L7LFJ|||||FJL7||LJ
    L7JLJL-JLJLJL--JLJ.L
    ";

    #[test]
    fn test_from_contents() {
        let grid = Grid::from_contents(EXAMPLE_INPUT_5);
        let mut grid = grid.with_inbetween_cells();
        grid.set_connecting_cells();
        grid.set_junk_pipes();
        grid.flood_fill_cells();
        let grid = grid.shrink_grid();
        let num = grid.count_cells('I');
        assert_eq!(num, 8);

        let grid = Grid::from_contents(EXAMPLE_INPUT_6);
        let mut grid = grid.with_inbetween_cells();
        grid.set_connecting_cells();
        grid.set_junk_pipes();
        grid.flood_fill_cells();
        let grid = grid.shrink_grid();
        let num = grid.count_cells('I');
        assert_eq!(num, 10);
    }
}
