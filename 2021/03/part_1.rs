fn main() {
    let mut lines = include_str!("input.txt").split('\n').peekable();

    let size = lines.peek().unwrap().len();
    let mut ones_by_columns = vec![0; size];
    let mut line_count = 0;

    for line in lines {
        if line.len() > 0 {
            line_count += 1;
        }

        for (ones, bit) in ones_by_columns.iter_mut().zip(line.chars()) {
            if bit == '1' {
                *ones += 1;
            }
        }
    }

    let mut gama = 0;
    let mut epsilon = 0;

    for (index, ones) in ones_by_columns.into_iter().rev().enumerate() {
        if ones * 2 > line_count {
            gama += 2_u32.pow(index as u32)
        } else {
            epsilon += 2_u32.pow(index as u32)
        }
    }

    println!("{} (gama) * {} (epsilon) = {}", gama, epsilon, gama * epsilon);
}
