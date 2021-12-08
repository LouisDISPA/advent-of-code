use std::env;
use std::fs::read_to_string;




fn main() {
    let mut args = env::args();
    args.next();
    let filepath = args.next().expect("Missing input file argument");
    let file = read_to_string(&filepath).expect("File not found");

    let count = file.split('\n').map(|s| s.split_once('|').unwrap().1).flat_map(|s| s.split(' ')).filter(|s| matches!(s.len(), 2 | 3 | 4 | 7)).count();
    println!("{}", count)

}
