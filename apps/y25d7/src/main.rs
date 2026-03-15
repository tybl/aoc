use clap::Parser;
use helpers::CliArguments;
use std::collections::{HashMap, hash_map::Entry};
use std::fs;

fn main() {
    let args = CliArguments::parse();
    let input = fs::read_to_string(args.input).unwrap();
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &str) -> usize {
    let mut input: Vec<Vec<u8>> = input.lines().map(|l| l.bytes().collect()).collect();
    let length = input.len();
    let width = input.first().unwrap().len();
    let mut count = 0;
    for r in 1..length {
        for c in 0..width {
            let prev_row = r - 1;
            let prev_col = c.checked_sub(1);
            let next_col = c + 1;
            if input[prev_row][c] == 'S' as u8 {
                match input[r][c] as char {
                    '^' => {
                        if let Some(col) = prev_col {
                            input[r][col] = 'S' as u8;
                        }
                        input[r][next_col] = 'S' as u8;
                        count += 1;
                    }
                    '.' => input[r][c] = 'S' as u8,
                    'S' => input[r][c] = 'S' as u8,
                    _ => panic!("Unexpected input: {}", input[r][c] as char),
                }
            }
        }
    }
    count
}

fn part_two(input: &str) -> usize {
    let input: Vec<Vec<u8>> = input.lines().map(|l| l.bytes().collect()).collect();
    let mut memo = HashMap::new();
    let start_col = input
        .first()
        .unwrap()
        .iter()
        .position(|c| *c == 'S' as u8)
        .unwrap();
    dfs(&input, 1, start_col, &mut memo)
}

fn dfs(
    tree: &Vec<Vec<u8>>,
    row: usize,
    col: usize,
    mut memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if row < tree.len() {
        match memo.entry((row, col)) {
            Entry::Occupied(o) => *o.get(),
            Entry::Vacant(_) => {
                let count = match tree[row][col] as char {
                    '^' => {
                        let left = dfs(&tree, row, col - 1, &mut memo);
                        let right = dfs(&tree, row, col + 1, &mut memo);
                        left + right
                    }
                    '.' => dfs(&tree, row + 1, col, &mut memo),
                    _ => panic!("Unexpected input: {}", tree[row][col] as char),
                };
                memo.insert((row, col), count);
                count
            }
        }
    } else {
        1
    }
}
