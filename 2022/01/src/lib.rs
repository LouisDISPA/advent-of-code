#[cfg(test)]
mod tests;

type Input = Vec<Vec<usize>>;

pub fn parse_input(text: &str) -> Input {
    text.split("\n\n")
        .map(|block| block.lines().flat_map(str::parse).collect())
        .collect()
}

pub fn solve_part1(input: Input) -> usize {
    input
        .into_iter()
        .map(|elf| elf.into_iter().sum::<usize>())
        .max()
        .unwrap()
}

/// find the 3 elf with the highest score
pub fn solve_part2(input: Input) -> usize {
    let mut elfs_score = input
        .into_iter()
        .map(|elf| elf.into_iter().sum::<usize>())
        .collect::<Vec<usize>>();
    elfs_score.sort_by(|a, b| b.cmp(a));
    elfs_score[..3].iter().sum()
}
