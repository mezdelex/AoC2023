use std::collections::HashMap;

use rayon::iter::{ParallelBridge, ParallelIterator};
use regex::Regex;

use crate::handler::Handler;

#[derive(Debug)]
pub struct Day08 {
    pub node_mapper: HashMap<char, u32>,
}

impl Day08 {
    fn new() -> Self {
        Self {
            node_mapper: HashMap::from([('L', 0), ('R', 1)]),
        }
    }

    fn process_input() -> (Vec<char>, HashMap<String, (String, String)>) {
        let input = Handler::new("./src/day08/input.txt")
            .handle_input()
            .unwrap();
        let mut lines = input.lines();
        let instructions: Vec<char> = lines.next().unwrap().chars().collect();
        let regex = Regex::new(r"\W+").unwrap();
        let node_map: HashMap<String, (String, String)> = lines
            .skip(1)
            .par_bridge()
            .map(|line| {
                let vec_values: Vec<&str> = regex.split(line).collect();

                (
                    vec_values[0].to_string(),
                    (vec_values[1].to_string(), vec_values[2].to_string()),
                )
            })
            .collect();

        (instructions, node_map)
    }

    // Recursion yields stack overflow; Works with examples though. Left for reference.
    #[allow(dead_code)]
    fn check_nodes_recursive(
        node_mapper: HashMap<char, u32>,
        instructions: &Vec<char>,
        node_map: &HashMap<String, (String, String)>,
        instruction_idx: &mut usize,
        current_node: &str,
    ) -> u32 {
        if current_node.ends_with('Z') {
            return 0;
        }

        if *instruction_idx > instructions.len() - 1 {
            *instruction_idx = 0
        }

        match node_mapper.get(&instructions[*instruction_idx]) {
            Some(&0) => {
                1 + Self::check_nodes_recursive(
                    node_mapper,
                    instructions,
                    node_map,
                    &mut (*instruction_idx + 1),
                    &node_map.get(current_node).unwrap().0,
                )
            }
            Some(&1) => {
                1 + Self::check_nodes_recursive(
                    node_mapper,
                    instructions,
                    node_map,
                    &mut (*instruction_idx + 1),
                    &node_map.get(current_node).unwrap().1,
                )
            }
            _ => panic!("Shouldn't reach this case."),
        }
    }

    fn check_nodes_iterative(
        node_mapper: HashMap<char, u32>,
        instructions: &Vec<char>,
        node_map: &HashMap<String, (String, String)>,
        instruction_idx: &mut usize,
        current_node: &mut String,
    ) -> usize {
        let mut sum = 1;
        loop {
            match node_mapper.get(&instructions[*instruction_idx]) {
                Some(&0) => *current_node = node_map.get(current_node).unwrap().0.to_string(),
                Some(&1) => *current_node = node_map.get(current_node).unwrap().1.to_string(),
                _ => panic!("Fuck borrowing."),
            }
            if current_node.ends_with('Z') {
                return sum;
            }

            *instruction_idx += 1;
            if *instruction_idx > instructions.len() - 1 {
                *instruction_idx = 0;
            }

            sum += 1;
        }
    }

    fn greatest_common_divisor(a: usize, b: usize) -> usize {
        if b == 0 {
            return a;
        }

        Self::greatest_common_divisor(b, a % b)
    }

    fn least_common_multiple(a: usize, b: usize) -> usize {
        (a * b) / Self::greatest_common_divisor(a, b)
    }

    pub fn part1() -> usize {
        let input = Self::process_input();
        let day_handler = Self::new();

        Self::check_nodes_iterative(
            day_handler.node_mapper,
            &input.0,
            &input.1,
            &mut 0,
            &mut "AAA".to_string(),
        )
    }

    pub fn part2() -> usize {
        let input = Self::process_input();
        let day_handler = Self::new();

        input
            .1
            .keys()
            .filter(|key| key.ends_with('A'))
            .map(|node| {
                Self::check_nodes_iterative(
                    day_handler.node_mapper.clone(),
                    &input.0,
                    &input.1,
                    &mut 0,
                    &mut node.to_string(),
                )
            })
            .reduce(|acc, next| Self::least_common_multiple(acc, next))
            .unwrap()
    }
}

#[test]
fn test1() {
    let expected = 21409;
    assert_eq!(expected, Day08::part1());
}

#[test]
fn test2() {
    let expected = 21165830176709;
    assert_eq!(expected, Day08::part2());
}
