use clap::Parser;
use helpers::CliArguments;
use std::fs;

/// Part 1

#[derive(Debug)]
enum Ops {
    Add(u64),
    Mul(u64),
}

fn part_one(input: &str) -> u64 {
    println!("here");
    let mut lines = input
        .lines()
        .map(|l| l.split(' ').filter(|w| !w.is_empty()))
        .rev();
    let mut accumulators = lines
        .next()
        .unwrap()
        .map(|w| {
            if w == "+" {
                Ops::Add(0)
            } else if w == "*" {
                Ops::Mul(1)
            } else {
                panic!("Invalid operator: {w}")
            }
        })
        .collect::<Vec<Ops>>();
    for line in lines {
        for (acc, n) in std::iter::zip(&mut accumulators, line) {
            let n = n.parse::<u64>().unwrap();
            match acc {
                Ops::Add(sum) => *sum += n,
                Ops::Mul(prod) => *prod *= n,
            }
        }
    }
    accumulators.iter().fold(0, |acc, x| match x {
        Ops::Add(x) => acc + x,
        Ops::Mul(x) => acc + x,
    })
}

/// Part 2

fn part_two(_input: &str) -> i64 {
    0
}

fn main() {
    let args = CliArguments::parse();
    let input = fs::read_to_string(args.input).unwrap();
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}
