use std::env;
use std::fs::read_to_string;
use std::collections::HashSet;
use std::str::FromStr;
use std::convert::TryFrom;
use std::ops::Add;
use std::ops::Sub;

fn main() {
    let mut args = env::args();
    args.next();
    args.next();
    let filepath = args.next().expect("Missing input file argument");
    let file = read_to_string(&filepath).expect("File not found");

    let mut sum = 0;

    for line in file.split('\n') {
        let (all, digits) = line.split_once('|').unwrap();
        let all: Vec<Digit> = all.split(' ').map(str::trim).filter(|s| !s.is_empty()).filter_map(|s| s.parse().ok()).collect();
        let digits: Vec<Digit> = digits.split(' ').map(str::trim).filter(|s| !s.is_empty()).filter_map(|s| s.parse().ok()).collect();

        let one = all.iter().find(|d| d.segments.len() == 2).unwrap();
        let seven = all.iter().find(|d| d.segments.len() == 3).unwrap();
        let four = all.iter().find(|d| d.segments.len() == 4).unwrap();
        let eight = all.iter().find(|d| d.segments.len() == 7).unwrap();

        let five_segments: Vec<&Digit> = all.iter().filter(|d| d.segments.len() == 5).collect();
        
        let three = if &(five_segments[0] + five_segments[1]) == eight {
            five_segments[2]
        } else if &(five_segments[0] + five_segments[2]) == eight {
            five_segments[1]
        } else {
            five_segments[0]
        };
        
        
        let nine = three + four;
        
        let five_segments: Vec<&Digit> = five_segments.into_iter().filter(|d| d != &three).collect();
        let (two, five) = if (five_segments[0] + three) == nine {
            (five_segments[1], five_segments[0])
        } else {
            (five_segments[0], five_segments[1])
        };

        let six = &(eight - one) + five;

        let mut number = 0;

        for (index, digit) in digits.iter().rev().enumerate() {
            let digit = match digit {
                x if one == x => 1,
                x if two == x => 2,
                x if three == x => 3,
                x if four == x => 4,
                x if five == x => 5,
                x if &six == x => 6,
                x if seven == x => 7,
                x if eight == x => 8,
                x if &nine == x => 9,
                _ => 0
            };
            number += 10_usize.pow(index as u32) * digit;
        }

        println!("{}", number);
        sum += number;
    }
    println!("sum: {}", sum);

}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Segment { A, B, C, D, E, F, G }

impl TryFrom<char> for Segment {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c.to_ascii_lowercase() {
            'a' => Ok(Segment::A),
            'b' => Ok(Segment::B),
            'c' => Ok(Segment::C),
            'd' => Ok(Segment::D),
            'e' => Ok(Segment::E),
            'f' => Ok(Segment::F),
            'g' => Ok(Segment::G),
            _ => Err(format!("Segment name unknown: '{}'", c)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Digit {
    segments: HashSet<Segment>
}

impl FromStr for Digit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let segments: Result<_, _> = s.chars().map(Segment::try_from).collect();
        segments.map(|segments| Self { segments })
    }

}

impl<'a> Add<&'a Digit> for &'a Digit {
    type Output = Digit;
    fn add(self, rhs: &'a Digit) -> Self::Output {
        let mut res = self.clone();
        for segment in &rhs.segments {
            res.segments.insert(*segment);
        }
        res
    }
}

impl<'a> Sub<&'a Digit> for &'a Digit {
    type Output = Digit;
    fn sub(self, rhs: &'a Digit) -> Self::Output {
        let mut res = self.clone();
        for segment in &rhs.segments {
            res.segments.remove(segment);
        }
        res
    }
}