#[cfg(test)]
mod tests;
use std::{collections::HashMap, mem::take};

type Input = HashMap<(usize, usize), SeaCuCumber>;

#[derive(Clone, Copy)]
pub enum SeaCuCumber {
    East,
    South,
}

pub fn parse_input(text: &str) -> Input {
    text.lines()
        .enumerate()
        .flat_map(|(y, s)| {
            s.char_indices().filter_map(move |(x, c)| match c {
                '>' => Some(((x, y), SeaCuCumber::East)),
                'v' => Some(((x, y), SeaCuCumber::South)),
                _ => None,
            })
        })
        .collect()
}

pub fn solve_part1(mut see_cucumbers: Input) -> usize {
    let x_mas = *see_cucumbers.keys().map(|(x, _)| x).max().unwrap() + 1;
    let y_max = *see_cucumbers.keys().map(|(_, y)| y).max().unwrap() + 1;

    let mut updates = 0;

    for _ in 0..1000 {
        // print_board(&see_cucumbers, x_mas, y_max);
        let moves = update_board(&mut see_cucumbers, x_mas, y_max);
        updates += 1;
        if moves == 0 {
            return updates;
        }
    }

    0
}

fn print_board(points: &Input, x_mas: usize, y_max: usize) {
    println!("-----");
    for y in 0..=y_max {
        let line: String = (0..=x_mas)
            .map(|x| match points.get(&(x, y)) {
                Some(&SeaCuCumber::East) => '>',
                Some(&SeaCuCumber::South) => 'v',
                None => '.',
            })
            .collect();
        println!("{}", line);
    }
    println!("-----");
}

fn update_board(board: &mut Input, x_mas: usize, y_max: usize) -> usize {
    let old_board = take(board);
    let mut moves = 0;

    let east = old_board
        .iter()
        .filter(|(_, v)| matches!(v, SeaCuCumber::East));

    for (&(x, y), &value) in east {
        let new_x = (x + 1) % x_mas;

        if old_board.contains_key(&(new_x, y)) {
            board.insert((x, y), value);
        } else {
            moves += 1;
            board.insert((new_x, y), value);
        }
    }

    let south: Input = old_board
        .iter()
        .filter(|(_, v)| matches!(v, SeaCuCumber::South))
        .map(|(&key, &value)| (key, value))
        .collect();

    for (&(x, y), &value) in &south {
        let new_y = (y + 1) % y_max;

        if !board.contains_key(&(x, new_y)) && !south.contains_key(&(x, new_y)) {
            moves += 1;
            board.insert((x, new_y), value);
        } else {
            board.insert((x, y), value);
        }
    }
    moves
}
