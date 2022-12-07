use std::collections::HashMap;

const EXAMPLE: &str = include_str!("../example.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part 1:");
    let output = solve1(EXAMPLE);
    println!("  example: {}", output);
    let output = solve1(INPUT);
    println!("  input: {}", output);

    println!("Part 2:");
    let output = solve2(EXAMPLE);
    println!("  example: {}", output);
    let output = solve2(INPUT);
    println!("  input: {}", output);
}

fn solve1(input: &str) -> usize {
    build_node_size_map(input)
        .values()
        .filter(|&&v| v <= 100000)
        .sum()
}

fn solve2(input: &str) -> usize {
    let node_size = build_node_size_map(input);
    let space_to_delete = node_size.get("").unwrap() - 40000000;
    node_size
        .values()
        .filter(|&&v| v > space_to_delete)
        .copied()
        .min()
        .unwrap()
}

fn build_node_size_map(input: &str) -> HashMap<String, usize> {
    let mut current_dir = Vec::new();
    let mut node_size = HashMap::new();
    for line in input.lines() {
        if !line.starts_with('$') {
            let (size, name) = line.split_once(' ').unwrap();
            if size.trim() == "dir" {
                continue;
            }
            let size: usize = size.parse().unwrap();
            for i in 0..=current_dir.len() {
                let dir: String = current_dir[..i].iter().copied().collect();
                *node_size.entry(dir).or_insert(0) += size;
            }
            continue;
        }
        match &line[..4] {
            "$ cd" => {
                let dir = line[5..].trim();
                match dir {
                    ".." => {
                        current_dir.pop();
                    }
                    "." => {}
                    "/" => current_dir.clear(),
                    _ => current_dir.push(dir),
                }
            }
            "$ ls" => {}
            _ => panic!("Unknown command: {}", line),
        }
    }
    node_size
}
