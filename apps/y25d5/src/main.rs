use clap::Parser;
use helpers::CliArguments;
use std::fs;
use std::ops::RangeInclusive;

/// Parses a dash-separated range string (e.g. "3-7") into an inclusive range.
fn to_range(input: &str) -> RangeInclusive<i64> {
    let extent: Vec<i64> = input
        .split('-')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    RangeInclusive::new(*extent.first().unwrap(), *extent.last().unwrap())
}

/// Part one: reads a block of ranges (one per line, terminated by a blank line),
/// then counts how many of the remaining lines contain a value that falls
/// within at least one of those ranges.
fn part_one(input: &str) -> usize {
    let mut input = input.lines();

    // Collect ranges from the first section of input (up to the first blank line)
    let ranges = input
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(to_range)
        .collect::<Vec<RangeInclusive<i64>>>();

    // For each value in the second section, check if it falls in any range
    input
        .by_ref()
        .map(|l| {
            ranges
                .iter()
                .any(|r| r.contains(&l.parse::<i64>().unwrap()))
        })
        .filter(|b| *b)
        .count()
}

/// Parses a dash-separated range string into a (start, end) tuple.
fn to_tuple(input: &str) -> (i64, i64) {
    let extent: Vec<i64> = input
        .split('-')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    (*extent.first().unwrap(), *extent.last().unwrap())
}

/// Merges a sorted list of (start, end) ranges, combining any that overlap or touch.
/// Returns a minimal list of non-overlapping ranges covering the same values.
fn merge_ranges(inputs: &Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    assert!(inputs.is_sorted());
    let mut result: Vec<(i64, i64)> = Vec::new();
    for next in inputs {
        if let Some(prev) = result.last_mut()
            // Since inputs are sorted, prev.0 <= next.0 is guaranteed.
            // If the previous range reaches or overlaps the start of the next,
            // merge them.
            && prev.1 >= next.0
        {
            // Extend the previous range _only_ if the next one reaches further
            if prev.1 < next.1 {
                prev.1 = next.1;
            }
        } else {
            // No overlap - start a new range
            result.push(*next);
        }
    }
    result
}

/// Part two: reads ranges from the first section, merges overlapping ones,
/// then sums the total number of integers covered across all merged ranges.
fn part_two(input: &str) -> i64 {
    let mut ingredients = input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(to_tuple)
        .collect::<Vec<(i64, i64)>>();

    ingredients.sort();

    merge_ranges(&ingredients)
        .iter()
        .map(|(a, b)| b - a + 1) // Count of numbers in each range
        .sum()
}

fn main() {
    let args = CliArguments::parse();
    let input = fs::read_to_string(args.input).unwrap();
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}
