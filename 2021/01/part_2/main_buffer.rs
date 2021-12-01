use std::io::{self, BufRead};

const BUFFER_SIZE: usize = 3;

fn main() {
    let stdin = io::stdin();
    let mut result: usize = 0;
    {
        let mut iter = stdin
            .lock()
            .lines()
            .filter_map(Result::ok)
            .filter_map(|line| line.parse::<isize>().ok());

        let mut buffer: [isize; BUFFER_SIZE] = Default::default();
        for value in &mut buffer {
            *value = iter.next().unwrap_or_default();
        }

        for (index, val) in (0..BUFFER_SIZE).cycle().zip(iter) {
            if val > buffer[index] {
                result += 1
            }
            buffer[index] = val
        }
    }

    println!("{}", result)
}
