use clap::Parser;
use helpers::CliArguments;
use std::fs;

fn remove_rolls(input: &mut Vec<Vec<u8>>) -> i32 {
    let mut removed_count = 0;
    let height = input.len();
    for row in 0..height {
        let width = input[row].len();
        for col in 0..width {
            let prev_row = row.checked_sub(1);
            let prev_col = col.checked_sub(1);
            let next_row = row + 1;
            let next_col = col + 1;
            if input[row][col] == '@' as u8 {
                let mut surround_count = 0;
                if let Some(prev_row) = prev_row {
                    if let Some(prev_col) = prev_col {
                        if input[prev_row][prev_col] != '.' as u8 {
                            surround_count += 1;
                        }
                    }
                    if input[prev_row][col] != '.' as u8 {
                        surround_count += 1;
                    }
                    if next_col < width {
                        if input[prev_row][next_col] != '.' as u8 {
                            surround_count += 1;
                        }
                    }
                }
                if let Some(prev_col) = prev_col {
                    if input[row][prev_col] != '.' as u8 {
                        surround_count += 1;
                    }
                }
                if next_col < width {
                    if input[row][next_col] != '.' as u8 {
                        surround_count += 1;
                    }
                }
                if next_row < height {
                    if let Some(prev_col) = prev_col {
                        if input[next_row][prev_col] != '.' as u8 {
                            surround_count += 1;
                        }
                    }
                    if input[next_row][col] != '.' as u8 {
                        surround_count += 1;
                    }
                    if next_col < width {
                        if input[next_row][next_col] != '.' as u8 {
                            surround_count += 1;
                        }
                    }
                }
                if surround_count < 4 {
                    input[row][col] = '*' as u8;
                    removed_count += 1;
                }
            }
        }
    }
    for row in input {
        for ch in row {
            *ch = if *ch == '*' as u8 { '.' as u8 } else { *ch }
        }
    }
    removed_count
}

fn main() {
    let args = CliArguments::parse();
    let input = fs::read_to_string(args.input).unwrap();
    let mut input: Vec<Vec<u8>> = input.lines().map(|l| l.bytes().collect()).collect();
    let part1 = remove_rolls(&mut input);
    let mut part2 = 0;
    let mut step = part1;
    while step != 0 {
        part2 += step;
        step = remove_rolls(&mut input);
    }
    println!("Part1: {part1}");
    println!("Part2: {part2}");
}
