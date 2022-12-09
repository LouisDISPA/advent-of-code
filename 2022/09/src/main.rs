use std::{collections::HashSet, str::FromStr};

const EXAMPLE: &str = include_str!("../example.txt");
const EXAMPLE2: &str = include_str!("../example2.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part 1:");
    let output = solve1(EXAMPLE);
    println!("  example: {}", output);
    let output = solve1(INPUT);
    println!("  input: {}", output);

    println!("Part 2:");
    let output = solve2(EXAMPLE2);
    println!("  example: {}", output);
    let output = solve2(INPUT);
    println!("  input: {}", output);
}

fn solve1(input: &str) -> usize {
    let mut visited = HashSet::new();
    let mut head = (0isize, 0isize);
    let mut tail = (0, 0);
    visited.insert(tail);

    for line in input.lines().flat_map(str::parse::<Move>) {
        for _ in 0..line.count {
            // Update the head
            match line.direction {
                Direction::Up => head.1 += 1,
                Direction::Down => head.1 -= 1,
                Direction::Left => head.0 -= 1,
                Direction::Right => head.0 += 1,
            }
            // Update the tail if it's to far from the head
            if (head.0 - tail.0).abs() > 1 {
                tail.0 = tail.0 + (head.0 - tail.0).signum();
                if (head.1 - tail.1).abs() >= 1 {
                    tail.1 = tail.1 + (head.1 - tail.1).signum();
                }
            } else if (head.1 - tail.1).abs() > 1 {
                tail.1 = tail.1 + (head.1 - tail.1).signum();
                if (head.0 - tail.0).abs() >= 1 {
                    tail.0 = tail.0 + (head.0 - tail.0).signum();
                }
            }
            visited.insert(tail);
        }
    }

    visited.len()
}

fn solve2(input: &str) -> usize {
    let mut visited = HashSet::new();
    let mut tail = [(0_isize, 0_isize); 10];
    visited.insert((0, 0));

    for movement in input.lines().flat_map(str::parse::<Move>) {
        for _ in 0..movement.count {
            let head = &mut tail[0];
            // Update the head
            match movement.direction {
                Direction::Up => head.1 += 1,
                Direction::Down => head.1 -= 1,
                Direction::Left => head.0 -= 1,
                Direction::Right => head.0 += 1,
            }

            // Update the tail if it's to far from the head
            for i in 1..10 {
                let previous = tail[i - 1];
                let current = &mut tail[i];

                let delta_x = previous.0 - current.0;
                let delta_y = previous.1 - current.1;

                if delta_x.abs() > 1 || delta_y.abs() > 1 {
                    current.0 += delta_x.signum();
                    current.1 += delta_y.signum();
                }
            }
            visited.insert(tail[9]);
        }
    }
    visited.len()
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    count: usize,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((direction, count)) = s.split_once(' ') else {
            return Err(());
        };
        let Ok(count) = count.parse() else {
            return Err(());
        };
        let direction = match direction {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return Err(()),
        };
        Ok(Move { direction, count })
    }
}
