use clap::Parser;
use helpers::CliArguments;
use std::fs;

fn main() {
    let args = CliArguments::parse();
    let input = fs::read_to_string(args.input).unwrap();
    println!(
        "{}",
        input
            .lines()
            .map(|x| get_max_bank_power(12, x))
            .sum::<u64>()
    );
}

fn get_max_bank_power(num_batteries: usize, bank: &str) -> u64 {
    let mut stack: Vec<u8> = Vec::new();
    for (i, c) in bank.bytes().rev().enumerate().rev() {
        while (num_batteries <= (stack.len() + i)) && stack.last().is_some_and(|x| x < &c) {
            stack.pop();
        }
        if stack.len() < num_batteries {
            stack.push(c);
        }
    }
    let result = String::from_utf8(stack)
        .unwrap_or("0".to_string())
        .parse()
        .unwrap_or(0);
    result
}

#[cfg(test)]
mod tests {
    use super::get_max_bank_power;

    #[test]
    fn input_equals_output() {
        assert!(9 == get_max_bank_power(1, "9"));
        assert!(987654321111 == get_max_bank_power(12, "987654321111"));
    }

    #[test]
    fn simple() {
        assert!(9 == get_max_bank_power(1, "89"));
        assert!(987654321111 == get_max_bank_power(12, "1987654321111"));
    }

    #[test]
    fn dont_pop_whole_stack() {
        assert!(89 == get_max_bank_power(2, "89"));
    }

    #[test]
    fn example_input() {
        assert!(987654321111 == get_max_bank_power(12, "987654321111111"));
        assert!(811111111119 == get_max_bank_power(12, "811111111111119"));
        assert!(434234234278 == get_max_bank_power(12, "234234234234278"));
        assert!(888911112111 == get_max_bank_power(12, "818181911112111"));
    }
}
