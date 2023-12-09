use std::collections::VecDeque;

use rayon::{iter::ParallelIterator, str::ParallelString};

use crate::handler::Handler;

#[derive(Debug)]
pub struct Day03;

impl Day03 {
    fn process_input() -> Vec<Vec<char>> {
        Handler::new("./src/day03/input.txt")
            .handle_input()
            .unwrap()
            .par_lines()
            .map(|line| line.chars().collect())
            .collect()
    }

    fn fill_left(
        input: &mut Vec<Vec<char>>,
        number: &mut VecDeque<char>,
        x: usize,
        y: usize,
        k: usize,
    ) -> bool {
        match y as i32 - k as i32 >= 0 && input[x][y - k].is_digit(10) {
            true => {
                number.push_front(input[x][y - k]);
                input[x][y - k] = '.';
                return true;
            }
            _ => return false,
        };
    }

    fn fill_right(
        input: &mut Vec<Vec<char>>,
        number: &mut VecDeque<char>,
        x: usize,
        y: usize,
        k: usize,
    ) -> bool {
        match y + k < input[x].len() && input[x][y + k].is_digit(10) {
            true => {
                number.push_back(input[x][y + k]);
                input[x][y + k] = '.';
                return true;
            }
            _ => return false,
        }
    }

    fn fill_number(input: &mut Vec<Vec<char>>, number: &mut VecDeque<char>, x: usize, y: usize) {
        number.push_back(input[x][y]);
        input[x][y] = '.';
        let mut k = 1;
        loop {
            if !Self::fill_left(input, number, x, y, k) {
                break;
            }
            k += 1;
        }
        k = 1;
        loop {
            if !Self::fill_right(input, number, x, y, k) {
                break;
            }
            k += 1;
        }
    }

    fn add_surround_numbers(
        input: &mut Vec<Vec<char>>,
        vec_of_values: &mut Vec<usize>,
        i: usize,
        j: usize,
        is_part2: bool,
    ) {
        let mut added_values: Vec<usize> = vec![];
        for x in i.saturating_sub(1)..=i + 1 {
            for y in j.saturating_sub(1)..=j + 1 {
                if is_part2 && added_values.len() > 2 {
                    return;
                }

                if x as i32 >= 0 && y as i32 >= 0 {
                    if !input[x][y].is_digit(10) {
                        continue;
                    }
                    let mut number: VecDeque<char> = VecDeque::new();
                    Self::fill_number(input, &mut number, x, y);
                    added_values.push(number.iter().collect::<String>().parse().unwrap());
                }
            }
        }

        if is_part2 {
            if added_values.len() > 1 {
                vec_of_values.push(added_values[0] * added_values[1]);
            }
        } else {
            vec_of_values.extend(added_values);
        }
    }

    pub fn part1() -> usize {
        let mut input = Self::process_input();
        let mut vec_of_values: Vec<usize> = vec![];
        for i in 0..input.len() {
            for j in 0..input[0].len() {
                if !input[i][j].is_digit(10) && !input[i][j].eq(&'.') {
                    Self::add_surround_numbers(&mut input, &mut vec_of_values, i, j, false);
                }
            }
        }

        vec_of_values.iter().sum()
    }

    pub fn part2() -> usize {
        let mut input = Self::process_input();
        let mut vec_of_values: Vec<usize> = vec![];
        for i in 0..input.len() {
            for j in 0..input[0].len() {
                if input[i][j].eq(&'*') {
                    Self::add_surround_numbers(&mut input, &mut vec_of_values, i, j, true);
                }
            }
        }

        vec_of_values.iter().sum()
    }
}

#[test]
fn test1() {
    let expected = 546563;
    assert_eq!(expected, Day03::part1());
}

#[test]
fn test2() {
    let expected = 91031374;
    assert_eq!(expected, Day03::part2());
}
