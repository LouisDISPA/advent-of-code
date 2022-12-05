use std::num::ParseIntError;
use std::str::FromStr;

const EXAMPLE: &str = include_str!("../example.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    let example = parse(EXAMPLE);
    let input = parse(INPUT);

    println!("Part 1:");
    let output = solve1(&example);
    println!("  example: {}", output);
    let output = solve1(&input);
    println!("  input: {}", output);

    println!("Part 2:");
    let output = solve2(&example);
    println!("  example: {}", output);
    let output = solve2(&input);
    println!("  input: {}", output);
}

fn solve1(input: &[[Assignment; 2]]) -> usize {
    let mut included_count = 0;
    for [first, second] in input.iter() {
        // check if first is included in second
        if first.start >= second.start && first.end <= second.end {
            included_count += 1;
        }
        // check if second is included in first
        else if second.start >= first.start && second.end <= first.end {
            included_count += 1;
        }
    }
    included_count
}

fn solve2(input: &[[Assignment; 2]]) -> usize {
    let mut overlap_count = 0;
    for [first, second] in input.iter() {
        // check if first overlaps with second
        if first.start <= second.start && first.end >= second.start {
            overlap_count += 1;
        }
        // check if second overlaps with first
        else if second.start <= first.start && second.end >= first.start {
            overlap_count += 1;
        }
    }
    overlap_count
}

struct Assignment {
    start: i32,
    end: i32,
}

impl FromStr for Assignment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').unwrap();
        let start = start.parse().unwrap();
        let end = end.parse().unwrap();
        Ok(Self { start, end })
    }
}

fn parse(input: &str) -> Vec<[Assignment; 2]> {
    input
        .lines()
        .map(|line| {
            let (first, second) = line.split_once(",").unwrap();
            let first = first.parse().unwrap();
            let second = second.parse().unwrap();
            [first, second]
        })
        .collect()
}
