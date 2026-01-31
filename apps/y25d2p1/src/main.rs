use clap::Parser;
use helpers::CliArguments;
use std::{fs, ops::RangeInclusive};

fn to_range(input: &str) -> RangeInclusive<i64> {
    let extent: Vec<i64> = input
        .split('-')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    RangeInclusive::new(*extent.first().unwrap(), *extent.last().unwrap())
}

fn is_invalid(x: i64) -> bool {
    let mut s = x.to_string();
    let len = s.len();
    if len.is_multiple_of(2) {
        let t = s.split_off(len / 2);
        s == t
    } else {
        false
    }
}

fn main() {
    let args = CliArguments::parse();
    let input = fs::read_to_string(args.input).unwrap();
    let sum = input
        .trim()
        .split(',')
        .flat_map(to_range)
        .filter(|x| is_invalid(*x))
        .sum::<i64>();
    println!("{sum}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn convert_single_range() {
        let input = "11-22";
        let output = RangeInclusive::new(11, 22);
        assert!(output == to_range(input));
    }

    #[test]
    fn find_invalid_ids() {
        assert!(is_invalid(22));
        assert!(!is_invalid(23));
        assert!(!is_invalid(233));
        assert!(is_invalid(2323));
    }
}
