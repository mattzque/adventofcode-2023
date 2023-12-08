#![warn(clippy::pedantic)]
use std::{collections::BTreeMap, fs::read_to_string};

#[derive(Debug, Default)]
struct Tree<'a> {
    instructions: &'a str,
    nodes: BTreeMap<&'a str, (&'a str, &'a str)>,
}

impl<'a> Tree<'a> {
    fn traverse(&self, from: &'a str, to: &'a str) -> Vec<&'a str> {
        let mut path = Vec::new();
        let mut ip = 0;
        let mut q = vec![from];

        while let Some(next) = q.pop() {
            path.push(next);
            if next == to {
                return path;
            }

            if let Some((left, right)) = self.nodes.get(next) {
                let instr = self.instructions.chars().nth(ip).unwrap();
                let child = if instr == 'L' { left } else { right };
                q.push(child);

                ip += 1;
                if ip >= self.instructions.len() {
                    ip = 0;
                }
            }
        }

        path
    }
}

fn parse_contents(contents: &str) -> Tree {
    let mut lines = contents.trim().lines();
    Tree {
        instructions: lines.next().expect("Invalid Instruction Input!"),
        nodes: lines
            .filter_map(|line| {
                if let Some((label, children)) = line.trim().split_once(" = ") {
                    let (left_child, right_child) = children[1..children.len() - 1]
                        .split_once(", ")
                        .expect("Invalid left/right children!");
                    Some((label, (left_child, right_child)))
                } else {
                    None
                }
            })
            .collect::<BTreeMap<&str, (&str, &str)>>(),
    }
}

fn main() {
    let contents = read_to_string("day8-input.txt").expect("Invalid Input!");
    let tree = parse_contents(&contents);
    let path = tree.traverse("AAA", "ZZZ");
    println!("Path: {:?}", path.len() - 1);
}

#[cfg(test)]
mod tests {
    use crate::parse_contents;

    const EXAMPLE_INPUT_1: &str = "
    RL

    AAA = (BBB, CCC)
    BBB = (DDD, EEE)
    CCC = (ZZZ, GGG)
    DDD = (DDD, DDD)
    EEE = (EEE, EEE)
    GGG = (GGG, GGG)
    ZZZ = (ZZZ, ZZZ)
    ";

    const EXAMPLE_INPUT_2: &str = "
    LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)
    ";

    #[test]
    fn test_parse_contents() {
        let tree = parse_contents(EXAMPLE_INPUT_1);
        assert_eq!(tree.instructions, "RL");
        assert_eq!(tree.nodes.len(), 7);

        let tree = parse_contents(EXAMPLE_INPUT_2);
        assert_eq!(tree.instructions, "LLR");
        assert_eq!(tree.nodes.len(), 3);
    }

    #[test]
    fn test_traverse() {
        let tree = parse_contents(EXAMPLE_INPUT_1);
        let path = tree.traverse("AAA", "ZZZ");
        assert_eq!(path, ["AAA", "CCC", "ZZZ"]);

        let tree = parse_contents(EXAMPLE_INPUT_2);
        let path = tree.traverse("AAA", "ZZZ");
        assert_eq!(path, ["AAA", "BBB", "AAA", "BBB", "AAA", "BBB", "ZZZ"]);
    }
}
