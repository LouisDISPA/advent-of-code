use std::{mem, str::FromStr};

use anyhow::{anyhow, bail};

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
    println!("\n  example: {}", output);
    let output = solve2(INPUT);
    println!("  input: {}", output);
}

fn solve1(input: &str) -> usize {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(|s| s.parse().unwrap()).collect();

    for _ in 0..20 {
        for index in 0..monkeys.len() {
            let items = mem::take(&mut monkeys[index].items);
            for item in items {
                monkeys[index].inspections += 1;
                let item = match monkeys[index].operation {
                    Operation::Add(value) => item + value,
                    Operation::Multiply(value) => item * value,
                    Operation::Power => item * item,
                };
                let item = item / 3;

                let test = &monkeys[index].test;
                let throw_to = if item % test.divisible == 0 {
                    test.if_true
                } else {
                    test.if_false
                };

                monkeys[throw_to].items.push(item);
            }
        }
    }
    monkeys.sort_by_key(|m| m.inspections);
    let len = monkeys.len();
    monkeys[len - 1].inspections * monkeys[len - 2].inspections
}

fn solve2(input: &str) -> usize {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(|s| s.parse().unwrap()).collect();

    let anneau = monkeys.iter().map(|m| m.test.divisible).product::<usize>();

    for _ in 0..10000 {
        for index in 0..monkeys.len() {
            let items = mem::take(&mut monkeys[index].items);
            for item in items {
                monkeys[index].inspections += 1;
                let item = match monkeys[index].operation {
                    Operation::Add(value) => item + value,
                    Operation::Multiply(value) => item * value,
                    Operation::Power => item * item,
                };

                let item = item % anneau;

                let test = &monkeys[index].test;
                let throw_to = if item % test.divisible == 0 {
                    test.if_true
                } else {
                    test.if_false
                };

                monkeys[throw_to].items.push(item);
            }
        }
    }
    monkeys.sort_by_key(|m| m.inspections);
    let len = monkeys.len();
    monkeys[len - 1].inspections * monkeys[len - 2].inspections
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test: Test,
    inspections: usize,
}

#[derive(Debug)]
enum Operation {
    Add(usize),
    Multiply(usize),
    Power,
}

#[derive(Debug)]
struct Test {
    divisible: usize,
    if_true: usize,
    if_false: usize,
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        lines.next().ok_or_else(|| anyhow!("no id"))?;

        let items = lines.next().ok_or_else(|| anyhow!("no items"))?;
        let items = items[18..]
            .split(',')
            .map(|s| s.trim().parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;

        let operation = lines.next().ok_or_else(|| anyhow!("no operation"))?;
        let operation = match operation[23..].split_once(' ') {
            Some(("*", "old")) => Operation::Power,
            Some(("+", value)) => Operation::Add(value.parse::<usize>()?),
            Some(("*", value)) => Operation::Multiply(value.parse::<usize>()?),
            operation => bail!("invalid operation: {:?}", operation),
        };

        let divisible = lines.next().ok_or_else(|| anyhow!("no test"))?;
        let divisible = divisible[21..].parse::<usize>()?;

        let if_true = lines.next().ok_or_else(|| anyhow!("no test"))?;
        let if_true = if_true[29..].parse::<usize>()?;

        let if_false = lines.next().ok_or_else(|| anyhow!("no test"))?;
        let if_false = if_false[30..].parse::<usize>()?;

        Ok(Monkey {
            items,
            operation,
            test: Test {
                divisible,
                if_true,
                if_false,
            },
            inspections: 0,
        })
    }
}
