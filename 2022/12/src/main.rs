use std::str::FromStr;

const EXAMPLE: &str = include_str!("../example.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part 1:");
    let output = solve1(EXAMPLE);
    println!("  example: {}", output);
    let output = solve1(INPUT);
    println!("  input: {}", output);

    println!("\nPart 2:");
    let output = solve2(EXAMPLE);
    println!("  example: {}", output);
    let output = solve2(INPUT);
    println!("  input: {}", output);
}

fn solve1(input: &str) -> usize {
    let map = Map::from_str(input).unwrap();
    let start = (map.start % map.width, map.start / map.width);
    let end_condition = |x, y| {
        let index = y * map.width + x;
        index == map.end
    };
    let step_condition = |before, after| after <= before + 1;
    map.djikstra(start, end_condition, step_condition).unwrap()
}

fn solve2(input: &str) -> usize {
    let map = Map::from_str(input).unwrap();
    let start = (map.end % map.width, map.end / map.width);
    let end_condition = |x, y| map.get(x, y).unwrap() == 0;
    let step_condition = |before, after| before <= after + 1;
    map.djikstra(start, end_condition, step_condition).unwrap()
}

struct Map {
    data: Vec<u8>,
    width: isize,
    height: isize,
    start: isize,
    end: isize,
}

impl Map {
    fn get(&self, x: isize, y: isize) -> Option<u8> {
        if x < 0 || y < 0 {
            return None;
        }
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(self.data[(y * self.width + x) as usize])
    }

    fn djikstra(
        &self,
        start: (isize, isize),
        end_condition: impl Fn(isize, isize) -> bool,
        step_condition: impl Fn(u8, u8) -> bool,
    ) -> Option<usize> {
        let mut queue = Vec::new();
        let mut visited = vec![false; self.data.len()];

        queue.push((start.0, start.1, 0));

        while let Some((x, y, distance)) = queue.pop() {
            let index = (y * self.width + x) as usize;
            if end_condition(x, y) {
                return Some(distance);
            }
            if visited[index] {
                continue;
            }
            visited[index] = true;

            let neighbors = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];

            let current_value = self.get(x, y).unwrap();

            for (x, y) in neighbors.iter() {
                if let Some(tile) = self.get(*x, *y) {
                    let index = y * self.width + x;
                    if !visited[index as usize] && step_condition(current_value, tile) {
                        let distance = distance + 1;
                        let index = queue.binary_search_by(|(_, _, d)| distance.cmp(d));
                        let index = match index {
                            Ok(index) => index,
                            Err(index) => index,
                        };
                        queue.insert(index, (*x, *y, distance));
                    }
                }
            }
        }
        None
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = None;
        let mut end = None;
        let width = s.lines().next().unwrap().len() as isize;
        let data: Vec<u8> = s
            .lines()
            .flat_map(|line| line.chars())
            .enumerate()
            .map(|(i, c)| match c {
                'S' => {
                    start = Some(i as isize);
                    0
                }
                'E' => {
                    end = Some(i as isize);
                    25
                }
                c => c as u8 - b'a',
            })
            .collect();
        let height = data.len() as isize / width;
        Ok(Map {
            data,
            start: start.unwrap(),
            end: end.unwrap(),
            width,
            height,
        })
    }
}
