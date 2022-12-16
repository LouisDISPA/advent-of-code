use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

const EXAMPLE: &str = include_str!("../example.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    let example: HashMap<String, Valve> = EXAMPLE
        .lines()
        .map(|line| line.parse::<Valve>().unwrap())
        .map(|valve| (valve.id.clone(), valve))
        .collect();

    let input: HashMap<String, Valve> = INPUT
        .lines()
        .map(|line| line.parse::<Valve>().unwrap())
        .map(|valve| (valve.id.clone(), valve))
        .collect();

    println!("Part 1:");
    let output = solve1(&example);
    println!("  example: {}", output);
    let output = solve1(&input);
    println!("  input: {}", output);

    println!("\nPart 2:");
    let output = solve2(EXAMPLE);
    println!("  example: {}", output);
    let output = solve2(INPUT);
    println!("  input: {}", output);
}

fn solve1(input: &HashMap<String, Valve>) -> usize {
    let mut current = input["AA"].id.as_str();
    let mut total = 0;
    let mut minutes = 0;
    let mut visited = HashSet::new();

    while minutes <= 30 {
        let distances = djikstra(input, current);
        let (next, amount, distance) = distances
            .into_iter()
            .map(|(next, distance)| {
                let amount = if visited.contains(next) {
                    0
                } else {
                    let distance = distance + minutes + 1;
                    if distance > 30 {
                        0
                    } else {
                        input[next].rate * (30 - distance)
                    }
                };
                // println!(" -> {}: {}", next, amount);
                (next, amount, distance)
            })
            .max_by_key(|(_, amount, _)| *amount)
            .unwrap();

        visited.insert(next);
        current = next;
        total += amount;
        minutes += distance + 1;
        // println!("{}: {} ({})", minutes, current, total);
    }
    total
}

fn solve2(input: &str) -> usize {
    0
}

fn djikstra<'a>(valves: &'a HashMap<String, Valve>, start: &str) -> HashMap<&'a str, usize> {
    let mut queue = Vec::new();
    let mut visited = HashSet::new();
    let mut distances = HashMap::new();

    queue.push((valves[start].id.as_str(), 0));

    while let Some((current, distance)) = queue.pop() {
        if visited.contains(current) {
            continue;
        }
        visited.insert(current);
        distances.insert(current, distance);

        for next in &valves[current].next {
            let distance = distance + 1;
            let (Ok(index) | Err(index)) = queue.binary_search_by(|(_, d)| distance.cmp(d));

            queue.insert(index, (next, distance));
        }
    }
    distances
}

#[derive(Debug)]
struct Valve {
    id: String,
    rate: usize,
    next: Vec<String>,
}

impl FromStr for Valve {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        let mut parts = s.split(" ");
        let id = parts.nth(1).unwrap().to_string();

        let rate = parts.nth(2).unwrap().split_once('=').unwrap().1;
        let rate = rate.trim_matches(';').parse().unwrap();
        let next = if let Some((_, next)) = s.split_once("valves ") {
            next.split(", ").map(str::to_owned).collect()
        } else {
            vec![s.split_once("valve ").unwrap().1.to_owned()]
        };

        Ok(Self { id, rate, next })
    }
}
