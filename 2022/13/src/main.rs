use std::{fmt::Display, str::FromStr};

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
        // println!("\n== Pair {} ==", id);
        // println!("{:?}", compare(pair.0, pair.1));

        if compare(pair.0, pair.1).unwrap() {
            total += id + 1;
        }
    }
    total
}

fn compare(left: Signal, right: Signal) -> Option<bool> {
    // if matches!((&left, &right), (Signal::Signal(_), Signal::Signal(_))) {
    //     println!("- compare {} vs {}", left, right);
    // }

    match (left, right) {
        (Signal::Value(left), Signal::Value(right)) => {
            // println!("- compare {} vs {}", left, right);
            if left == right {
                None
            } else {
                Some(left < right)
            }
        }
        (Signal::Signal(left), Signal::Signal(right)) => {
            let result = if left.len() == right.len() {
                None
            } else {
                Some(left.len() < right.len())
            };

            for (left, right) in left.into_iter().zip(right.into_iter()) {
                if let Some(result) = compare(left, right) {
                    return Some(result);
                }
            }
            result
        }
        (left, right) => {
            let left = if matches!(left, Signal::Signal(_)) {
                left
            } else {
                Signal::Signal(vec![left])
            };
            let right = if matches!(right, Signal::Signal(_)) {
                right
            } else {
                Signal::Signal(vec![right])
            };
            compare(left, right)
        }
    }
}

fn solve2(input: &str) -> usize {
    let mut signals: Vec<Signal> = vec![];

    for signal in input
        .lines()
        .filter(|line| !line.is_empty())
        .flat_map(str::parse::<Signal>)
    {
        let index = signals
            .iter()
            .position(|signal2| compare(signal.clone(), signal2.clone()).unwrap())
            .unwrap_or_else(|| signals.len());
        signals.insert(index, signal);
    }

    let signal_0 = Signal::Signal(vec![Signal::Signal(vec![Signal::Value(2)])]);
    let signal_1 = Signal::Signal(vec![Signal::Signal(vec![Signal::Value(6)])]);

    let mut index_0 = signals
        .iter()
        .position(|signal| compare(signal_0.clone(), signal.clone()).unwrap())
        .unwrap_or_else(|| signals.len());
    let mut index_1 = signals
        .iter()
        .position(|signal| compare(signal_1.clone(), signal.clone()).unwrap())
        .unwrap_or_else(|| signals.len());

    if index_0 < index_1 {
        index_1 += 1;
    } else {
        index_0 += 1;
    }

    (index_0 + 1) * (index_1 + 1)
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

#[derive(Debug, Clone)]
enum Signal {
    Value(usize),
    Signal(Vec<Signal>),
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

        Ok(Signal::Signal(signals))
    }
}

impl Display for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Signal::Value(value) => write!(f, "{}", value),
            Signal::Signal(signals) => {
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
