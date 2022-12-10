use std::str::FromStr;

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
    println!("\n  example:\n\n{}\n", output);
    let output = solve2(INPUT);
    println!("  input:\n\n{}", output);
}

fn solve1(input: &str) -> isize {
    let mut total = 0;
    let mut cycle = 0;
    let mut strength = 1;
    for command in input.lines().flat_map(str::parse::<Command>) {
        for _ in 0..command.cycle_count() {
            cycle += 1;
            if cycle % 40 == 20 {
                total += strength * cycle;
            }
        }
        match command {
            Command::Noop => {}
            Command::Addx(x) => strength += x,
        }
    }
    total
}

fn solve2(input: &str) -> String {
    let mut screen = String::new();
    let mut cycle = 0_isize;
    let mut strength = 1;
    for command in input.lines().flat_map(str::parse::<Command>) {
        for _ in 0..command.cycle_count() {
            if (strength - (cycle % 40)).abs() < 2 {
                screen.push('#');
            } else {
                screen.push('.');
            }
            cycle += 1;
            if cycle % 40 == 0 {
                screen.push('\n');
            }
        }
        match command {
            Command::Noop => {}
            Command::Addx(x) => strength += x,
        }
    }
    screen
}

#[derive(Debug)]
enum Command {
    Noop,
    Addx(isize),
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[..4] {
            "noop" => Ok(Command::Noop),
            "addx" => {
                let x = s[5..].parse().map_err(|_| ())?;
                Ok(Command::Addx(x))
            }
            _ => Err(()),
        }
    }
}

impl Command {
    fn cycle_count(&self) -> isize {
        match self {
            Command::Noop => 1,
            Command::Addx(_) => 2,
        }
    }
}
