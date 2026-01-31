use clap::Parser;
use helpers::CliArguments;
use std::fs;

fn main() {
    let mut password = 0;
    let mut dial = 50;
    let args = CliArguments::parse();
    let input = fs::read_to_string(args.input).unwrap();
    let input = input.split("\n");
    for line in input {
        let mut line = line.chars();
        match line.next() {
            Some('L') => {
                let num = line.collect::<String>().parse::<i32>().unwrap();
                dial = (100 + dial - num) % 100;
            }
            Some('R') => {
                let num = line.collect::<String>().parse::<i32>().unwrap();
                dial = (dial + num) % 100;
            }
            Some(_) => panic!("Unknown instruction"),
            None => {}
        }
        if 0 == dial {
            password += 1;
        }
    }
    println!("{password}");
}
