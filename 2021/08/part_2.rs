use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Sub};
use std::str::FromStr;

fn main() {
    let mut args = env::args();
    args.next();
    args.next();
    let filepath = args.next().expect("Missing input file argument");
    let file = File::open(&filepath).expect("File not found");
    let mut lines = BufReader::new(file).lines();

    let mut sum = 0;

    while let Some(Ok(line)) = lines.next() {
        let (all, digits) = line.split_once('|').unwrap();
        let all: Vec<Digit> = all
            .split(' ')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .filter_map(|s| s.parse().ok())
            .collect();

        let one = *all.iter().find(|d| d.segment_count() == 2).unwrap();
        let seven = *all.iter().find(|d| d.segment_count() == 3).unwrap();
        let four = *all.iter().find(|d| d.segment_count() == 4).unwrap();
        let eight = *all.iter().find(|d| d.segment_count() == 7).unwrap();

        let five_segments: Vec<Digit> =
            all.into_iter().filter(|d| d.segment_count() == 5).collect();

        let three = if (five_segments[0] + five_segments[1]) == eight {
            five_segments[2]
        } else if (five_segments[0] + five_segments[2]) == eight {
            five_segments[1]
        } else {
            five_segments[0]
        };

        let nine = three + four;

        let five_segments: Vec<Digit> = five_segments.into_iter().filter(|d| d != &three).collect();
        let (two, five) = if (five_segments[0] + three) == nine {
            (five_segments[1], five_segments[0])
        } else {
            (five_segments[0], five_segments[1])
        };

        let six = (eight - one) + five;

        let mut number = 0;
        let digits = digits
            .split(' ')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .filter_map(|s| s.parse().ok());

        for (index, digit) in digits.into_iter().rev().enumerate() {
            let digit = match digit {
                x if one == x => 1,
                x if two == x => 2,
                x if three == x => 3,
                x if four == x => 4,
                x if five == x => 5,
                x if six == x => 6,
                x if seven == x => 7,
                x if eight == x => 8,
                x if nine == x => 9,
                _ => 0,
            };
            number += 10_usize.pow(index as u32) * digit;
        }

        println!("{}", number);
        sum += number;
    }
    println!("sum: {}", sum);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Digit {
    segments: [bool; 7],
}

impl FromStr for Digit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut segments = [false; 7];
        for c in s.chars() {
            let index = match c.to_ascii_lowercase() {
                'a' => 0,
                'b' => 1,
                'c' => 2,
                'd' => 3,
                'e' => 4,
                'f' => 5,
                'g' => 6,
                _ => return Err(format!("Segment name unknown: '{}'", c)),
            };
            segments[index] = true;
        }
        Ok(Self { segments })
    }
}

impl Add<Digit> for Digit {
    type Output = Digit;
    fn add(mut self, rhs: Digit) -> Self::Output {
        for (index, segment) in rhs.segments.iter().enumerate() {
            if *segment {
                self.segments[index] = true
            }
        }
        self
    }
}

impl Sub<Digit> for Digit {
    type Output = Digit;
    fn sub(mut self, rhs: Digit) -> Self::Output {
        for (index, segment) in rhs.segments.iter().enumerate() {
            if *segment {
                self.segments[index] = false
            }
        }
        self
    }
}

impl Digit {
    fn segment_count(&self) -> usize {
        self.segments.iter().filter(|sgt| **sgt).count()
    }
}
