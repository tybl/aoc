#![warn(clippy::pedantic)]
use clap::Parser;
use helpers::CliArguments;
use std::fs;

fn main() {
    let args = CliArguments::parse();
    let input = fs::read_to_string(args.input).unwrap();
    println!("Part one: {}", one::part_one(&input));
    println!("Part two: {}", two::part_two(&input));
}

mod one {
    use std::collections::VecDeque;

    pub fn part_one(input: &str) -> usize {
        input
            .lines()
            .map(|s| {
                let (goal, buttons) = s.rsplit_once(' ').unwrap().0.split_once(' ').unwrap();
                let buttons = buttons
                    .split(' ')
                    .map(generate_button)
                    .collect::<Vec<u16>>();
                let goal = parse_lights(goal);
                find_min_button_presses_for_lights(goal, &buttons)
            })
            .sum()
    }

    fn parse_lights(input: &str) -> u16 {
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

    fn find_min_button_presses_for_lights(target: u16, buttons: &Vec<u16>) -> usize {
        bfs(target, buttons)
    }

    fn bfs(goal: u16, edges: &Vec<u16>) -> usize {
        let mut queue: VecDeque<(u16, usize)> = VecDeque::new();
        let mut explored = vec![false; u16::MAX as usize + 1];
        queue.push_back((0, 0));
        while !queue.is_empty() {
            let (x, count) = queue.pop_front().unwrap();
            if x == goal {
                return count;
            }
            let count = count + 1;
            for e in edges {
                let n = x ^ e;
                if !explored[n as usize] {
                    explored[n as usize] = true;
                    queue.push_back((n, count));
                }
            }
        }
        usize::MAX
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_parse_lights() {
            assert_eq!(0, parse_lights("[...]"));
            assert_eq!(6, parse_lights("[.##.]"));
            assert_eq!(8, parse_lights("[...#.]"));
            assert_eq!(46, parse_lights("[.###.#]"));
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
            assert_eq!(
                2,
                find_min_button_presses_for_lights(6, &vec![8, 10, 4, 12, 5, 3])
            );
            assert_eq!(
                3,
                find_min_button_presses_for_lights(8, &vec![29, 12, 17, 7, 30])
            );
            assert_eq!(
                2,
                find_min_button_presses_for_lights(46, &vec![31, 25, 55, 6])
            );
        }
    }
}

mod two {
    use itertools::Itertools;
    use nalgebra::SVector;
    use std::{
        collections::{BTreeMap, HashMap, HashSet},
        iter,
    };

    type Joltage = SVector<u16, 10>;

    pub fn part_two(input: &str) -> usize {
        input
            .lines()
            .map(|s| {
                let (buttons, goal) = s.split_once(' ').unwrap().1.rsplit_once(' ').unwrap();
                let buttons = buttons
                    .split(' ')
                    .map(generate_vec_button)
                    .collect::<Vec<Joltage>>();
                let goal = parse_joltage(goal);
                let edges = buttons
                    .iter()
                    .map(to_pattern)
                    .enumerate()
                    .collect::<BTreeMap<usize, u16>>();
                let lookup = find_all_sequences(edges);
                let result = solve_single(0, goal, &buttons, &lookup);
                //println!("{:?} -- {:?} = {result}", goal, buttons);
                result
            })
            .sum()
    }

    fn generate_vec_button(input: &str) -> Joltage {
        let result = input
            .trim_start_matches("(")
            .trim_end_matches(")")
            .split(',')
            .fold(Joltage::zeros(), |mut acc, s| {
                acc[s.parse::<usize>().unwrap()] = 1;
                acc
            });
        result
    }

    fn parse_joltage(input: &str) -> Joltage {
        input
            .trim_start_matches("{")
            .trim_end_matches("}")
            .split(',')
            .enumerate()
            .fold(Joltage::zeros(), |mut acc, (i, s)| {
                acc[i] = s.parse().unwrap();
                acc
            })
    }

    fn solve_single(
        depth: u16,
        goal: Joltage,
        edges: &Vec<Joltage>,
        lookup: &HashMap<u16, HashSet<BTreeMap<usize, usize>>>,
    ) -> usize {
        if goal == Joltage::zeros() {
            return 0;
        }
        // Find pattern of lights for given Joltage levels
        // Find all button combos that result in the given light pattern
        // For each button combos
        //   Subtract button combo from Joltage
        //   Divide remaining Joltage by 2
        //   Call solve_single with the new Joltage goal
        //   return minimum result from calling solve_single
        let goal_pattern = to_pattern(&goal);
        let mut min_count = 1000000;
        if !lookup.contains_key(&goal_pattern) {
            return min_count;
        }
        let sequences = &lookup[&goal_pattern];
        for bp in sequences {
            let button_press_joltage = bp.iter().fold(Joltage::zeros(), |acc, (&i, count)| {
                acc + edges[i] * *count as u16
            });
            if iter::zip(goal.iter(), button_press_joltage.iter()).any(|(g, c)| g < c) {
                continue;
            }
            let inter = goal - button_press_joltage;
            let new_goal = (goal - button_press_joltage) / 2;
            if new_goal * 2 != inter {
                let count: usize = solve_single(depth + 1, inter, edges, lookup)
                    + bp.iter().map(|(_, n)| n).sum::<usize>();
                min_count = count.min(min_count);
            } else {
                let count: usize = 2 * solve_single(depth + 1, new_goal, edges, lookup)
                    + bp.iter().map(|(_, n)| n).sum::<usize>();
                min_count = count.min(min_count);
            }
        }
        min_count
    }

    fn to_pattern(input: &Joltage) -> u16 {
        input
            .iter()
            .enumerate()
            .fold(0u16, |acc, (i, n)| acc | n % 2 << i)
    }

    fn find_all_sequences(
        edges: BTreeMap<usize, u16>,
    ) -> HashMap<u16, HashSet<BTreeMap<usize, usize>>> {
        let mut result: HashMap<u16, HashSet<BTreeMap<usize, usize>>> = HashMap::new();
        for count in 1..=edges.len() {
            for button_presses in edges.iter().combinations(count) {
                let mut path: BTreeMap<usize, usize> = BTreeMap::new();
                let mut state = 0u16;
                for (index, value) in button_presses {
                    *path.entry(*index).or_default() += 1;
                    state = state ^ *value;
                }
                result.entry(state).or_default().insert(path.clone());
                result.entry(0).or_default().insert(path);
            }
        }
        //println!("{result:?}");
        result
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use nalgebra::vector;

        #[test]
        fn test_parse_joltage() {
            assert_eq!(
                vector![3, 5, 4, 7, 0, 0, 0, 0, 0, 0],
                parse_joltage("{3,5,4,7}")
            );
        }

        #[test]
        fn no_answer() {
            let goal = vector![55, 8, 40, 49, 30, 37, 0, 0, 0, 0];
            let buttons = vec![
                vector![0, 1, 0, 1, 1, 0, 0, 0, 0, 0],
                vector![1, 0, 1, 1, 1, 1, 0, 0, 0, 0],
                vector![0, 0, 1, 1, 0, 0, 0, 0, 0, 0],
                vector![1, 0, 0, 1, 0, 1, 0, 0, 0, 0],
                vector![1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                vector![1, 0, 1, 0, 1, 0, 0, 0, 0, 0],
            ];
            let edges = buttons
                .iter()
                .map(to_pattern)
                .enumerate()
                .collect::<BTreeMap<usize, u16>>();
            let lookup = find_all_sequences(edges);

            assert_eq!(67, solve_single(0, goal, &buttons, &lookup));
        }

        #[test]
        fn next_no_answer() {
            let goal = vector![220, 13, 8, 220, 220, 0, 0, 0, 0, 0];
            let buttons = vec![
                vector![1, 0, 1, 1, 1, 0, 0, 0, 0, 0],
                vector![1, 1, 0, 1, 1, 0, 0, 0, 0, 0],
                vector![1, 0, 0, 1, 1, 0, 0, 0, 0, 0],
            ];
            let edges = buttons
                .iter()
                .map(to_pattern)
                .enumerate()
                .collect::<BTreeMap<usize, u16>>();
            let lookup = find_all_sequences(edges);

            assert_eq!(220, solve_single(0, goal, &buttons, &lookup));
        }

        #[test]
        fn next_next_no_answer() {
            // [[143, 136, 136, 28, 141, 0, 0, 0, 0, 0]] -- [[[1, 1, 1, 0, 1, 0, 0, 0, 0, 0]], [[0, 1, 1, 1, 1, 0, 0, 0, 0, 0]], [[1, 0, 0, 0, 1, 0, 0, 0, 0, 0]], [[1, 0, 0, 1, 0, 0, 0, 0, 0, 0]]] = 1000000
            let goal = vector![143, 136, 136, 28, 141, 0, 0, 0, 0, 0];
            let buttons = vec![
                vector![1, 1, 1, 0, 1, 0, 0, 0, 0, 0],
                vector![0, 1, 1, 1, 1, 0, 0, 0, 0, 0],
                vector![1, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                vector![1, 0, 0, 1, 0, 0, 0, 0, 0, 0],
            ];
            let edges = buttons
                .iter()
                .map(to_pattern)
                .enumerate()
                .collect::<BTreeMap<usize, u16>>();
            let lookup = find_all_sequences(edges);

            assert_eq!(156, solve_single(0, goal, &buttons, &lookup));
        }
    }
}
