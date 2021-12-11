#[cfg(test)]
mod tests;

type Input = Vec<Vec<usize>>;

pub fn parse_input(text: &str) -> Input {
    text.lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as usize))
        .map(Iterator::collect::<Vec<_>>)
        .collect()
}

pub fn solve_part1(mut input: Input) -> usize {
    // for line in &input {
    //     println!("{:?}", line);
    // }
    let mut sum = 0;
    for _ in 0..100 {
        for y in 0..input.len() {
            for x in 0..input[0].len() {
                sum += update_position(&mut input, x, y);
            }
        }
        for pos in input.iter_mut().flatten() {
            if *pos == 10 {
                *pos = 0;
            }
        }
    }
    sum
}

pub fn solve_part2(mut input: Input) -> usize {

    for index in 0..1000 {
        for y in 0..input.len() {
            for x in 0..input[0].len() {
                update_position(&mut input, x, y);
            }
        }
        let mut count = 0;
        for pos in input.iter_mut().flatten() {
            if *pos == 10 {
                *pos = 0;
                count += 1;
            }
        }
        if count == 100 {
            return index + 1;
        }
    }
    0
}

const POSITIONS_CHECKED: [(isize, isize); 8] = [(0, 1), (0, -1), (1, 0), (-1, 0),  (-1, -1), (1, 1), (-1, 1), (1, -1)];

fn update_position(grid: &mut [Vec<usize>], x: usize, y: usize) -> usize {
    if grid[y][x] >= 10 {
        return 0;
    }
    let len_y = grid.len() as isize;
    let len_x = grid[0].len() as isize;
    grid[y][x] += 1;
    if grid[y][x] < 10 {
        return 0
    }

    let mut sum = 0;
    for (x, y) in POSITIONS_CHECKED
        .iter()
        .map(|(delta_x, delta_y)| (x as isize + delta_x, y as isize + delta_y))
        .filter(|&(x, y)| x >= 0 && y >= 0 && x < len_x && y < len_y)
    {
        sum += update_position(grid, x as usize, y as usize);
    }

    sum + 1
}
