use std::array;

const EXAMPLE: &str = include_str!("../example.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part 1:");
    for (i, line) in EXAMPLE.lines().enumerate() {
        let output = solve::<4>(line);
        println!("  example {}: {}", i, output);
    }
    let output = solve::<4>(INPUT);
    println!("  input: {}", output);

    println!("Part 2:");
    for (i, line) in EXAMPLE.lines().enumerate() {
        let output = solve::<14>(line);
        println!("  example {}: {}", i, output);
    }
    let output = solve::<14>(INPUT);
    println!("  input: {}", output);
}

fn solve<const N: usize>(input: &str) -> usize {
    let mut iterator = input.chars().enumerate();
    let mut window: [char; N] = array::from_fn(|_| iterator.next().unwrap().1);

    for (i, c) in iterator {
        let mut same = false;
        'cmp: for (j, first) in window.iter().enumerate() {
            for second in &window[j + 1..] {
                if first == second {
                    same = true;
                    break 'cmp;
                }
            }
        }
        if !same {
            return i;
        }
        window.copy_within(1.., 0);
        window[N - 1] = c;
    }
    0
}
