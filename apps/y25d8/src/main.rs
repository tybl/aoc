use clap::Parser;
use helpers::CliArguments;
use nalgebra::Point3;
use ordered_float::NotNan;
use std::fs;

fn main() {
    let args = CliArguments::parse();
    let input = fs::read_to_string(args.input).unwrap();
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &str) -> usize {
    let boxes: Vec<Point3<f64>> = input
        .lines()
        .map(|l| {
            let mut iter = l.split(',').map(|w| w.parse::<f64>().unwrap());
            Point3::new(
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            )
        })
        .collect();
    let mut queue = std::collections::BTreeMap::new();
    for i in 0..boxes.len() {
        for j in (i + 1)..boxes.len() {
            let d = nalgebra::distance(&boxes[i], &boxes[j]);
            queue.insert(NotNan::new(d).unwrap(), (i, j));
        }
    }
    let mut circuits = (0..boxes.len()).collect::<Vec<usize>>();
    for _ in 0..1000 {
        let (_, (a, b)) = queue.pop_first().unwrap();
        let new_id = circuits[a];
        let old_id = circuits[b];
        for c in circuits.iter_mut() {
            if *c == old_id {
                *c = new_id;
            }
        }
    }
    circuits.sort();
    let mut top: Vec<usize> = circuits.chunk_by(|a, b| a == b).map(|s| s.len()).collect();
    top.sort();
    top.pop().unwrap() * top.pop().unwrap() * top.pop().unwrap()
}

fn part_two(input: &str) -> usize {
    let boxes: Vec<Point3<f64>> = input
        .lines()
        .map(|l| {
            let mut iter = l.split(',').map(|w| w.parse::<f64>().unwrap());
            Point3::new(
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            )
        })
        .collect();
    let mut queue = std::collections::BTreeMap::new();
    for i in 0..boxes.len() {
        for j in (i + 1)..boxes.len() {
            let d = nalgebra::distance(&boxes[i], &boxes[j]);
            queue.insert(NotNan::new(d).unwrap(), (i, j));
        }
    }
    let mut circuits = (0..boxes.len()).collect::<Vec<usize>>();
    let mut prev_a = 0;
    let mut prev_b = 0;
    while circuits.iter().min() != circuits.iter().max() {
        let (_, (a, b)) = queue.pop_first().unwrap();
        prev_a = a;
        prev_b = b;
        let new_id = circuits[a];
        let old_id = circuits[b];
        for c in circuits.iter_mut() {
            if *c == old_id {
                *c = new_id;
            }
        }
    }
    (boxes[prev_a].x * boxes[prev_b].x) as usize
}
