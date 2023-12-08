#![warn(clippy::pedantic)]
use std::{collections::BTreeMap, fs::read_to_string};

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn lcm_list(numbers: &[u64]) -> u64 {
    numbers.iter().fold(1, |l, &n| lcm(l, n))
}

#[derive(Debug, Default)]
struct Tree<'a> {
    instructions: &'a str,
    nodes: BTreeMap<&'a str, (&'a str, &'a str)>,
}

impl<'a> Tree<'a> {
    fn ends_with(&self, suffix: char) -> Vec<&'a str> {
        self.nodes
            .keys()
            .filter(|name| name.ends_with(suffix))
            .copied()
            .collect::<Vec<&'a str>>()
    }

    fn traverse(&self, from: char, to: char) -> u64 {
        let mut steps = Vec::new();
        for start_node in self.ends_with(from) {
            let mut step = 0;
            let mut ip = 0;
            let mut q = vec![start_node];

            while let Some(next) = q.pop() {
                step += 1;
                if next.ends_with(to) {
                    break;
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
            steps.push(step - 1);
        }

        lcm_list(&steps)
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
    let path_len = tree.traverse('A', 'Z');
    println!("Path: {:?}", path_len);
}

#[cfg(test)]
mod tests {
    use crate::parse_contents;

    const EXAMPLE_INPUT: &str = "
    LR

    11A = (11B, XXX)
    11B = (XXX, 11Z)
    11Z = (11B, XXX)
    22A = (22B, XXX)
    22B = (22C, 22C)
    22C = (22Z, 22Z)
    22Z = (22B, 22B)
    XXX = (XXX, XXX)
    ";

    #[test]
    fn test_parse_contents() {
        let tree = parse_contents(EXAMPLE_INPUT);
        assert_eq!(tree.instructions, "LR");
        assert_eq!(tree.nodes.len(), 8);
    }

    #[test]
    fn test_traverse() {
        let tree = parse_contents(EXAMPLE_INPUT);
        let path_len = tree.traverse('A', 'Z');
        assert_eq!(path_len, 6);
    }
}
