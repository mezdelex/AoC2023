use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use regex::Regex;

use crate::handler::Handler;

#[derive(Debug)]
pub struct Day05;

impl Day05 {
    fn process_input() -> Vec<Vec<u64>> {
        Regex::new(r".+: ?")
            .unwrap()
            .split(
                &Handler::new("./src/day05/input.txt")
                    .handle_input()
                    .unwrap(),
            )
            .skip(1)
            .map(|chunk| {
                chunk
                    .split_whitespace()
                    .map(|num| num.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>()
            })
            .collect()
    }

    pub fn part1() -> u64 {
        let input = Self::process_input();
        input[0]
            .iter()
            .map(|&seed| {
                let mut current = seed;
                let mut min = u64::MAX;

                for i in 0..input[1..].len() {
                    for j in (0..input[1..][i].len()).step_by(3) {
                        if (input[1..][i][j + 1]..input[1..][i][j + 1] + input[1..][i][j + 2])
                            .contains(&current)
                        {
                            current = input[1..][i][j] + current - input[1..][i][j + 1];
                            break;
                        }
                    }
                    if i == input[1..].len() - 1 && current < min {
                        min = current;
                    }
                }
                min
            })
            .min()
            .unwrap()
    }

    pub fn part2() -> u64 {
        let input = Self::process_input();
        input[0]
            .chunks(2)
            .map(|chunk| {
                (chunk[0]..=chunk[0] + chunk[1])
                    .into_par_iter()
                    .map(|seed| {
                        let mut current = seed;
                        let mut min = u64::MAX;

                        for i in 0..input[1..].len() {
                            for j in (0..input[1..][i].len()).step_by(3) {
                                if (input[1..][i][j + 1]
                                    ..input[1..][i][j + 1] + input[1..][i][j + 2])
                                    .contains(&current)
                                {
                                    current = input[1..][i][j] + current - input[1..][i][j + 1];
                                    break;
                                }
                            }
                            if i == input[1..].len() - 1 && current < min {
                                min = current;
                            }
                        }
                        min
                    })
                    .min()
                    .unwrap()
            })
            .min()
            .unwrap()
    }
}

#[test]
fn test1() {
    let expected = 662197086;
    assert_eq!(expected, Day05::part1());
}

#[test]
#[ignore = "brute force, too heavy to run in ci"]
fn test2() {
    let expected = 52510809;
    assert_eq!(expected, Day05::part2());
}
