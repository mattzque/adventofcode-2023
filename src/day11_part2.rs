#![warn(clippy::pedantic)]
use std::{
    cmp::Ordering,
    collections::HashSet,
    fmt::{self},
    fs::read_to_string,
};

type Point = (i64, i64);

fn distance(a: Point, b: Point) -> i64 {
    (b.0 - a.0).abs() + (b.1 - a.1).abs()
}

const CELL_EMPTY: char = '.';
const CELL_GALAXY: char = '#';

#[derive(PartialEq)]
struct Grid {
    pub rows: Vec<Vec<char>>,
}

impl Grid {
    fn from_contents(contents: &str) -> Self {
        Self {
            rows: contents
                .trim()
                .lines()
                .map(|row| row.trim().chars().collect())
                .collect(),
        }
    }

    fn is_row_empty(&self, y: usize) -> bool {
        self.rows[y].iter().all(|ch| *ch == CELL_EMPTY)
    }

    fn is_col_empty(&self, x: usize) -> bool {
        self.rows.iter().all(|row| row[x] == CELL_EMPTY)
    }

    fn galaxies(&self, expansion: usize) -> Vec<Point> {
        let mut galaxies = Vec::new();
        let mut gx = 0;
        let mut gy = 0;
        self.rows.iter().enumerate().for_each(|(y, row)| {
            if self.is_row_empty(y) {
                gy += expansion;
            } else {
                gx = 0;
                row.iter().enumerate().for_each(|(x, ch)| {
                    if self.is_col_empty(x) {
                        gx += expansion;
                    } else if *ch == CELL_GALAXY {
                        galaxies.push((i64::try_from(gx).unwrap(), i64::try_from(gy).unwrap()));
                    }

                    gx += 1;
                });
            }
            gy += 1;
        });
        galaxies
    }

    fn all_pairs_of_galaxies(&self, expansion: usize) -> Vec<(Point, Point)> {
        // collect all the coordinates of galaxies
        let galaxies = self.galaxies(expansion);

        // combine them together into all two possible pairs
        // then collect into a set to deduplicate and finally return as a list
        galaxies
            .iter()
            .flat_map(|a| {
                galaxies.iter().filter_map(move |b| match a.cmp(b) {
                    Ordering::Greater => Some((*a, *b)),
                    Ordering::Less => Some((*b, *a)),
                    Ordering::Equal => None,
                })
            })
            .collect::<HashSet<_>>()
            .iter()
            .copied()
            .collect::<Vec<_>>()
    }

    fn distances_between_galaxies(&self, expansion: usize) -> Vec<i64> {
        self.all_pairs_of_galaxies(expansion)
            .into_iter()
            .map(|(a, b)| distance(a, b))
            .collect()
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.rows {
            for cell in row {
                write!(f, "{cell}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let contents = read_to_string("day11-input.txt").expect("Invalid Input!");
    let grid = Grid::from_contents(&contents);
    println!(
        "Solution: {}",
        grid.distances_between_galaxies(1_000_000 - 1).iter().sum::<i64>()
    );
}

#[cfg(test)]
mod tests {
    use crate::Grid;

    const EXAMPLE_INPUT: &str = "
    ...#......
    .......#..
    #.........
    ..........
    ......#...
    .#........
    .........#
    ..........
    .......#..
    #...#.....
    ";

    const EXAMPLE_INPUT_EXPANDED: &str = "
    ....#........
    .........#...
    #............
    .............
    .............
    ........#....
    .#...........
    ............#
    .............
    .............
    .........#...
    #....#.......
    ";

    #[test]
    fn test_from_contents() {
        let grid = Grid::from_contents(EXAMPLE_INPUT);
        assert_eq!(
            grid.galaxies(1),
            Grid::from_contents(EXAMPLE_INPUT_EXPANDED).galaxies(0)
        );
        assert_eq!(grid.galaxies(1).len(), 9);
    }

    #[test]
    fn test_all_pairs_of_galaxies() {
        let grid = Grid::from_contents(EXAMPLE_INPUT);
        assert_eq!(grid.all_pairs_of_galaxies(10).len(), 36);
    }

    #[test]
    fn test_distances_between_galaxies() {
        let grid = Grid::from_contents(EXAMPLE_INPUT);
        assert_eq!(grid.distances_between_galaxies(1).iter().sum::<i64>(), 374);
    }
}
