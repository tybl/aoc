use clap::Parser;
use helpers::CliArguments;
use std::fs;

fn main() {
    let args = CliArguments::parse();
    let input = fs::read_to_string(args.input).unwrap();
    println!("Part one: {}", part_one(&input));
    //println!("Part two: {}", part_two(&input));
}

fn part_one(input: &str) -> usize {
    let tiles = input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|w| w.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    let mut max_area = 0;
    for i in 0..tiles.len() {
        for j in (i + 1)..tiles.len() {
            let a_x = tiles[i].first().unwrap();
            let a_y = tiles[i].last().unwrap();
            let b_x = tiles[j].first().unwrap();
            let b_y = tiles[j].last().unwrap();
            let a = (a_x.max(b_x) - a_x.min(b_x) + 1) * (a_y.max(b_y) - a_y.min(b_y) + 1);
            max_area = max_area.max(a);
        }
    }
    max_area
}
