use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let input: Vec<isize> = stdin
        .lock()
        .lines()
        .filter_map(Result::ok)
        .filter_map(|line| line.parse().ok())
        .collect();

    let result: usize = input
        .windows(2)
        .map(|slice| if slice[0] < slice[1] { 1 } else { 0 })
        .sum();

    println!("{}", result)
}
