use crate::{parse_input, snailfish::Snailfish, solve_part1, solve_part2};

const EXAMPLE: &str = include_str!("../example.txt");
const INPUT: &str = include_str!("../input.txt");

#[test]
fn part_1_example_reduces() {
    let a: Snailfish = "[[[[4,3],4],4],[7,[[8,4],9]]]".parse().unwrap();
    let b: Snailfish = "[1,1]".parse().unwrap();
    let mut sum = a + b;
    sum.reduce();
    assert_eq!(&sum.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");

    let reduce_tests = [
        ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
        ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
        ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
        (
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        ),
    ];

    for (start, end) in reduce_tests {
        let mut a: Snailfish = start.parse().unwrap();
        a.reduce();
        assert_eq!(&a.to_string(), end);
    }
}

#[test]
fn part_1_example_list() {
    let reduce_tests = [
        (
            "[1,1]\n[2,2]\n[3,3]\n[4,4]",
            "[[[[1,1],[2,2]],[3,3]],[4,4]]",
        ),
        (
            "[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]\n",
            "[[[[3,0],[5,3]],[4,4]],[5,5]]",
        ),
        (
            "[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]\n[6,6]",
            "[[[[5,0],[7,4]],[5,5]],[6,6]]",
        ),
    ];
    for (start, end) in reduce_tests {
        let snails = parse_input(start);
        let snail = snails
            .into_iter()
            .reduce(|a, b| {
                println!("{}", a);
                println!("+ {}", b);
                let mut sum = a + b;
                sum.reduce();
                println!("= {}", sum);
                sum
            })
            .unwrap();
        assert_eq!(&snail.to_string(), end)
    }
}


#[test]
fn part_1_example_big() {
    let reduce_tests = [
        (
            "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]",
            "[2,9]",
            "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]"
        )
    ];
    for (a, b, res) in reduce_tests {
        let a: Snailfish = a.parse().unwrap();
        let b: Snailfish = b.parse().unwrap();
        let mut sum = a + b;
        sum.reduce();
        assert_eq!(&sum.to_string(), res)
    }
}

#[test]
fn part_1_example() {
    let input = parse_input(EXAMPLE);
    let result = solve_part1(input);
    assert_eq!(result, 4140);
}

#[test]
fn part_1_input() {
    let input = parse_input(INPUT);
    let result = solve_part1(input);
    assert_eq!(result, 4033);
}

#[test]
fn part_2_example() {
    let input = parse_input(EXAMPLE);
    let result = solve_part2(input);
    assert_eq!(result, 3993);
}

#[test]
fn part_2_input() {
    let input = parse_input(INPUT);
    let result = solve_part2(input);
    assert_eq!(result, 0);
}
