use std::i64;

use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    str::ParallelString,
};

use crate::handler::Handler;

#[derive(Debug)]
pub struct Day09;

impl Day09 {
    fn process_input() -> Vec<Vec<i64>> {
        Handler::new("./src/day09/input.txt")
            .handle_input()
            .unwrap()
            .par_lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|split| split.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>()
            })
            .collect()
    }

    fn fill_secuences(secuences: &mut Vec<Vec<i64>>, secuence: &Vec<i64>) {
        if secuence.par_iter().all(|value| *value == 0) {
            return;
        }

        let mut next_secuence: Vec<i64> = Vec::new();
        secuence.iter().reduce(|acc, next| {
            next_secuence.push(next - acc);
            next
        });
        secuences.push(next_secuence.clone());

        Self::fill_secuences(secuences, &next_secuence)
    }

    fn calculate_extrapolation(history: &Vec<i64>, is_part2: bool) -> i64 {
        let mut secuences: Vec<Vec<i64>> = Vec::new();
        secuences.push(history.clone());
        Self::fill_secuences(&mut secuences, history);
        secuences.reverse();

        if is_part2 {
            return secuences.iter().fold(0, |acc, next| next[0] - acc);
        }

        secuences
            .iter()
            .fold(0, |acc, next| next[next.len() - 1] + acc)
    }

    pub fn part1() -> i64 {
        Self::process_input()
            .par_iter()
            .map(|history| Self::calculate_extrapolation(history, false))
            .sum()
    }

    pub fn part2() -> i64 {
        Self::process_input()
            .par_iter()
            .map(|history| Self::calculate_extrapolation(history, true))
            .sum()
    }
}

#[test]
fn test1() {
    let expected = 1980437560;
    assert_eq!(expected, Day09::part1());
}

#[test]
fn test2() {
    let expected = 977;
    assert_eq!(expected, Day09::part2());
}
