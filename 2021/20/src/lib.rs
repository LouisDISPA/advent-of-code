#[cfg(test)]
mod tests;

mod point;
use std::collections::HashSet;

use point::Point;

type Input = (Vec<bool>, HashSet<Point>);

pub fn parse_input(text: &str) -> Input {
    let (bits, points) = text.split_once("\n\n").unwrap();

    let bits = bits.chars().map(|s| s == '#').collect();
    let points = points
        .lines()
        .enumerate()
        .flat_map(|(y, s)| {
            s.char_indices().filter_map(move |(x, c)| {
                if c == '#' {
                    Some(Point {
                        x: x as isize,
                        y: y as isize,
                    })
                } else {
                    None
                }
            })
        })
        .collect();

    (bits, points)
}

pub fn solve_part1((bits, mut points): Input) -> usize {
    let mut x_max = points.iter().map(|p| p.x).max().unwrap() + 3;
    let mut x_min = points.iter().map(|p| p.x).min().unwrap() - 3;
    let mut y_max = points.iter().map(|p| p.y).max().unwrap() + 3;
    let mut y_min = points.iter().map(|p| p.y).min().unwrap() - 3;

    println!("{}, {}, {}, {}", x_max, x_min, y_max, y_min);

    for _ in 0..2 {
        println!("{}", points.len());
        print_board(&points);

        let positions_to_check =
            (x_min..=x_max).flat_map(move |x| (y_min..=y_max).map(move |y| Point { x, y }));
        let mut new_points = HashSet::new();

        for pos in positions_to_check {
            let number = pos.kernel().enumerate().fold(0, |acc, (index, pos)| {
                if points.contains(&pos) {
                    acc + 2u32.pow(index as u32)
                } else {
                    acc
                }
            });
            if bits[number as usize] {
                new_points.insert(pos);
            }
        }
        points = new_points;
        print_board(&points);

        let insert = points.contains(&Point {
            x: x_min + 2,
            y: y_min + 2,
        });

        for y in [y_min, y_max] {
            for x in x_min..=x_max {
                let point = Point { x, y };
                if insert {
                    points.insert(point);
                } else {
                    points.remove(&point);
                }
            }
        }

        for x in [x_min, x_max] {
            for y in y_min..=y_max {
                let point = Point { x, y };
                if insert {
                    points.insert(point);
                } else {
                    points.remove(&point);
                }
            }
        }

        x_max += 1;
        x_min -= 1;
        y_max += 1;
        y_min -= 1;

        for y in [y_min, y_max] {
            for x in x_min..=x_max {
                let point = Point { x, y };
                if insert {
                    points.insert(point);
                } else {
                    points.remove(&point);
                }
            }
        }

        for x in [x_min, x_max] {
            for y in y_min..=y_max {
                let point = Point { x, y };
                if insert {
                    points.insert(point);
                } else {
                    points.remove(&point);
                }
            }
        }
        print_board(&points);
    }
    println!("{}", points.len());

    print_board(&points);
    points.len()
}

pub fn solve_part2((bits, mut points): Input) -> usize {
    let mut x_max = points.iter().map(|p| p.x).max().unwrap() + 3;
    let mut x_min = points.iter().map(|p| p.x).min().unwrap() - 3;
    let mut y_max = points.iter().map(|p| p.y).max().unwrap() + 3;
    let mut y_min = points.iter().map(|p| p.y).min().unwrap() - 3;

    for _ in 0..50 {
        let positions_to_check =
            (x_min..=x_max).flat_map(move |x| (y_min..=y_max).map(move |y| Point { x, y }));
        let mut new_points = HashSet::new();

        for pos in positions_to_check {
            let number = pos.kernel().enumerate().fold(0, |acc, (index, pos)| {
                if points.contains(&pos) {
                    acc + 2u32.pow(index as u32)
                } else {
                    acc
                }
            });
            if bits[number as usize] {
                new_points.insert(pos);
            }
        }
        points = new_points;

        let insert = points.contains(&Point {
            x: x_min + 2,
            y: y_min + 2,
        });

        for y in [y_min, y_max] {
            for x in x_min..=x_max {
                let point = Point { x, y };
                if insert {
                    points.insert(point);
                } else {
                    points.remove(&point);
                }
            }
        }

        for x in [x_min, x_max] {
            for y in y_min..=y_max {
                let point = Point { x, y };
                if insert {
                    points.insert(point);
                } else {
                    points.remove(&point);
                }
            }
        }

        x_max += 1;
        x_min -= 1;
        y_max += 1;
        y_min -= 1;

        for y in [y_min, y_max] {
            for x in x_min..=x_max {
                let point = Point { x, y };
                if insert {
                    points.insert(point);
                } else {
                    points.remove(&point);
                }
            }
        }

        for x in [x_min, x_max] {
            for y in y_min..=y_max {
                let point = Point { x, y };
                if insert {
                    points.insert(point);
                } else {
                    points.remove(&point);
                }
            }
        }
    }

    points.len()
}

fn print_board(points: &HashSet<Point>) {
    let x_max = points.iter().map(|p| p.x).max().unwrap();
    let x_min = points.iter().map(|p| p.x).min().unwrap();
    let y_max = points.iter().map(|p| p.y).max().unwrap();
    let y_min = points.iter().map(|p| p.y).min().unwrap();
    println!("-----");
    for y in y_min..=y_max {
        let line: String = (x_min..=x_max)
            .map(|x| {
                if points.contains(&Point { x, y }) {
                    '#'
                } else {
                    '.'
                }
            })
            .collect();
        println!("{}", line);
    }
    println!("-----");
}
