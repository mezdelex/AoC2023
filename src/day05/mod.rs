use std::{collections::HashSet, u64};

use rayon::{
    iter::IntoParallelRefIterator,
    prelude::{IntoParallelIterator, ParallelIterator},
    slice::ParallelSlice,
};
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

    fn map_to_location(input: &Vec<Vec<u64>>, seed: u64) -> u64 {
        let mut mapped = seed;
        for i in 0..input[1..].len() {
            for j in (0..input[1..][i].len()).step_by(3) {
                if (input[1..][i][j + 1]..input[1..][i][j + 1] + input[1..][i][j + 2])
                    .contains(&mapped)
                {
                    mapped = input[1..][i][j] + mapped - input[1..][i][j + 1];
                    break;
                }
            }
        }

        mapped
    }

    pub fn part1() -> u64 {
        let input = Self::process_input();
        input[0]
            .par_iter()
            .map(|&seed| Self::map_to_location(&input, seed))
            .min()
            .unwrap()
    }

    pub fn part2() -> u64 {
        let input = Self::process_input();
        let seeds: HashSet<&u64> = HashSet::new();

        input[0].par_chunks(2).for_each(|chunk| {
            (chunk[0]..chunk[0] + chunk[1])
                .into_par_iter()
                .for_each(|seed| {
                    let seed_clone = seed.clone();
                    let mut seeds_clone = seeds.clone();
                    seeds_clone.insert(&seed_clone);
                });
        });

        seeds
            .par_iter()
            .map(|&seed| Self::map_to_location(&input, *seed))
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
#[ignore = "too heavy"]
fn test2() {
    let expected = 52510809;
    assert_eq!(expected, Day05::part2());
}
