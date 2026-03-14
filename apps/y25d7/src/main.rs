use clap::Parser;
use helpers::CliArguments;
use std::fs;

fn main() {
    let args = CliArguments::parse();
    let input = fs::read_to_string(args.input).unwrap();
    println!("Part one: {}", part_one(&input));
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
    for r in input {
        for c in r {
            print!("{}", c as char);
        }
        println!("");
    }
    count
}
