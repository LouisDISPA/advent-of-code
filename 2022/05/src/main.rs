use std::str::FromStr;

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
    let output = solve2(&example.0, &example.1);
    println!("  example: {}", output);
    let output = solve2(&input.0, &input.1);
    println!("  input: {}", output);
}

fn solve1(crates: &[Vec<char>], movements: &[Movement]) -> String {
    let mut crates = crates.to_owned();
    let mut buffer = Vec::new();
    for movement in movements {
        let from_crate = &mut crates[movement.from];
        for _ in 0..movement.count {
            buffer.push(from_crate.swap_remove(from_crate.len() - 1));
        }
        let to_crate = &mut crates[movement.to];
        for c in buffer.drain(..) {
            to_crate.push(c);
        }
    }
    crates.into_iter().flat_map(|v| v.last().copied()).collect()
}

fn solve2(crates: &[Vec<char>], movements: &[Movement]) -> String {
    let mut crates = crates.to_owned();
    let mut buffer = Vec::new();
    for movement in movements {
        let from_crate = &mut crates[movement.from];
        for _ in 0..movement.count {
            buffer.push(from_crate.swap_remove(from_crate.len() - 1));
        }
        let to_crate = &mut crates[movement.to];
        for c in buffer.drain(..).rev() {
            to_crate.push(c);
        }
    }
    crates.into_iter().flat_map(|v| v.last().copied()).collect()
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
        let from = from.trim().parse::<usize>().unwrap() - 1;
        let to = to.trim().parse::<usize>().unwrap() - 1;

        Ok(Self { count, from, to })
    }
}

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<Movement>) {
    let (crates, movements) = input.split_once("\n\n").unwrap();
    let crates = crates
        .lines()
        .rev()
        .skip(1)
        .fold(Vec::new(), |mut acc, line| {
            for (i, letter) in line.chars().skip(1).step_by(4).enumerate() {
                if letter != ' ' {
                    if acc.len() <= i {
                        acc.resize(i + 1, Vec::new());
                    }
                    acc[i].push(letter);
                }
            }
            acc
        });
    let movements = movements.lines().flat_map(str::parse).collect();
    (crates, movements)
}
