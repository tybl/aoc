use clap::Parser;
use helpers::CliArguments;
use std::fs;

const DIAL_MAX: i32 = 100;
const INIT_DIAL: i32 = 50;

fn count_zeros(init: i32, input: &str) -> u32 {
    let mut count = 0;
    let mut dial = init;
    let input = input.lines();
    for line in input {
        let mut line = line.chars();
        match line.next() {
            Some('L') => {
                let mut temp = dial;
                let num = line.collect::<String>().parse::<i32>().unwrap();
                for _ in 0..num {
                    temp = (temp - 1).rem_euclid(DIAL_MAX);
                    if 0 == temp {
                        count += 1;
                    }
                }
                println!("{dial:3} - {num:2} -> {temp:3}");
                dial = temp;
            }
            Some('R') => {
                let num = line.collect::<String>().parse::<i32>().unwrap();
                let temp = dial + num;
                let rem = temp.rem_euclid(DIAL_MAX);
                let div = temp.div_euclid(DIAL_MAX);
                println!("{dial:3} + {num:2} -> {temp:3} -> {div:2} * {DIAL_MAX} + {rem:2}");
                dial = rem;
                count += div.abs_diff(0);
            }
            Some(_) => panic!("Unknown instruction"),
            None => {}
        }
    }
    count
}

fn main() {
    let args = CliArguments::parse();
    let input = fs::read_to_string(args.input).unwrap();
    let password = count_zeros(INIT_DIAL, &input);
    println!("{password}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_step_01() {
        assert!(1 == count_zeros(INIT_DIAL, "L68"));
    }

    #[test]
    fn example_step_02() {
        assert!(0 == count_zeros(82, "L30"));
    }

    #[test]
    fn example_step_03() {
        assert!(1 == count_zeros(52, "R48"));
    }

    #[test]
    fn example_step_04() {
        assert!(0 == count_zeros(0, "L5")); //
    }

    #[test]
    fn example_step_05() {
        assert!(1 == count_zeros(95, "R60"));
    }

    #[test]
    fn example_step_06() {
        assert!(1 == count_zeros(55, "L55")); //
    }

    #[test]
    fn rotr50() {
        assert!(1 == count_zeros(INIT_DIAL, "R50"));
    }

    #[test]
    fn rotl50() {
        assert!(1 == count_zeros(INIT_DIAL, "L50"));
    }

    #[test]
    fn rotl50l100() {
        assert!(2 == count_zeros(INIT_DIAL, "L50\nL100"));
    }

    #[test]
    fn rotl250() {
        assert!(3 == count_zeros(INIT_DIAL, "L250"));
    }
}
