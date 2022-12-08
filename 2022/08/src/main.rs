use std::str::FromStr;

const EXAMPLE: &str = include_str!("../example.txt");
const INPUT: &str = include_str!("../input.txt");

fn main() {
    let example: Grid = EXAMPLE.parse().unwrap();
    let input: Grid = INPUT.parse().unwrap();
    println!("Part 1:");
    let output = solve1(example.clone());
    println!("  example: {}", output);
    let output = solve1(input.clone());
    println!("  input: {}", output);

    println!("Part 2:");
    let output = solve2(example);
    println!("  example: {}", output);
    let output = solve2(input);
    println!("  input: {}", output);
}

fn solve1(mut input: Grid) -> usize {
    for y in 0..input.height {
        for x in 0..input.width {
            input.check_visibility(x, y);
        }
    }

    // input.print();
    input.count_visible()
}

fn solve2(input: Grid) -> i32 {
    let mut max = 0;
    for y in 0..input.height {
        for x in 0..input.width {
            let score = input.scenic_score(x, y);
            if score > max {
                max = score;
            }
        }
    }
    max
}

#[derive(Clone)]
struct Tree {
    height: u8,
    visible: Option<bool>,
}

#[derive(Clone)]
struct Grid {
    map: Vec<Tree>,
    width: i32,
    height: i32,
}

impl Grid {
    fn get(&self, x: i32, y: i32) -> Option<&Tree> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return None;
        }
        let index = (y * self.width + x) as usize;
        Some(&self.map[index])
    }

    fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut Tree> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return None;
        }
        let index = (y * self.width + x) as usize;
        Some(&mut self.map[index])
    }

    fn count_visible(&self) -> usize {
        self.map.iter().filter(|t| t.visible == Some(true)).count()
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let tree = self.get(x, y).unwrap();
                let c = match tree.visible {
                    Some(true) => 'O',
                    Some(false) => 'X',
                    None => '.',
                };
                print!("{}", c);
            }
            println!();
        }
    }

    fn scenic_score(&self, x: i32, y: i32) -> i32 {
        let current_height = self.get(x, y).unwrap().height;
        let mut x_tmp = x + 1;
        let mut score = 1;
        loop {
            match self.get(x_tmp, y) {
                Some(tree) => {
                    if tree.height >= current_height {
                        score *= x_tmp - x;
                        break;
                    }
                }
                None => {
                    score *= x_tmp - x - 1;
                    break;
                }
            }
            x_tmp += 1;
        }

        let mut x_tmp = x - 1;
        loop {
            match self.get(x_tmp, y) {
                Some(tree) => {
                    if tree.height >= current_height {
                        score *= x - x_tmp;
                        break;
                    }
                }
                None => {
                    score *= x - x_tmp - 1;
                    break;
                }
            }
            x_tmp -= 1;
        }

        let mut y_tmp = y + 1;
        loop {
            match self.get(x, y_tmp) {
                Some(tree) => {
                    if tree.height >= current_height {
                        score *= y_tmp - y;
                        break;
                    }
                }
                None => {
                    score *= y_tmp - y - 1;
                    break;
                }
            }
            y_tmp += 1;
        }

        let mut y_tmp = y - 1;
        loop {
            match self.get(x, y_tmp) {
                Some(tree) => {
                    if tree.height >= current_height {
                        score *= y - y_tmp;
                        break;
                    }
                }
                None => {
                    score *= y - y_tmp - 1;
                    break;
                }
            }
            y_tmp -= 1;
        }

        score
    }

    fn check_visibility(&mut self, x: i32, y: i32) {
        let current_height = self.get(x, y).unwrap().height;
        let mut x_tmp = x + 1;
        loop {
            match self.get(x_tmp, y) {
                Some(tree) => {
                    if tree.height >= current_height {
                        break;
                    }
                }
                None => {
                    self.get_mut(x, y).unwrap().visible = Some(true);
                    return;
                }
            }
            x_tmp += 1;
        }

        let mut x_tmp = x - 1;
        loop {
            match self.get(x_tmp, y) {
                Some(tree) => {
                    if tree.height >= current_height {
                        break;
                    }
                }
                None => {
                    self.get_mut(x, y).unwrap().visible = Some(true);
                    return;
                }
            }
            x_tmp -= 1;
        }

        let mut y_tmp = y + 1;
        loop {
            match self.get(x, y_tmp) {
                Some(tree) => {
                    if tree.height >= current_height {
                        break;
                    }
                }
                None => {
                    self.get_mut(x, y).unwrap().visible = Some(true);
                    return;
                }
            }
            y_tmp += 1;
        }

        let mut y_tmp = y - 1;
        loop {
            match self.get(x, y_tmp) {
                Some(tree) => {
                    if tree.height >= current_height {
                        break;
                    }
                }
                None => {
                    self.get_mut(x, y).unwrap().visible = Some(true);
                    return;
                }
            }
            y_tmp -= 1;
        }

        self.get_mut(x, y).unwrap().visible = Some(false);
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Vec::new();

        let width = s.lines().next().unwrap().len() as i32;
        let mut height = 0;

        for line in s.lines() {
            for c in line.chars() {
                let height = c as u8 - b'0';
                let tree = Tree {
                    height,
                    visible: None,
                };
                map.push(tree);
            }
            height += 1;
        }
        Ok(Grid { map, width, height })
    }
}
