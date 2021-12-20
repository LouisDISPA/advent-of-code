use crate::{parse_input, solve_part1, solve_part2};

const EXAMPLE: &str = include_str!("../example.txt");
const INPUT: &str = include_str!("../input.txt");

#[test]
fn part_1_example() {
    let input = parse_input(EXAMPLE);
    let result = solve_part1(input);
    assert_eq!(result, 35);
}

#[test]
fn part_1_input() {
    let input = parse_input(INPUT);
    let result = solve_part1(input);
    assert_eq!(result, 5229);
}

#[test]
fn part_2_example() {
    let input = parse_input(EXAMPLE);
    let result = solve_part2(input);
    assert_eq!(result, 3351);
}

#[test]
fn part_2_input() {
    let input = parse_input(INPUT);
    let result = solve_part2(input);
    assert_eq!(result, 17009);
}
