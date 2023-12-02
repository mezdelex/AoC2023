use std::collections::HashMap;

use regex::Regex;

use crate::handler::Handler;

#[derive(Debug)]
pub struct Day02 {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Day02 {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        Day02 { red, green, blue }
    }

    fn process_input() -> Vec<Vec<HashMap<String, u32>>> {
        let handler = Handler::new("./src/day02/input.txt");
        let processed_input = handler
            .handle_input()
            .unwrap()
            .lines()
            .map(|line| {
                let isolated_grabs = Regex::new(r"Game \d+: ")
                    .unwrap()
                    .split(line)
                    .collect::<Vec<&str>>()[1];

                let vec_of_grabs: Vec<&str> = isolated_grabs.split("; ").collect();

                let processed_line = vec_of_grabs
                    .iter()
                    .map(|grab| {
                        let values_and_keys: Vec<&str> =
                            Regex::new("[, ]+").unwrap().split(&grab).collect();
                        let mut grab_map: HashMap<String, u32> = HashMap::new();
                        values_and_keys.chunks(2).for_each(|chunk| {
                            grab_map.insert(chunk[1].to_string(), chunk[0].parse().unwrap());
                        });

                        grab_map
                    })
                    .collect();

                processed_line
            })
            .collect();

        processed_input
    }

    pub fn part1() -> usize {
        let max_values = Self::new(12, 13, 14);
        let input = Self::process_input();
        input
            .iter()
            .enumerate()
            .map(|(game, vec_of_grabs)| {
                match vec_of_grabs.iter().any(|grab| {
                    grab.get("red").unwrap_or(&0) > &max_values.red
                        || grab.get("green").unwrap_or(&0) > &max_values.green
                        || grab.get("blue").unwrap_or(&0) > &max_values.blue
                }) {
                    true => 0,
                    false => game + 1,
                }
            })
            .sum()
    }

    pub fn part2() -> u32 {
        let input = Self::process_input();
        input
            .iter()
            .map(|vec_of_grabs| {
                let mut min_values = Self::new(1, 1, 1);
                vec_of_grabs.iter().for_each(|grab| {
                    if let Some(&value) = grab.get("red") {
                        if value > min_values.red || min_values.red.eq(&1) {
                            min_values.red = value;
                        }
                    }
                    if let Some(&value) = grab.get("green") {
                        if value > min_values.green || min_values.green.eq(&1) {
                            min_values.green = value;
                        }
                    }
                    if let Some(&value) = grab.get("blue") {
                        if value > min_values.blue || min_values.blue.eq(&1) {
                            min_values.blue = value;
                        }
                    }
                });

                min_values.red * min_values.green * min_values.blue
            })
            .sum()
    }
}

#[test]
fn test1() {
    let expected = 2617;
    assert_eq!(expected, Day02::part1());
}

#[test]
fn test2() {
    let expected = 59795;
    assert_eq!(expected, Day02::part2());
}
