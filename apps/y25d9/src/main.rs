#![warn(clippy::pedantic)]
use clap::Parser;
use helpers::CliArguments;
use std::fs;

fn main() {
    let args = CliArguments::parse();
    let input = fs::read_to_string(args.input).unwrap();
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
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
        let x_i = tiles[i].first().unwrap();
        let y_i = tiles[i].last().unwrap();
        for j in (i + 1)..tiles.len() {
            let x_j = tiles[j].first().unwrap();
            let y_j = tiles[j].last().unwrap();
            let a = (x_i.max(x_j) - x_i.min(x_j) + 1) * (y_i.max(y_j) - y_i.min(y_j) + 1);
            max_area = max_area.max(a);
        }
    }
    max_area
}

fn part_two(input: &str) -> isize {
    let tiles = input
        .lines()
        .map(|l| {
            let mut p = l.split(',').map(|w| w.parse::<isize>().unwrap());
            (p.next().unwrap(), p.next().unwrap())
        })
        .collect::<Vec<(isize, isize)>>();
    let mut max_area = 0;
    for i in 0..tiles.len() {
        let x_i = tiles[i].0;
        let y_i = tiles[i].1;
        for j in (i + 1)..tiles.len() {
            let x_j = tiles[j].0;
            let y_j = tiles[j].1;
            let x_min = x_i.min(x_j);
            let y_min = y_i.min(y_j);
            let x_max = x_i.max(x_j);
            let y_max = y_i.max(y_j);
            if !tiles.iter().any(|p| {
                let x = p.0;
                let y = p.1;
                x_min < x && x < x_max && y_min < y && y < y_max
            }) {
                if is_point_contained_by(&(x_i, y_j), &tiles)
                    && is_point_contained_by(&(x_j, y_i), &tiles)
                {
                    if !does_poly_line_cross_rect(&tiles, (x_min, y_min, x_max, y_max)) {
                        let a = (x_max - x_min + 1) * (y_max - y_min + 1);
                        if a > max_area {
                            println!("({x_i},{y_i}) - ({x_j},{y_j}) = {a}");
                            max_area = a;
                        }
                    }
                }
            }
        }
    }
    max_area
}

fn is_point_contained_by((x, y): &(isize, isize), polygon: &Vec<(isize, isize)>) -> bool {
    if 0 == polygon.len() {
        return false;
    }
    let mut result = false;
    let mut j = polygon.len() - 1;
    for i in 0..polygon.len() {
        // If the polygon line segment crosses line O, i.e. if it starts above and ends below, or starts below and ends above.
        if (polygon[i].1 < *y && polygon[j].1 >= *y) || (polygon[j].1 < *y && polygon[i].1 >= *y) {
            // Calculate the X coordinate where the polygon line segment crosses line O, then test if that is to the left of target point.
            if polygon[i].0
                + (y - polygon[i].1) / (polygon[j].1 - polygon[i].1) * (polygon[j].0 - polygon[i].0)
                < *x
            {
                result = !result;
            }
        } else if *x == polygon[i].0 && *y == polygon[i].1 {
            return true;
        }
        j = i;
    }
    result
}

fn does_poly_line_cross_rect(
    polygon: &Vec<(isize, isize)>,
    (x_min, y_min, x_max, y_max): (isize, isize, isize, isize),
) -> bool {
    let (mut x_j, mut y_j) = *polygon.last().unwrap();
    for i in 0..polygon.len() {
        let x_i = polygon[i].0;
        let y_i = polygon[i].1;
        if x_i == x_j
            && x_min < x_i
            && x_i < x_max
            && y_i.min(y_j) <= y_min
            && y_max <= y_i.max(y_j)
        {
            //println!("({x_i},{y_i}) - ({x_j},{y_j}) crosses Rect({x_min},{y_min},{x_max},{y_max})");
            return true;
        }
        if y_i == y_j
            && y_min < y_i
            && y_i < y_max
            && x_i.min(x_j) <= x_min
            && x_max <= x_i.max(x_j)
        {
            //println!("({x_i},{y_i}) - ({x_j},{y_j}) crosses Rect({x_min},{y_min},{x_max},{y_max})");
            return true;
        }
        x_j = x_i;
        y_j = y_i;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_poly() {
        assert!(!is_point_contained_by(&(1, 1), &vec![]));
    }

    #[test]
    fn center_of_square() {
        assert!(is_point_contained_by(
            &(2, 2),
            &vec![(1, 1), (3, 1), (3, 3), (1, 3)]
        ));
    }

    #[test]
    fn points_of_poly_are_inside() {
        assert!(is_point_contained_by(
            &(1, 1),
            &vec![(1, 1), (2, 1), (2, 2), (1, 2)]
        ));
        assert!(is_point_contained_by(
            &(2, 1),
            &vec![(1, 1), (2, 1), (2, 2), (1, 2)]
        ));
        assert!(is_point_contained_by(
            &(2, 2),
            &vec![(1, 1), (2, 1), (2, 2), (1, 2)]
        ));
        assert!(is_point_contained_by(
            &(1, 2),
            &vec![(1, 1), (2, 1), (2, 2), (1, 2)]
        ));
    }
}
