use clap::Parser;
use std::fs;

/// Command line parameters
#[derive(Parser)]
pub struct CliArguments {
    #[arg(short, long, default_value = "input/0.txt")]
    pub input: String,
}

fn main() {
    let mut password = 0;
    let mut dial = 50;
    println!("The dial starts by pointing at {dial}");
    let args = CliArguments::parse();
    let input = fs::read_to_string(args.input).unwrap();
    let input = input.split("\n");
    for line in input {
        let mut line = line.chars();
        match line.next() {
            Some('L') => {
                let num = line.collect::<String>().parse::<i32>().unwrap();
                let mut notice = String::from(".");
                if dial - num < 0 {
                    password += 1;
                    notice = String::from("; during this rotation, it points at 0");
                }
                dial = (100 + dial - num) % 100;
                println!("The dial is rotated L{num} to point at {dial}{notice}");
            }
            Some('R') => {
                let num = line.collect::<String>().parse::<i32>().unwrap();
                let mut notice = String::from(".");
                if dial + num > 100 {
                    password += 1;
                    notice = String::from("; during this rotation, it points at 0");
                }
                dial = (dial + num) % 100;
                println!("The dial is rotated R{num} to point at {dial}{notice}");
            }
            Some(_) => panic!("Unknown instruction"),
            None => {}
        }
        //if 0 == dial {
        //    password += 1;
        //}
    }
    println!("{password}");
}
