use std::env;
use std::fs::read_to_string;
use std::collections::HashMap;

fn average(numbers: &[isize]) -> f32 {
    numbers.iter().sum::<isize>() as f32 / numbers.len() as f32
}

fn median(numbers: &mut [isize]) -> isize {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

fn main() {
    let mut args = env::args();
    args.next();
    let filepath = args.next().expect("Missing input file argument");

    let file = read_to_string(&filepath).expect("File not found");

    let mut values: Vec<isize> = file.split(',').map(str::parse).filter_map(Result::ok).collect();

    let av = average(&values) ;

    println!("{:?}", av);
    println!("{:?}", values.iter().fold(0, |acc, v| {
        let delta = (v - av as isize).abs();
        acc + (delta * (delta + 1) / 2)
    }));
    println!("{:?}", median(&mut values));
}
