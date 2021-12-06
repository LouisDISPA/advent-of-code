use std::env;
use std::fs::read_to_string;

fn main() {
    let mut args = env::args();
    args.next();
    let filepath = args.next().expect("Missing input file argument");
    let days = args
        .next()
        .expect("Missing input file argument")
        .parse()
        .expect("Days argument parsing error: not a number");

    let file = read_to_string(&filepath).expect("File not found");

    let mut fish_count = [0u64; 9];
    for fish in file.split(',').map(str::trim) {
        let size: usize = fish.parse().unwrap();
        fish_count[size] += 1;
    }

    for day in (0..9).cycle().take(days) {
        let zero_fish = fish_count[day];
        fish_count[(day + 7) % 9] += zero_fish;
        println!("day {} -> {:?}", day, fish_count)
    }
    println!("{}", fish_count.iter().sum::<u64>())
}
