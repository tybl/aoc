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
        let (x_i, y_i) = tiles[i];
        for j in (i + 1)..tiles.len() {
            let (x_j, y_j) = tiles[j];
            let x_min = x_i.min(x_j);
            let y_min = y_i.min(y_j);
            let x_max = x_i.max(x_j);
            let y_max = y_i.max(y_j);
            if !tiles
                .iter()
                .any(|(x, y)| x_min < *x && *x < x_max && y_min < *y && *y < y_max)
                && is_point_contained_by_orthogonal_polygon((x_i, y_j), &tiles)
                && is_point_contained_by_orthogonal_polygon((x_j, y_i), &tiles)
                && !does_poly_line_cross_rect(&tiles, (x_min, y_min, x_max, y_max))
            {
                let a = (x_max - x_min + 1) * (y_max - y_min + 1);
                max_area = max_area.max(a);
            }
        }
    }
    max_area
}

fn is_point_contained_by_orthogonal_polygon(
    (x, y): (isize, isize),
    polygon: &Vec<(isize, isize)>,
) -> bool {
    if 0 == polygon.len() {
        return false;
    }
    let mut result = false;
    let (mut x_j, mut y_j) = *polygon.last().unwrap();
    for i in 0..polygon.len() {
        let (x_i, y_i) = polygon[i];
        let x_max = x_i.max(x_j);
        let y_min = y_i.min(y_j);

        assert!((x_i == x_j) || (y_i == y_j)); // Either horizontal or vertical line segment

        if y_min <= y && y <= y_i.max(y_j) {
            if x_i.min(x_j) <= x && x <= x_max {
                return true;
            }
            if x_max < x && y_min < y {
                result = !result;
            }
        }
        x_j = x_i;
        y_j = y_i;
    }
    result
}

fn does_poly_line_cross_rect(
    polygon: &Vec<(isize, isize)>,
    (x_min, y_min, x_max, y_max): (isize, isize, isize, isize),
) -> bool {
    let (mut x_j, mut y_j) = *polygon.last().unwrap();
    for i in 0..polygon.len() {
        let (x_i, y_i) = polygon[i];
        if x_i == x_j
            && x_min < x_i
            && x_i < x_max
            && y_i.min(y_j) <= y_min
            && y_max <= y_i.max(y_j)
        {
            return true;
        }
        if y_i == y_j
            && y_min < y_i
            && y_i < y_max
            && x_i.min(x_j) <= x_min
            && x_max <= x_i.max(x_j)
        {
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
        assert!(!is_point_contained_by_orthogonal_polygon((1, 1), &vec![]));
    }

    #[test]
    fn center_of_square() {
        assert!(is_point_contained_by_orthogonal_polygon(
            (2, 2),
            &vec![(1, 1), (3, 1), (3, 3), (1, 3)]
        ));
    }

    #[test]
    fn points_of_poly_are_inside() {
        assert!(is_point_contained_by_orthogonal_polygon(
            (1, 1),
            &vec![(1, 1), (2, 1), (2, 2), (1, 2)]
        ));
        assert!(is_point_contained_by_orthogonal_polygon(
            (2, 1),
            &vec![(1, 1), (2, 1), (2, 2), (1, 2)]
        ));
        assert!(is_point_contained_by_orthogonal_polygon(
            (2, 2),
            &vec![(1, 1), (2, 1), (2, 2), (1, 2)]
        ));
        assert!(is_point_contained_by_orthogonal_polygon(
            (1, 2),
            &vec![(1, 1), (2, 1), (2, 2), (1, 2)]
        ));
    }

    #[test]
    fn points_on_the_line_are_inside() {
        assert!(is_point_contained_by_orthogonal_polygon(
            (2, 3),
            &vec![(1, 1), (3, 1), (3, 3), (1, 3)]
        ));
        assert!(is_point_contained_by_orthogonal_polygon(
            (1, 2),
            &vec![(1, 1), (3, 1), (3, 3), (1, 3)]
        ));
        assert!(is_point_contained_by_orthogonal_polygon(
            (3, 2),
            &vec![(1, 1), (3, 1), (3, 3), (1, 3)]
        ));
        assert!(is_point_contained_by_orthogonal_polygon(
            (2, 1),
            &vec![(1, 1), (3, 1), (3, 3), (1, 3)]
        ));
    }

    // 01234567890123
    //0..............
    //1.......1---2..
    //2.......|...|..
    //3<=5====6=*.|..
    //4..|........|..
    //5..4--------3..
    //6..............

    #[test]
    fn ray_inline_with_segment() {
        assert!(is_point_contained_by_orthogonal_polygon(
            (9, 3),
            &vec![(7, 1), (11, 1), (11, 5), (2, 5), (2, 3), (7, 3)]
        ));
        assert!(!is_point_contained_by_orthogonal_polygon(
            (13, 3),
            &vec![(7, 1), (11, 1), (11, 5), (2, 5), (2, 3), (7, 3)]
        ));
        assert!(!is_point_contained_by_orthogonal_polygon(
            (13, 1),
            &vec![(7, 1), (11, 1), (11, 5), (2, 5), (2, 3), (7, 3)]
        ));
    }
}
