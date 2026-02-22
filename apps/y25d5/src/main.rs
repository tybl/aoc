use clap::Parser;
use helpers::CliArguments;
use std::fs;
use std::ops::RangeInclusive;

fn main() {
    let args = CliArguments::parse();
    let input = fs::read_to_string(args.input).unwrap();
    let mut input = input.lines();
    let mut ranges = Vec::new();
    let mut count = 0;
    for l in &mut input {
        if l.is_empty() {
            break;
        }
        println!("{l}");

        ranges.push(to_range(l));
    }
    for l in &mut input {
        println!("{l}");
        let n = l.parse::<i64>().unwrap();
        if ranges.iter().any(|r| r.contains(&n)) {
            count += 1;
        }
    }
    println!("count: {count}");
}

fn to_range(input: &str) -> RangeInclusive<i64> {
    let extent: Vec<i64> = input
        .split('-')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    RangeInclusive::new(*extent.first().unwrap(), *extent.last().unwrap())
}
