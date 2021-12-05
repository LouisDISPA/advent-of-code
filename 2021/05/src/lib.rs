mod position;
mod vent;

#[cfg(test)]
mod tests;

use std::collections::HashMap;

use position::Position;
use vent::Vent;

type Input = Vec<Vent>;

pub fn parse_input(text: &str) -> Input {
    text.lines().map(str::parse).map(Result::unwrap).collect()
}

pub fn solve_part1(input: Input) -> usize {
    let mut position_count = HashMap::<Position, isize>::new();

    input
        .iter()
        .filter(|vent| !vent.is_diag())
        .flat_map(Vent::positions)
        .fold(&mut position_count, |position_count, pos| {
            if let Some(count) = position_count.get_mut(&pos) {
                *count += 1;
            } else {
                position_count.insert(pos, 1);
            }
            position_count
        });
    position_count.into_values().filter(|&v| v > 1).count()
}

pub fn solve_part2(input: Input) -> usize {
    let mut position_count = HashMap::<Position, isize>::new();

    input
        .iter()
        .flat_map(Vent::positions)
        .fold(&mut position_count, |position_count, pos| {
            if let Some(count) = position_count.get_mut(&pos) {
                *count += 1;
            } else {
                position_count.insert(pos, 1);
            }
            position_count
        });
    position_count.into_values().filter(|&v| v > 1).count()
}
