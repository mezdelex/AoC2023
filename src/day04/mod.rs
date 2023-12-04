use crate::handler::Handler;

#[derive(Debug)]
pub struct Day04;

impl Day04 {
    fn process_input() -> Vec<Vec<Vec<u32>>> {
        let handler = Handler::new("./src/day04/input.txt");

        handler
            .handle_input()
            .unwrap()
            .lines()
            .map(|line| {
                line.split(": ").collect::<Vec<&str>>()[1]
                    .split(" | ")
                    .map(|stack_of_numbers| {
                        stack_of_numbers
                            .split_whitespace()
                            .map(|number| number.parse::<u32>().unwrap())
                            .collect::<Vec<u32>>()
                    })
                    .collect::<Vec<Vec<u32>>>()
            })
            .collect()
    }

    pub fn part1() -> u32 {
        Self::process_input()
            .iter()
            .map(|pile| {
                let coincidences = pile[1]
                    .iter()
                    .filter(|&number| pile[0].contains(number))
                    .count();

                if coincidences > 1 {
                    return 2_u32.pow(coincidences as u32) - 1;
                }

                coincidences as u32
            })
            .sum()
    }

    pub fn part2() -> u32 {
        let mut extra_cards: Vec<u32> = vec![];
        Self::process_input()
            .iter()
            .map(|pile| {
                let coincidences = pile[1]
                    .iter()
                    .filter(|&number| pile[0].contains(number))
                    .count();

                let mut current_extra_cards = 0;
                if !extra_cards.is_empty() {
                    current_extra_cards = extra_cards[0];
                    extra_cards.remove(0);
                }

                for i in 0..coincidences {
                    if let Some(card_position) = extra_cards.get_mut(i) {
                        *card_position += 1 + current_extra_cards
                    } else {
                        extra_cards.push(1 + current_extra_cards)
                    }
                }

                1 + current_extra_cards
            })
            .sum()
    }
}

#[test]
fn test1() {
    let expected = 21138;
    assert_eq!(expected, Day04::part1());
}

#[test]
fn test2() {
    let expected = 7185540;
    assert_eq!(expected, Day04::part2());
}
