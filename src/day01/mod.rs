use crate::handler::Handler;

pub struct Day01;

impl Day01 {
    fn process_input() -> String {
        let path_to_file = "./src/day01/input.txt".to_string();
        let handler = Handler::new(&path_to_file);
        let processed_input = handler.handle_input();
        // this might return a vec! of something, depending on the exercise

        processed_input.unwrap().trim().to_string()
    }

    pub fn part1() -> String {
        let input = Self::process_input();
        // lines, etc.
        input
    }
    pub fn part2() -> String {
        let input = Self::process_input();
        // lines, etc.
        input
    }
}

#[test]
fn test1() {
    let expected = "todo".to_string();
    assert_eq!(expected, Day01::part1());
}

#[test]
fn test2() {
    let expected = "todo".to_string();
    assert_eq!(expected, Day01::part2());
}
