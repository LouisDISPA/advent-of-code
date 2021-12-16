#[cfg(test)]
mod tests;

type Input<'a> = Vec<Vec<usize>>;

pub fn parse_input(text: &str) -> Input {
    text.lines()
        .map(|s| s.trim().split("").filter_map(|s| s.parse().ok()).collect())
        .collect()
}

const POSITION_CHECK: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

pub fn solve_part1(grid: Input) -> usize {
    dijkstra(grid)
}

pub fn solve_part2(mut grid: Input) -> usize {
    for line in &mut grid {
        let old = line.clone();
        for i in 1..5 {
            line.append(&mut old.iter().map(|v| (((v - 1) + i) % 9) + 1).collect())
        }
    }

    let old = grid.clone();
    for i in 1..5 {
        for line in &old {
            grid.push(line.iter().map(|v| (((v - 1) + i) % 9) + 1).collect())
        }
    }
    // for line in &grid {
    //     println!("{:?}", line);
    // }

    dijkstra(grid)
}

fn dijkstra(mut grid: Input) -> usize {
    let mut visited_node = Vec::new();

    let end = (grid[0].len() - 1, grid.len() - 1);
    let start = (0, 0);
    let start_cost = 0;
    visited_node.push((start_cost, start));

    loop {
        if let Some((cost, pos)) = visited_node.pop() {
            if pos == end {
                break cost;
            }
            // println!("{:?} -> {}", pos, cost);
            // println!("{:?}", visited_node);
            grid[pos.1][pos.0] = usize::MAX;

            for pos in POSITION_CHECK
                .iter()
                .map(|delta| (pos.0 as isize + delta.0, pos.1 as isize + delta.1))
                .filter(|&pos| {
                    pos.0 >= 0 && pos.1 >= 0 && pos.0 <= end.0 as isize && pos.1 <= end.1 as isize
                })
            {
                let pos = (pos.0 as usize, pos.1 as usize);
                if grid[pos.1][pos.0] != usize::MAX {
                    let cost = cost + grid[pos.1][pos.0];
                    let index = visited_node
                        .binary_search_by(|(c, _)| cost.cmp(c))
                        .unwrap_or_else(|e| e);
                    if !visited_node.iter().any(|(_, p)| p == &pos) {
                        visited_node.insert(index, (cost, pos));
                    }
                }
            }
        } else {
            panic!("No trace found to the end.")
        }
    }
}