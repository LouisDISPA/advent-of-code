#![feature(bool_to_option)]

use std::convert::TryInto;
use std::str::FromStr;
use std::process::exit;
use std::iter::repeat;

const BOARD_HEIGHT: usize = 5;
const BOARD_WIDTH: usize = 5;

fn main() {
    let mut inputs = include_str!("input.txt").split("\n\n");

    let first_line = inputs.next().unwrap();
    let numbers: Vec<isize> = first_line.split(',').filter_map(|s| s.parse().ok()).collect();
    let mut boards: Vec<Board> = inputs.map(str::parse).collect::<Result<_, _>>().unwrap();

    for draw in numbers {
        for board in boards.iter_mut() {
            let win = board.check_draw(draw);
            if win {
                let sum = board.sum_unchecked_squares();
                println!("{} (sum) * {} (draw) = {}", sum, draw, sum * draw);
                exit(0);
            }
        }
    }
}

#[derive(Debug, Default)]
struct Square {
    value: isize,
    checked: bool,
}

impl From<(isize, bool)> for Square {
    fn from((value, checked): (isize, bool)) -> Self {
        Self { value, checked }
    }
}

#[derive(Debug, Default)]
struct Board {
    squares: [Square; BOARD_HEIGHT * BOARD_WIDTH],
}

#[derive(Debug)]
struct ParseBoardError;

impl FromStr for Board {
    type Err = ParseBoardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s.split('\n').flat_map(|s| s.split(' ')).filter_map(|s| s.parse::<isize>().ok());
        let squares: Vec<Square> = values.zip(repeat(false)).map(From::from).collect();

        let squares = match squares.try_into() {
            Err(_) => return Err(ParseBoardError),
            Ok(v) => v
        };
    
        Ok(Self { squares })
    }
}

fn index(row: usize, col: usize) -> usize {
    row * BOARD_WIDTH + col
}

impl Board {
    fn check_draw(&mut self, draw: isize) -> bool {
        if let Some((row, col)) = self.find_value(draw) {
            self.squares[index(row, col)].checked = true;

            let row = self.squares.iter().skip(row * BOARD_WIDTH).take(BOARD_WIDTH);
            let col = self.squares.iter().skip(col).step_by(BOARD_WIDTH);

            if row.filter(|s| s.checked).count() == BOARD_WIDTH ||
               col.filter(|s| s.checked).count() == BOARD_HEIGHT {
                return true;
            }
        }
        false
    }

    fn find_value(&self, value: isize) -> Option<(usize, usize)> {
        for row in 0..BOARD_WIDTH {
            for col in 0..BOARD_HEIGHT {
                if value == self.squares[index(row, col)].value {
                    return Some((row, col));
                }
            }
        }
        None
    }

    fn sum_unchecked_squares(&self) -> isize {
        self.squares.iter().filter_map(|sq| (!sq.checked).then_some(sq.value)).sum()
    }
}
