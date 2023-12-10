use std::collections::VecDeque;

use rayon::{collections::vec_deque, iter::ParallelIterator, str::ParallelString};

use crate::handler::Handler;

enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
pub struct Day10;

impl Day10 {
    fn process_input() -> Vec<Vec<char>> {
        Handler::new("./src/day10/input.txt")
            .handle_input()
            .unwrap()
            .par_lines()
            .map(|line| line.chars().collect())
            .collect()
    }

    fn count_tiles(input: &Vec<Vec<char>>) -> usize {
        let mut count = 0;
        let rows = input.len();
        let cols = input[0].len();

        fn dfs(input: &Vec<Vec<char>>, i: i32, j: i32, visited: &mut Vec<Vec<bool>>) {
            if i >= 0
                && i < input.len() as i32
                && j >= 0
                && j < input[0].len() as i32
                && !visited[i as usize][j as usize]
            {
                visited[i as usize][j as usize] = true;

                if input[i as usize][j as usize] == '.' {
                    dfs(input, i + 1, j, visited);
                    dfs(input, i - 1, j, visited);
                    dfs(input, i, j + 1, visited);
                    dfs(input, i, j - 1, visited);
                }
            }
        }

        let mut visited = vec![vec![false; cols]; rows];

        for i in 0..rows as i32 {
            for j in 0..cols as i32 {
                if input[i as usize][j as usize] == '.' && !visited[i as usize][j as usize] {
                    dfs(input, i, j, &mut visited);
                    count += 1;
                }
            }
        }

        count
    }

    fn get_steps_or_tiles(input: &Vec<Vec<char>>, i: usize, j: usize, is_part1: bool) -> usize {
        let mut input_clone = input.clone();
        input_clone[i][j] = '#';

        let mut direction = Direction::East;
        let mut position = (i, j + 1);
        let mut steps = 1;
        loop {
            if input[position.0][position.1].eq(&'S') {
                break;
            }

            (direction, position) = match (direction, input[position.0][position.1]) {
                (Direction::North, '|') => {
                    input_clone[position.0][position.1] = '#';
                    (Direction::North, (position.0 - 1, position.1))
                }
                (Direction::South, '|') => {
                    input_clone[position.0][position.1] = '#';
                    (Direction::South, (position.0 + 1, position.1))
                }
                (Direction::West, '-') => {
                    input_clone[position.0][position.1] = '#';
                    (Direction::West, (position.0, position.1 - 1))
                }
                (Direction::East, '-') => {
                    input_clone[position.0][position.1] = '#';
                    (Direction::East, (position.0, position.1 + 1))
                }
                (Direction::West, 'L') => {
                    input_clone[position.0][position.1] = '#';
                    (Direction::North, (position.0 - 1, position.1))
                }
                (Direction::South, 'L') => {
                    input_clone[position.0][position.1] = '#';
                    (Direction::East, (position.0, position.1 + 1))
                }
                (Direction::East, 'J') => {
                    input_clone[position.0][position.1] = '#';
                    (Direction::North, (position.0 - 1, position.1))
                }
                (Direction::South, 'J') => {
                    input_clone[position.0][position.1] = '#';
                    (Direction::West, (position.0, position.1 - 1))
                }
                (Direction::East, '7') => {
                    input_clone[position.0][position.1] = '#';
                    (Direction::South, (position.0 + 1, position.1))
                }
                (Direction::North, '7') => {
                    input_clone[position.0][position.1] = '#';
                    (Direction::West, (position.0, position.1 - 1))
                }
                (Direction::West, 'F') => {
                    input_clone[position.0][position.1] = '#';
                    (Direction::South, (position.0 + 1, position.1))
                }
                (Direction::North, 'F') => {
                    input_clone[position.0][position.1] = '#';
                    (Direction::East, (position.0, position.1 + 1))
                }
                _ => panic!("Start in another direction and position."),
            };

            steps += 1;
        }

        input_clone.iter().for_each(|line| println!("{:?}", line));

        if is_part1 {
            return steps;
        }

        Self::count_tiles(&input_clone)
    }

    pub fn part1() -> usize {
        let input = Self::process_input();
        for i in 0..input.len() {
            for j in 0..input[0].len() {
                if input[i][j].eq(&'S') {
                    return Self::get_steps_or_tiles(&input, i, j, true) / 2;
                }
            }
        }

        0
    }

    pub fn part2() -> usize {
        let input = Self::process_input();
        for i in 0..input.len() {
            for j in 0..input[0].len() {
                if input[i][j].eq(&'S') {
                    return Self::get_steps_or_tiles(&input, i, j, false);
                }
            }
        }

        0
    }
}

#[test]
fn test1() {
    let expected = 6812;
    assert_eq!(expected, Day10::part1());
}

#[test]
#[ignore = "not finished"]
fn test2() {
    let expected = 0;
    assert_eq!(expected, Day10::part2());
}
