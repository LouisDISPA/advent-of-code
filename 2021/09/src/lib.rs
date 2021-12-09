#[cfg(test)]
mod tests;

type Input = Vec<Vec<usize>>;

pub fn parse_input(text: &str) -> Input {
    text.lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as usize))
        .map(Iterator::collect::<Vec<_>>)
        .collect()
}

pub fn solve_part1(input: Input) -> usize {
    // for line in &input {
    //     println!("{:?}", line);
    // }
    let mut sum = 0;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if check_position(&input, x, y) {
                sum += input[y][x] + 1;
            }
        }
    }
    sum
}

pub fn solve_part2(input: Input) -> usize {
    // for line in &input {
    //     println!("{:?}", line);
    // }
    let mut positions = Vec::new();
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if check_position(&input, x, y) {
                positions.push((x, y));
            }
        }
    }

    let mut pass = vec![vec![false; input[0].len()]; input.len()];

    let mut sizes: Vec<usize> = positions
        .into_iter()
        .map(|(x, y)| basin_size(&input, &mut pass, x, y))
        .collect();

    sizes.sort();
    sizes.reverse();
    sizes[0] * sizes[1] * sizes[2]
}

const POSITIONS_CHECKED: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn check_position(grid: &[Vec<usize>], x: usize, y: usize) -> bool {
    let value = grid[y][x];
    let len_y = grid.len() as isize;
    let len_x = grid[0].len() as isize;

    for (x, y) in POSITIONS_CHECKED
        .iter()
        .map(|(delta_x, delta_y)| (x as isize + delta_x, y as isize + delta_y))
        .filter(|&(x, y)| x >= 0 && y >= 0 && x < len_x && y < len_y)
    {
        if grid[y as usize][x as usize] <= value {
            return false;
        }
    }
    true
}

fn basin_size(grid: &[Vec<usize>], pass: &mut [Vec<bool>], x: usize, y: usize) -> usize {
    if pass[y][x] || grid[y][x] >= 9 {
        return 0;
    }
    let len_y = grid.len() as isize;
    let len_x = grid[0].len() as isize;
    pass[y][x] = true;
    let mut sum = 0;
    for (x, y) in POSITIONS_CHECKED
        .iter()
        .map(|(delta_x, delta_y)| (x as isize + delta_x, y as isize + delta_y))
        .filter(|&(x, y)| x >= 0 && y >= 0 && x < len_x && y < len_y)
    {
        sum += basin_size(grid, pass, x as usize, y as usize);
    }
    sum + 1
}
