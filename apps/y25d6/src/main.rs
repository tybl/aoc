use clap::Parser;
use helpers::CliArguments;
use std::fs;

/// Part 1

#[derive(Debug)]
enum Ops {
    Add(u64),
    Mul(u64),
    Invalid,
}

fn part_one(input: &str) -> u64 {
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
                Ops::Invalid => {}
            }
        }
    }
    accumulators.iter().fold(0, |acc, x| match x {
        Ops::Add(x) => acc + x,
        Ops::Mul(x) => acc + x,
        Ops::Invalid => panic!("Should not reach this point"),
    })
}

/// Part 2

fn part_two(input: &str) -> u64 {
    let input = input.lines().map(|l| l.as_bytes()).collect::<Vec<&[u8]>>();
    let len = input.first().unwrap().len();
    for row in &input {
        assert_eq!(len, row.len());
    }
    let mut acc = Ops::Invalid;
    let mut total = 0;
    for i in 0..len {
        if input.iter().all(|row| row[i] == ' ' as u8) {
            match acc {
                Ops::Invalid => {}
                Ops::Add(n) => total += n,
                Ops::Mul(n) => total += n,
            }
            acc = Ops::Invalid;
        } else {
            // Do calc
            let mut num = 0;
            for row in &input {
                let c = row[i] as char;
                match c {
                    // Assuming operators are specified in the last row
                    '*' => acc = Ops::Mul(1),
                    '+' => acc = Ops::Add(0),
                    ' ' => {} // Assuming whitespaces can be ignored
                    _ if c.is_digit(10) => {
                        num = num * 10 + c.to_digit(10).unwrap() as u64;
                    }
                    _ => panic!("Unexpected character!"),
                }
            }
            match &mut acc {
                Ops::Invalid => panic!("That was unexpected!"),
                Ops::Add(sum) => *sum += num,
                Ops::Mul(product) => *product *= num,
            }
        }
    }
    match acc {
        Ops::Invalid => {}
        Ops::Add(n) => total += n,
        Ops::Mul(n) => total += n,
    }
    total
}

fn main() {
    let args = CliArguments::parse();
    let input = fs::read_to_string(args.input).unwrap();
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}
