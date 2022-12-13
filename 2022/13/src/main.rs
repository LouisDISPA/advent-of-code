use std::{cmp::Ordering, fmt::Display, str::FromStr};

use serde::Deserialize;

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
    let mut total = 0;
    for (id, pair) in input
        .split("\n\n")
        .map(|pair| pair.parse::<Pair>().unwrap())
        .enumerate()
    {
        if pair.0 < pair.1 {
            total += id + 1;
        }
    }
    total
}

fn solve2(input: &str) -> usize {
    let mut signals: Vec<Signal> = input.lines().flat_map(serde_json::from_str).collect();
    signals.sort();

    let signal_0 = Signal::List(vec![Signal::List(vec![Signal::Value(2)])]);
    let signal_1 = Signal::List(vec![Signal::List(vec![Signal::Value(6)])]);

    let (Ok(mut index_0) | Err(mut index_0)) = signals.binary_search(&signal_0);
    let (Ok(mut index_1) | Err(mut index_1)) = signals.binary_search(&signal_1);

    if index_0 < index_1 {
        index_1 += 1;
    } else {
        index_0 += 1;
    }

    (index_0 + 1) * (index_1 + 1)
}

impl Eq for Signal {}

impl PartialEq for Signal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Signal::Value(left), Signal::Value(right)) => left == right,
            (Signal::List(left), Signal::List(right)) => left == right,
            (left, Signal::List(right)) => Some(left) == right.first() && right.len() == 1,
            (Signal::List(left), right) => left.first() == Some(right) && left.len() == 1,
        }
    }
}

impl Ord for Signal {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // println!("-compare- {} {}", self, other);
        match (self, other) {
            (Signal::Value(left), Signal::Value(right)) => PartialOrd::partial_cmp(left, right),
            (Signal::List(left), Signal::List(right)) => {
                for (left, right) in left.iter().zip(right.iter()) {
                    let result = PartialOrd::partial_cmp(left, right).unwrap();
                    if result != Ordering::Equal {
                        return Some(result);
                    }
                }
                PartialOrd::partial_cmp(&left.len(), &right.len())
            }
            (left, Signal::List(right)) => {
                if let Some(right) = right.first() {
                    if left != right {
                        return PartialOrd::partial_cmp(left, right);
                    }
                }
                PartialOrd::partial_cmp(&1, &right.len())
            }
            (Signal::List(left), right) => {
                if let Some(left) = left.first() {
                    if left != right {
                        return PartialOrd::partial_cmp(left, right);
                    }
                }
                PartialOrd::partial_cmp(&left.len(), &1)
            }
        }
    }
}

#[derive(Debug)]
struct Pair(Signal, Signal);

impl FromStr for Pair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut signals = s.lines();
        let signal = signals.next().unwrap().parse::<Signal>().unwrap();
        let signal2 = signals.next().unwrap().parse::<Signal>().unwrap();
        Ok(Pair(signal, signal2))
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum Signal {
    Value(usize),
    List(Vec<Signal>),
}

impl FromStr for Signal {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(value) = s.parse::<usize>() {
            return Ok(Signal::Value(value));
        }
        let s = &s[1..s.len() - 1];
        let mut signals = vec![];

        let mut depth = 0;
        let mut start = 0;
        for (end, c) in s.chars().enumerate() {
            if c == '[' {
                depth += 1;
            } else if c == ']' {
                depth -= 1;
            } else if c == ',' && depth == 0 {
                let signal = s[start..end].parse::<Signal>().unwrap();
                signals.push(signal);
                start = end + 1;
                continue;
            }
        }

        if start < s.len() {
            let signal = s[start..].parse::<Signal>().unwrap();
            signals.push(signal);
        }

        Ok(Signal::List(signals))
    }
}

impl Display for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Signal::Value(value) => write!(f, "{}", value),
            Signal::List(signals) => {
                write!(f, "[")?;
                for (i, signal) in signals.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", signal)?;
                }
                write!(f, "]")
            }
        }
    }
}
