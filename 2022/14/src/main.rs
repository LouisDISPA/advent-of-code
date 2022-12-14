use std::{
    cmp::{max, min},
    collections::HashSet,
};

const EXAMPLE: &str = include_str!("../example.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part 1:");
    let output = solve1(EXAMPLE);
    println!("  example: {}", output);
    let output = solve1(INPUT);
    println!("  input: {}", output);

    println!("\nPart 2:");
    let output = solve2(EXAMPLE);
    println!("  example: {}", output);
    let output = solve2(INPUT);
    println!("  input: {}", output);
}

fn solve1(input: &str) -> usize {
    let mut used = parse(input);

    // for y in 0..=10 {
    //     for x in 494..=503 {
    //         if used.contains(&(x, y)) {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    let max_y = *used.iter().map(|(_, y)| y).max().unwrap();
    let mut sand = (500, 0);
    let mut total = 0;

    while sand.1 <= max_y {
        // try to go down
        let under = (sand.0, sand.1 + 1);
        if !used.contains(&under) {
            sand = under;
            continue;
        }
        // look for a way to go diagonally left
        let left = (under.0 - 1, under.1);
        if !used.contains(&left) {
            sand = left;
            continue;
        }

        // look for a way to go diagonally right
        let right = (under.0 + 1, under.1);
        if !used.contains(&right) {
            sand = right;
            continue;
        }

        // we're stuck, go up
        if used.insert(sand) {
            sand = (500, 0);
            total += 1;
        } else {
            sand = (sand.0, sand.1 - 1);
        }
    }
    total
}

fn solve2(input: &str) -> usize {
    let mut used = parse(input);

    // for y in 0..=10 {
    //     for x in 494..=503 {
    //         if used.contains(&(x, y)) {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    let max_y = *used.iter().map(|(_, y)| y).max().unwrap() + 2;
    let mut sand = (500, 0);
    let mut total = 0;

    loop {
        // try to go down
        let under = (sand.0, sand.1 + 1);
        if under.1 < max_y {
            if !used.contains(&under) {
                sand = under;
                continue;
            }
            // look for a way to go diagonally left
            let left = (under.0 - 1, under.1);
            if !used.contains(&left) {
                sand = left;
                continue;
            }

            // look for a way to go diagonally right
            let right = (under.0 + 1, under.1);
            if !used.contains(&right) {
                sand = right;
                continue;
            }
        }

        // we're stuck, go up
        if used.insert(sand) {
            sand = (500, 0);
            total += 1;
        } else {
            break;
        }
    }
    total
}

fn parse(input: &str) -> HashSet<(isize, isize)> {
    let mut used = HashSet::new();
    for line in input.lines() {
        let positions: Vec<(isize, isize)> = line
            .split(" -> ")
            .map(|s| {
                let (a, b) = s.split_once(',').unwrap();
                (a.parse().unwrap(), b.parse().unwrap())
            })
            .collect();

        // println!("{:?}", positions);

        positions.windows(2).for_each(|w| {
            let (start_x, start_y) = w[0];
            let (end_x, end_y) = w[1];

            if start_x != end_x {
                for x in min(start_x, end_x)..=max(start_x, end_x) {
                    used.insert((x, start_y));
                }
            } else {
                for y in min(start_y, end_y)..=max(start_y, end_y) {
                    used.insert((start_x, y));
                }
            }
        });
    }
    used
}
