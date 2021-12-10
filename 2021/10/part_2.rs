use std::env;
use std::fs::read_to_string;

fn main() {
    let mut args = env::args();
    args.next();
    args.next();
    let filepath = args.next().expect("Missing input file argument");
    let file = read_to_string(&filepath).expect("File not found");

    let mut scores = Vec::new();

    for line in file.split('\n') {
        let mut buff = Vec::new();
        let mut corrupted = false;
        // println!("{}", line);
        for current in line.chars() {
            match current {
                '(' | '[' | '{' | '<' => buff.push(current),
                ')' | ']' | '}' | '>' => {
                    let before = buff.pop().unwrap();
                    if before != current.reverse() {
                        corrupted = true;
                        break;
                    }
                },
                _ => panic!("unknown character: {}", current)
            }
        }
        if corrupted {
            // println!("corrupted");
            continue
        }
        // println!("missing {:?}", buff);
        let mut score = 0;
        for c in buff.into_iter().rev() {
            score *= 5;
            score += c.cost();
        }

        scores.push(score);
    }
    scores.sort();
    println!("{}", scores[scores.len() / 2]);
}

trait Brackets {
    fn reverse(&self) -> Self;
    fn cost(&self) -> usize;
}

impl Brackets for char {
    fn reverse(&self) -> Self {
        match self {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            ')' => '(',
            '}' => '{',
            ']' => '[',
            '>' => '<',
            _ => panic!("Unknown character: {}", self)
        }
    }

    fn cost(&self) -> usize {
        match self {
            '(' | ')' => 1,
            '[' | ']' => 2,
            '{' | '}' => 3,
            '<' | '>' => 4,
            _ => 0,
        }
    }
}

