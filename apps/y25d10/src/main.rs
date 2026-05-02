#![warn(clippy::pedantic)]
use clap::Parser;
use helpers::CliArguments;
use std::collections::VecDeque;
use std::fs;

fn main() {
    let args = CliArguments::parse();
    let input = fs::read_to_string(args.input).unwrap();
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &str) -> usize {
    input
        .lines()
        .map(|s| {
            let (goal, buttons) = s.rsplit_once(' ').unwrap().0.split_once(' ').unwrap();
            let buttons = buttons
                .split(' ')
                .map(generate_button)
                .collect::<Vec<u16>>();
            let goal = parse_goal(goal);
            find_min_button_presses(goal, &buttons)
        })
        .sum()
}

fn part_two(_input: &str) -> usize {
    0
}

fn parse_goal(input: &str) -> u16 {
    input
        .trim_start_matches("[")
        .trim_end_matches("]")
        .bytes()
        .rev()
        .map(|c| if c as char == '#' { 1 } else { 0 })
        .fold(0, |acc, n| (acc << 1) + n)
}

fn generate_button(input: &str) -> u16 {
    input
        .trim_start_matches("(")
        .trim_end_matches(")")
        .split(',')
        .map(|s| 1 << s.parse::<u16>().unwrap())
        .fold(0, |acc, n| acc | n)
}

fn find_min_button_presses(target: u16, buttons: &Vec<u16>) -> usize {
    xor_bfs(target, buttons)
}

fn xor_bfs(goal: u16, edges: &Vec<u16>) -> usize {
    let mut queue: VecDeque<(u16, usize)> = VecDeque::new();
    queue.push_back((0, 0));
    loop {
        let (x, count) = queue.pop_front().unwrap();
        if x == goal {
            return count;
        }
        let count = count + 1;
        for e in edges {
            queue.push_back((x ^ e, count));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_goal() {
        assert_eq!(0, parse_goal("[...]"));
        assert_eq!(6, parse_goal("[.##.]"));
        assert_eq!(8, parse_goal("[...#.]"));
        assert_eq!(46, parse_goal("[.###.#]"));
    }

    #[test]
    fn test_generate_button() {
        assert_eq!(1, generate_button("(0)"));
        assert_eq!(2, generate_button("(1)"));
        assert_eq!(4, generate_button("(2)"));
        assert_eq!(8, generate_button("(3)"));
        assert_eq!(16, generate_button("(4)"));
        assert_eq!(32, generate_button("(5)"));
        assert_eq!(64, generate_button("(6)"));
        assert_eq!(10, generate_button("(1,3)"));
        assert_eq!(12, generate_button("(2,3)"));
        assert_eq!(29, generate_button("(0,2,3,4)"));
        assert_eq!(55, generate_button("(0,1,2,4,5)"));
    }

    #[test]
    fn test_find_min_button_presses() {
        assert_eq!(2, find_min_button_presses(6, &vec![8, 10, 4, 12, 5, 3]));
        assert_eq!(3, find_min_button_presses(8, &vec![29, 12, 17, 7, 30]));
        assert_eq!(2, find_min_button_presses(46, &vec![31, 25, 55, 6]));
    }
}
