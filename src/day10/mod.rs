use rayon::{iter::ParallelIterator, str::ParallelString};

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

    fn get_steps(input: &Vec<Vec<char>>, i: usize, j: usize) -> usize {
        let mut steps = 1;
        let mut position = (i, j + 1);
        let mut direction = Direction::East;

        loop {
            if input[position.0][position.1].eq(&'S') {
                break;
            }

            (direction, position) = match (direction, input[position.0][position.1]) {
                (Direction::North, '|') => (Direction::North, (position.0 - 1, position.1)),
                (Direction::South, '|') => (Direction::South, (position.0 + 1, position.1)),
                (Direction::West, '-') => (Direction::West, (position.0, position.1 - 1)),
                (Direction::East, '-') => (Direction::East, (position.0, position.1 + 1)),
                (Direction::West, 'L') => (Direction::North, (position.0 - 1, position.1)),
                (Direction::South, 'L') => (Direction::East, (position.0, position.1 + 1)),
                (Direction::East, 'J') => (Direction::North, (position.0 - 1, position.1)),
                (Direction::South, 'J') => (Direction::West, (position.0, position.1 - 1)),
                (Direction::East, '7') => (Direction::South, (position.0 + 1, position.1)),
                (Direction::North, '7') => (Direction::West, (position.0, position.1 - 1)),
                (Direction::West, 'F') => (Direction::South, (position.0 + 1, position.1)),
                (Direction::North, 'F') => (Direction::East, (position.0, position.1 + 1)),
                _ => panic!("Start in another direction and position."),
            };

            steps += 1;
        }

        steps
    }

    pub fn part1() -> usize {
        let input = Self::process_input();
        for i in 0..input.len() {
            for j in 0..input[0].len() {
                if input[i][j].eq(&'S') {
                    return Self::get_steps(&input, i, j) / 2;
                }
            }
        }

        0
    }

    pub fn part2() -> usize {
        todo!()
    }
}

#[test]
fn test1() {
    let expected = 6812;
    assert_eq!(expected, Day10::part1());
}

#[test]
#[ignore = "not implemented"]
fn test2() {
    let expected = 0;
    assert_eq!(expected, Day10::part2());
}
