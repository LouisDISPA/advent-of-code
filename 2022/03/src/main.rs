#![feature(iter_array_chunks)]

use std::collections::HashSet;

const EXAMPLE: &str = include_str!("../example.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part 1:");
    let output = solve1(EXAMPLE);
    println!("  example: {}", output);
    let output = solve1(INPUT);
    println!("  input: {}", output);

    println!("Part 2:");
    let output = solve2(EXAMPLE);
    println!("  example: {}", output);
    let output = solve2(INPUT);
    println!("  input: {}", output);
}

fn solve1(input: &str) -> u32 {
    let mut total = 0;
    for line in input.lines() {
        let half = line.len() / 2;
        // seperate the string into the first and second halves
        let first: HashSet<char> = line[0..half].chars().collect();
        let second = &line[half..];

        let c = second
            .chars()
            .filter(|c| first.contains(&c))
            .next()
            .unwrap();
        total += eval_character(c);
    }
    total
}

fn solve2(input: &str) -> u32 {
    let mut total = 0;
    for [first, second, third] in input.lines().array_chunks() {
        // seperate the string into the first and second halves
        let first: HashSet<char> = first.chars().collect();
        let second: HashSet<char> = second.chars().filter(|c| first.contains(&c)).collect();

        let c = third
            .chars()
            .filter(|c| second.contains(&c))
            .next()
            .unwrap();
        total += eval_character(c);
    }
    total
}

fn eval_character(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else {
        c as u32 - 'A' as u32 + 27
    }
}
