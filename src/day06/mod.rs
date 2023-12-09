use rayon::{
    iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator},
    str::ParallelString,
};
use regex::Regex;

use crate::handler::Handler;

#[derive(Debug)]
pub struct Day06;

impl Day06 {
    fn process_input() -> Vec<Vec<u64>> {
        let regex = Regex::new(r"\D+").unwrap();
        Handler::new("./src/day06/input.txt")
            .handle_input()
            .unwrap()
            .par_lines()
            .map(|line| {
                regex
                    .split(&line)
                    .map(|num| num.parse::<u64>().unwrap_or_default())
                    .skip(1)
                    .collect::<Vec<u64>>()
            })
            .collect()
    }

    fn get_chances(time: &u64, distance: u64, time_holding: u64) -> u64 {
        if time_holding == *time {
            return 0;
        }

        if (time - time_holding) * time_holding <= distance {
            return Self::get_chances(time, distance, time_holding + 1);
        }

        1 + Self::get_chances(time, distance, time_holding + 1)
    }

    pub fn part1() -> u64 {
        let input = Self::process_input();
        input[0]
            .par_iter()
            .enumerate()
            .map(|(idx, time)| {
                let chances = Self::get_chances(time, input[1][idx], 1);

                if chances == 0 {
                    1
                } else {
                    chances
                }
            })
            .product()
    }

    pub fn part2() -> u64 {
        let input = Self::process_input();
        let time: u64 = input[0]
            .par_iter()
            .map(|&num| num.to_string())
            .collect::<String>()
            .parse()
            .unwrap();
        let distance: u64 = input[1]
            .par_iter()
            .map(|&num| num.to_string())
            .collect::<String>()
            .parse()
            .unwrap();

        let mut chances = 0;
        for holding_time in 1..time {
            let speed = time - holding_time;
            let chance_time = time - speed;

            if speed * chance_time > distance {
                chances += 1;
            }
        }

        chances
    }
}

#[test]
fn test1() {
    let expected = 1108800;
    assert_eq!(expected, Day06::part1());
}

#[test]
fn test2() {
    let expected = 36919753;
    assert_eq!(expected, Day06::part2());
}
