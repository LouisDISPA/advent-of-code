#[cfg(test)]
mod tests;

mod snailfish;

use snailfish::Snailfish;

type Input = Vec<Snailfish>;

pub fn parse_input(input: &str) -> Input {
    input.lines().map(str::parse).map(Result::unwrap).collect()
}

pub fn solve_part1(snails: Input) -> u32 {
    for snail in &snails {
        println!("{} -> {}", snail, snail.magnitude())
    }

    let snail = snails
        .into_iter()
        .reduce(|a, b| {
            println!("{}", a);
            println!("+ {}", b);
            let mut sum = a + b;
            sum.reduce();
            println!("= {}", sum);
            sum
        })
        .unwrap();

    snail.magnitude()
}

pub fn solve_part2(snails: Input) -> u32 {
    let mut max = 0;

    for (i, a) in snails.iter().enumerate() {
        for (j, b) in snails.iter().enumerate() {
            if i != j {
                let mut sum = a.clone() + b.clone();
                sum.reduce();
                let magnitude = sum.magnitude();
                if magnitude > max {
                    max = magnitude;
                }
            }
        }
    }

    max
}
