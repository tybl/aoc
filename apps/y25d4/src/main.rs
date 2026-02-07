use clap::Parser;
use helpers::CliArguments;
use std::fs;

fn main() {
    let args = CliArguments::parse();
    let input = fs::read_to_string(args.input).unwrap();
    let width = input.find('\n').unwrap();
    let height = input.lines().count();
    println!("width: {width}, height: {height}");
    let mut result = vec![vec![0; width]; height];
    let mut row: i32 = 0;
    let mut col: i32 = 0;
    for ch in input.chars() {
        if ch == '\n' {
            row += 1;
            col = 0;
            continue;
        }
        let prev_row = row - 1;
        let prev_col = col - 1;
        let next_row = row + 1;
        let next_col = col + 1;
        if ch == '@' {
            if 0 <= prev_col {
                if 0 <= prev_row {
                    result[prev_row as usize][prev_col as usize] += 1;
                }
                result[row as usize][prev_col as usize] += 1;
                if next_row < height as i32 {
                    result[next_row as usize][prev_col as usize] += 1;
                }
            }
            if 0 <= prev_row {
                result[prev_row as usize][col as usize] += 1;
            }
            //result[row as usize][col as usize] += 1;
            if next_row < height as i32 {
                result[next_row as usize][col as usize] += 1;
            }
            if next_col < width as i32 {
                if 0 <= prev_row {
                    result[prev_row as usize][next_col as usize] += 1;
                }
                result[row as usize][next_col as usize] += 1;
                if next_row < height as i32 {
                    result[next_row as usize][next_col as usize] += 1;
                }
            }
        }
        if ch == '.' {
            // Nothing to grab here, remove from consideration
            result[row as usize][col as usize] += 10;
        }
        col += 1;
    }
    let answer = result.iter().flatten().filter(|&x| *x < 4).count();
    println!("{answer}");
}
