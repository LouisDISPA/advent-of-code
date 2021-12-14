#![feature(iter_intersperse)]

use std::{collections::HashMap, iter::repeat};

#[cfg(test)]
mod tests;

type Input<'a> = (&'a str, HashMap<&'a str, &'a str>);

pub fn parse_input(text: &str) -> Input {
    let (polymer, rules) = text.split_once("\n\n").unwrap();

    let rules = rules
        .lines()
        .map(|s| s.trim().split_once(" -> ").unwrap())
        .collect();

    (polymer, rules)
}

pub fn solve_part1((polymer, rules): Input) -> usize {
    // println!("{}", polymer);
    let mut polymer = polymer.to_owned();
    for _ in 0..10 {
        let mut inter = Vec::new();
        for pair_start in 0..(polymer.len() - 1) {
            if let Some(element) = rules.get(&polymer[pair_start..(pair_start + 2)]) {
                inter.push(*element);
            } else {
                inter.push("");
            }
        }
        let mut inter = inter.into_iter();
        polymer = polymer
            .chars()
            .map(|c| c.to_string())
            .intersperse_with(|| inter.next().unwrap_or_default().to_string())
            .collect();
        // println!("{}", polymer);
    }
    let mut uniques: HashMap<char, usize> = polymer.chars().zip(repeat(0)).collect();
    for c in polymer.chars() {
        let count = uniques.get_mut(&c).unwrap();
        *count += 1;
    }
    let (max, min) = uniques.into_values().fold((0, usize::MAX), |acc, value| {
        (acc.0.max(value), acc.1.min(value))
    });
    max - min
}

pub fn solve_part2((polymer, rules): Input) -> usize {
    let rules: HashMap<&str, (&str, &str)> = rules
        .iter()
        .map(|(key, value)| {
            let mut rule_1 = key[0..1].to_string();
            rule_1.push_str(value);
            let rule_1 = rules.keys().find(|&&v| v == rule_1).unwrap();

            let mut rule_2 = value.to_string();
            rule_2.push_str(&key[1..2]);
            let rule_2 = rules.keys().find(|&&v| v == rule_2).unwrap();

            (*key, (*rule_1, *rule_2))
        })
        .collect();

    let mut counts: HashMap<&str, usize> = rules.keys().map(|s| *s).zip(repeat(0)).collect();

    for pair_start in 0..(polymer.len() - 1) {
        let pair = &polymer[pair_start..(pair_start + 2)];
        let count = counts.get_mut(pair).unwrap();
        *count += 1;
    }

    for _ in 0..40 {
        for (key, old_count) in counts.clone() {
            let count = counts.get_mut(key).unwrap();
            *count -= old_count;

            let next_state = rules.get(key).unwrap().0;
            let count = counts.get_mut(next_state).unwrap();
            *count += old_count;

            let next_state = rules.get(key).unwrap().1;
            let count = counts.get_mut(next_state).unwrap();
            *count += old_count;
        }
    }

    let mut uniques: HashMap<char, usize> = counts
        .keys()
        .flat_map(|s| s.chars())
        .zip(repeat(0))
        .collect();

    for (key, count) in counts {
        let c = key.chars().next().unwrap();
        let value = uniques.get_mut(&c).unwrap();
        *value += count;
    }

    let c = polymer.chars().last().unwrap();
    let value = uniques.get_mut(&c).unwrap();
    *value += 1;

    let (max, min) = uniques
        .into_values()
        .fold((0, usize::MAX), |(max, min), value| {
            (max.max(value), min.min(value))
        });
    max - min
}
