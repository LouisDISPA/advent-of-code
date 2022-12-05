use std::{collections::HashMap, str::FromStr};

const EXAMPLE: &str = include_str!("../example.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    let example = parse(EXAMPLE);
    let input = parse(INPUT);

    println!("Part 1:");
    let output = solve1(&example.0, &example.1);
    println!("  example: {}", output);
    let output = solve1(&input.0, &input.1);
    println!("  input: {}", output);

    println!("Part 2:");
    let output = solve2(example.0, &example.1);
    println!("  example: {}", output);
    let output = solve2(input.0, &input.1);
    println!("  input: {}", output);
}

fn solve1(crates: &HashMap<usize, Vec<char>>, movements: &[Movement]) -> String {
    let mut crates = crates.clone();
    let mut buffer = Vec::new();
    for movement in movements {
        let from_crate = crates.entry(movement.from).or_default();
        for _ in 0..movement.count {
            buffer.push(from_crate.pop().unwrap());
        }
        let to_crate = crates.entry(movement.to).or_default();
        for c in buffer.drain(..) {
            to_crate.push(c);
        }
    }
    let mut vec: Vec<(usize, char)> = crates
        .into_iter()
        .map(|(i, c)| (i, *c.last().unwrap()))
        .collect();
    vec.sort_by_key(|(i, _)| *i);
    vec.into_iter().map(|(_, c)| c).collect()
}

fn solve2(crates: HashMap<usize, Vec<char>>, movements: &[Movement]) -> String {
    let mut crates = crates.clone();
    let mut buffer = Vec::new();
    for movement in movements {
        let from_crate = crates.entry(movement.from).or_default();
        for _ in 0..movement.count {
            buffer.push(from_crate.pop().unwrap());
        }
        let to_crate = crates.entry(movement.to).or_default();
        for c in buffer.drain(..).rev() {
            to_crate.push(c);
        }
    }
    let mut vec: Vec<(usize, char)> = crates
        .into_iter()
        .map(|(i, c)| (i, *c.last().unwrap()))
        .collect();
    vec.sort_by_key(|(i, _)| *i);
    vec.into_iter().map(|(_, c)| c).collect()
}

#[derive(Debug)]
struct Movement {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Movement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (count, movement) = s.split_once("from").unwrap();
        let count = count[5..].trim().parse().unwrap();

        let (from, to) = movement.split_once("to").unwrap();
        let from = from.trim().parse().unwrap();
        let to = to.trim().parse().unwrap();

        Ok(Self { count, from, to })
    }
}

fn parse(input: &str) -> (HashMap<usize, Vec<char>>, Vec<Movement>) {
    let (crates, movements) = input.split_once("\n\n").unwrap();
    let crates = crates
        .lines()
        .rev()
        .skip(1)
        .fold(HashMap::new(), |mut acc, line| {
            for (i, letter) in line.chars().skip(1).step_by(4).enumerate() {
                if letter != ' ' {
                    let entry = acc.entry(i + 1).or_insert_with(Vec::new);
                    entry.push(letter);
                }
            }
            acc
        });
    let movements = movements.lines().flat_map(str::parse).collect();
    (crates, movements)
}
