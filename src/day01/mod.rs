use std::collections::{HashMap, VecDeque};

use crate::handler::Handler;

#[derive(Debug)]
pub struct Day01;

impl Day01 {
    fn get_hashmap() -> HashMap<String, u32> {
        HashMap::from([
            ("one".to_string(), 1),
            ("two".to_string(), 2),
            ("three".to_string(), 3),
            ("four".to_string(), 4),
            ("five".to_string(), 5),
            ("six".to_string(), 6),
            ("seven".to_string(), 7),
            ("eight".to_string(), 8),
            ("nine".to_string(), 9),
        ])
    }

    fn process_input() -> Vec<String> {
        Handler::new("./src/day01/input.txt")
            .handle_input()
            .unwrap()
            .lines()
            .map(|line| line.to_string())
            .collect()
    }

    pub fn part1() -> u32 {
        Self::process_input()
            .iter()
            .map(|line| {
                let numbers_as_chars: Vec<char> = line.chars().filter(|c| c.is_numeric()).collect();

                if numbers_as_chars.is_empty() {
                    return 0;
                }

                numbers_as_chars[0].to_digit(10).unwrap() * 10
                    + numbers_as_chars[numbers_as_chars.len() - 1]
                        .to_digit(10)
                        .unwrap()
            })
            .sum()
    }

    pub fn part2() -> u32 {
        let map = Self::get_hashmap();
        Self::process_input()
            .iter()
            .map(|line| {
                let mut numbers: Vec<u32> = vec![];
                let mut word: VecDeque<char> = VecDeque::new();

                line.chars().for_each(|c| {
                    if c.is_numeric() {
                        numbers.push(c.to_digit(10).unwrap());
                        word.clear();
                    } else {
                        word.push_back(c);
                        let word_as_string: String = word.iter().collect();

                        if map.keys().any(|key| key.contains(&word_as_string)) {
                            if let Some(&number) = map.get(&word_as_string) {
                                numbers.push(number);

                                let overlapping_value = word.pop_back().unwrap();
                                word.clear();
                                word.push_back(overlapping_value);
                            }
                        } else {
                            word.pop_front();
                        }
                    }
                });

                if numbers.is_empty() {
                    return 0;
                }

                numbers[0] * 10 + numbers[numbers.len() - 1]
            })
            .sum()
    }
}

#[test]
fn test1() {
    let expected = 55208;
    assert_eq!(expected, Day01::part1());
}

#[test]
fn test2() {
    let expected = 54578;
    assert_eq!(expected, Day01::part2());
}
