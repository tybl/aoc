use clap::Parser;
use helpers::CliArguments;
use std::{cmp, fs};

fn main() {
    let args = CliArguments::parse();
    let input = fs::read_to_string(args.input).unwrap();
    println!("{}", input.lines().map(get_max_bank_power).sum::<u32>());
}

static RADIX: u32 = 10;

fn get_max_bank_power(input: &str) -> u32 {
    input
        .chars()
        .map(|c| c.to_digit(RADIX).unwrap())
        .fold(0, check_next_battery)
}

fn check_next_battery(state: u32, next: u32) -> u32 {
    let tens = state / RADIX;
    let ones = state % RADIX;
    cmp::max(state, cmp::max(tens * RADIX + next, ones * RADIX + next))
}

#[cfg(test)]
mod tests {
    use super::get_max_bank_power;

    #[test]
    fn example_input() {
        assert!(98 == get_max_bank_power("987654321111111"));
        assert!(89 == get_max_bank_power("811111111111119"));
        assert!(78 == get_max_bank_power("234234234234278"));
        assert!(92 == get_max_bank_power("818181911112111"));
    }
}
