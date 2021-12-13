#[cfg(test)]
mod tests;

mod fold;
mod point;

use std::collections::HashSet;

use fold::{Axis, Fold};
use point::Point;

type Input = (HashSet<Point>, Vec<Fold>);

pub fn parse_input(text: &str) -> Input {
    let (points, folds) = text.split_once("\n\n").unwrap();

    let points: HashSet<Point> = points.lines().map(|s| s.trim().parse().unwrap()).collect();
    let folds: Vec<Fold> = folds.lines().map(|s| s.trim().parse().unwrap()).collect();

    (points, folds)
}

pub fn solve_part1((mut points, folds): Input) -> usize {
    let fold = &folds[0];
    // print_boad(&points);
    points = points
        .into_iter()
        .map(|point| apply_fold(point, fold))
        .collect();
    // print_boad(&points);

    points.len()
}

pub fn solve_part2((mut points, folds): Input) -> usize {
    for fold in folds {
        points = points
            .into_iter()
            .map(|point| apply_fold(point, &fold))
            .collect()
    }
    print_boad(&points);
    points.len()
}

fn apply_fold(mut point: Point, fold: &Fold) -> Point {
    let value = match fold.axis {
        Axis::X => &mut point.x,
        Axis::Y => &mut point.y,
    };
    let new_len = fold.position;
    if *value > new_len {
        *value = 2 * new_len - *value;
    }
    point
}

fn print_boad(points: &HashSet<Point>) {
    let x = points.iter().map(|p| p.x).max().unwrap();
    let y = points.iter().map(|p| p.y).max().unwrap();
    println!("-----");
    for y in 0..=y {
        let line: String = (0..=x)
            .map(|x| {
                if points.contains(&Point { x, y }) {
                    '#'
                } else {
                    '.'
                }
            })
            .collect();
        println!("{}", line);
    }
    println!("-----");
}
