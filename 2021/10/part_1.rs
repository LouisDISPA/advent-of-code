use std::env;
use std::fs::read_to_string;

fn main() {
    let mut args = env::args();
    args.next();
    args.next();
    let filepath = args.next().expect("Missing input file argument");
    let file = read_to_string(&filepath).expect("File not found");

    for line in file.split('\n') {
        let mut buff = Vec::new();
        for current in line.chars() {
            match current {
                '(' | '[' | '{' | '<' => buff.push(current),
                ')' | ']' | '}' | '>' => {
                    let before = buff.pop().unwrap();
                    if before != current.reverse() {
                        break;
                    }
                },
                _ => panic!("unknown character: {}", current)
            }
        }
    }
    println!("{}", sum);
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
            '(' | ')' => 3,
            '[' | ']' => 57,
            '{' | '}' => 1197,
            '<' | '>' => 25137,
            _ => 0,
        }
    }
}

