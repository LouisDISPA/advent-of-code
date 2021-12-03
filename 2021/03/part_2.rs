fn main() {
    let lines: Vec<&str> = include_str!("input.txt").split('\n').collect();

    let gama = calculate(&lines, |ones, zeros| ones >= zeros);
    let epsilon = calculate(&lines, |ones, zeros| ones < zeros);

    let gama = bits_to_number(gama);
    let epsilon = bits_to_number(epsilon);

    println!(
        "{} (gama) * {} (epsilon) = {}",
        gama,
        epsilon,
        gama * epsilon
    );
}

fn calculate<F: Fn(usize, usize) -> bool>(lines: &[&str], cmp: F) -> String {
    let mut lines = lines.to_vec();
    for index in 0..lines[0].len() {
        let (ones, zeros): (Vec<&str>, Vec<&str>) = lines
            .iter()
            .partition(|line| line.chars().nth(index) == Some('1'));

        if cmp(ones.len(), zeros.len()) {
            lines = ones;
        } else {
            lines = zeros;
        }
        if lines.len() == 1 {
            return lines[0].to_string();
        }
    }
    return lines[0].to_string();
}

fn bits_to_number(bits: String) -> u32 {
    let mut value = 0;

    for (index, bit) in bits.chars().rev().enumerate() {
        if bit == '1' {
            value += 2_u32.pow(index as u32);
        }
    }

    value
}
