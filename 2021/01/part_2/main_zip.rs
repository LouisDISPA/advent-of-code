use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let input: Vec<isize> = stdin
        .lock()
        .lines()
        .filter_map(Result::ok)
        .filter_map(|line| line.parse().ok())
        .collect();

    let mut result: usize = 0;
    for (a, b) in input.iter().zip(input.iter().skip(3)) {
        if a < b {
            result += 1;
        }
    }

    println!("{}", result)
}
