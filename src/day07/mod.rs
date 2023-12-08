use std::{cmp::Ordering, collections::HashMap};

use crate::handler::Handler;

#[derive(Debug)]
pub struct Hand {
    pub bid: usize,
    pub cards: Vec<char>,
    pub strength: u32,
}

impl Hand {
    pub fn new(cards: Vec<char>, bid: usize) -> Self {
        Hand {
            bid,
            cards,
            strength: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct Day07 {
    pub card_converter1: HashMap<char, u32>,
    pub card_converter2: HashMap<char, u32>,
}

impl Day07 {
    fn new() -> Self {
        Day07 {
            card_converter1: HashMap::from([
                ('2', 2),
                ('3', 3),
                ('4', 4),
                ('5', 5),
                ('6', 6),
                ('7', 7),
                ('8', 8),
                ('9', 9),
                ('T', 10),
                ('J', 11),
                ('Q', 12),
                ('K', 13),
                ('A', 14),
            ]),
            card_converter2: HashMap::from([
                ('J', 1),
                ('2', 2),
                ('3', 3),
                ('4', 4),
                ('5', 5),
                ('6', 6),
                ('7', 7),
                ('8', 8),
                ('9', 9),
                ('T', 10),
                ('Q', 11),
                ('K', 12),
                ('A', 13),
            ]),
        }
    }

    fn calculate_hand_value(hand: &mut Hand, is_part2: bool) {
        let mut coincidences: HashMap<char, u32> = HashMap::new();
        hand.cards.iter().for_each(|&card| {
            *coincidences.entry(card).or_insert(0) += 1;
        });
        match coincidences.keys().count() {
            1 => hand.strength = 7,
            2 => {
                if coincidences.values().any(|&value| value == 4) {
                    hand.strength = 6
                } else {
                    hand.strength = 5
                }
            }
            3 => {
                if coincidences.values().any(|&value| value == 3) {
                    hand.strength = 4
                } else {
                    hand.strength = 3
                }
            }
            4 => hand.strength = 2,
            _ => hand.strength = 1,
        }
        if is_part2 && coincidences.contains_key(&'J') {
            match hand.strength {
                7 => {}
                6 | 1 => hand.strength += 1,
                3 => hand.strength += coincidences.get(&'J').unwrap() + 1,
                _ => hand.strength += 2,
            }
        }
    }

    fn process_input(is_part2: bool) -> Vec<Hand> {
        Handler::new("./src/day07/input.txt")
            .handle_input()
            .unwrap()
            .lines()
            .map(|line| {
                let values: Vec<&str> = line.split_whitespace().collect();
                let mut hand = Hand::new(values[0].chars().collect(), values[1].parse().unwrap());
                Self::calculate_hand_value(&mut hand, is_part2);
                hand
            })
            .collect::<Vec<Hand>>()
    }

    pub fn part1() -> usize {
        let mut hands = Self::process_input(false);
        let day_handler = Self::new();
        hands.sort_by(|prev, next| {
            if prev.strength == next.strength {
                for i in 0..prev.cards.len() {
                    let ordering = day_handler
                        .card_converter1
                        .get(&prev.cards[i])
                        .unwrap()
                        .cmp(day_handler.card_converter1.get(&next.cards[i]).unwrap());

                    if ordering != Ordering::Equal {
                        return ordering;
                    }
                }
            }
            prev.strength.cmp(&next.strength)
        });
        hands
            .iter()
            .enumerate()
            .map(|(idx, card)| (idx + 1) * card.bid)
            .sum()
    }

    pub fn part2() -> usize {
        let mut hands = Self::process_input(true);
        let day_handler = Self::new();
        hands.sort_by(|prev, next| {
            if prev.strength == next.strength {
                for i in 0..prev.cards.len() {
                    let ordering = day_handler
                        .card_converter2
                        .get(&prev.cards[i])
                        .unwrap()
                        .cmp(day_handler.card_converter2.get(&next.cards[i]).unwrap());

                    if ordering != Ordering::Equal {
                        return ordering;
                    }
                }
            }
            prev.strength.cmp(&next.strength)
        });
        hands
            .iter()
            .enumerate()
            .map(|(idx, card)| (idx + 1) * card.bid)
            .sum()
    }
}

#[test]
fn test1() {
    let expected = 248569531;
    assert_eq!(expected, Day07::part1());
}

#[test]
fn test2() {
    let expected = 250382098;
    assert_eq!(expected, Day07::part2());
}
