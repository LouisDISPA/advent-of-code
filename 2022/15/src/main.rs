use std::{
    cmp::{max, min},
    collections::HashSet,
    str::FromStr,
};

const EXAMPLE: &str = include_str!("../example.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    let example: Vec<Scan> = EXAMPLE.lines().flat_map(str::parse).collect();
    let input: Vec<Scan> = INPUT.lines().flat_map(str::parse).collect();
    println!("Part 1:");
    let output = solve1(&example, 10);
    println!("  example: {}", output);
    // let output = solve1(&input, 2000000);
    // println!("  input: {}", output);

    println!("\nPart 2:");
    let output = solve2_dummy::<20>(&example);
    println!("  example: {} (dummy)", output);
    let output = solve2(&example, 20);
    println!("  example: {}", output);
    let output = solve2(&input, 4000000);
    println!("  input: {}", output);
}

fn solve1(input: &[Scan], line: isize) -> usize {
    let mut scanned: HashSet<isize> = HashSet::new();
    for scan in input {
        let distance =
            (scan.position.0 - scan.detected.0).abs() + (scan.position.1 - scan.detected.1).abs();
        let spread = distance - (scan.position.1 - line).abs();
        if spread >= 0 {
            let start = scan.position.0 - spread;
            let end = scan.position.0 + spread;
            for y in start..=end {
                scanned.insert(y);
            }
        }
    }
    for scan in input {
        if scan.detected.1 == line {
            scanned.remove(&scan.detected.0);
        }
    }
    scanned.len()
}

fn solve2_dummy<const N: usize>(input: &[Scan]) -> usize {
    let mut grid = [[true; N]; N];
    for scan in input {
        let distance =
            (scan.position.0 - scan.detected.0).abs() + (scan.position.1 - scan.detected.1).abs();
        let x_min = scan.position.0 - distance;
        let x_max = scan.position.0 + distance;
        let y_min = scan.position.1 - distance;
        let y_max = scan.position.1 + distance;

        for x in max(x_min, 0)..min(x_max + 1, N as isize) {
            for y in max(y_min, 0)..min(y_max + 1, N as isize) {
                if (scan.position.0 - x).abs() + (scan.position.1 - y).abs() <= distance {
                    grid[x as usize][y as usize] = false;
                }
            }
        }
    }

    // for y in 0..N {
    //     for x in 0..N {
    //         if grid[x][y] {
    //             print!("X");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    for x in 0..N {
        for y in 0..N {
            if grid[x][y] {
                // println!("found: {} {}", x, y);
                return x * 4000000 + y;
            }
        }
    }
    panic!("No solution found");
}

fn solve2(input: &[Scan], size: isize) -> isize {
    for scan in input {
        let distance =
            (scan.position.0 - scan.detected.0).abs() + (scan.position.1 - scan.detected.1).abs();

        let x_pos = scan.position.0;
        let y_pos = scan.position.1;

        let x_min = x_pos - distance - 1;
        let x_max = x_pos + distance + 1;
        let y_min = y_pos - distance - 1;
        let y_max = y_pos + distance + 1;

        let borders = (
            (x_min..=x_pos).zip(y_pos..=y_max),
            (x_pos..=x_max).zip((y_pos..=y_max).rev()),
            (x_pos..=x_max).zip(y_min..=y_pos),
            ((x_min..=x_pos).rev()).zip(y_min..=y_pos),
        );

        for (x, y) in borders.0.chain(borders.1).chain(borders.2).chain(borders.3) {
            if x < 0 || x > size || y < 0 || y > size {
                continue;
            }
            if is_valid(input, x, y) {
                // println!("found: {} {}", x, y);
                return x * 4000000 + y;
            }
        }
    }
    panic!("No solution found");
}

fn is_valid(input: &[Scan], x: isize, y: isize) -> bool {
    for scan in input {
        let distance =
            (scan.position.0 - scan.detected.0).abs() + (scan.position.1 - scan.detected.1).abs();

        if (scan.position.0 - x).abs() + (scan.position.1 - y).abs() <= distance {
            return false;
        }
    }
    true
}

#[derive(Debug)]
struct Scan {
    position: (isize, isize),
    detected: (isize, isize),
}

impl FromStr for Scan {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // example input: "Sensor at x=2, y=18: closest beacon is at x=-2, y=15"

        let mut parts = s.split(' ');
        let x = parts.nth(2).unwrap();
        let x = x.split_once('=').unwrap().1;
        let x = x.trim_matches(',').parse().unwrap();

        let y = parts.next().unwrap();
        let y = y.split_once('=').unwrap().1;
        let y = y.trim_matches(':').parse().unwrap();

        let position = (x, y);

        let x = parts.nth(4).unwrap();
        let x = x.split_once('=').unwrap().1;
        let x = x.trim_matches(',').parse().unwrap();

        let y = parts.next().unwrap();
        let y = y.split_once('=').unwrap().1;
        let y = y.parse().unwrap();

        let detected = (x, y);

        Ok(Scan { position, detected })
    }
}
