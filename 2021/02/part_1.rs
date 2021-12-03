fn main() {
    let lines = include_str!("input.txt").split('\n');

    let mut depth = 0;
    let mut x = 0;

    for line in lines {
        let mut words = line.split(' ');

        let command = words.next();
        let value: Option<isize> = words.next().map(str::parse).map(Result::unwrap);
        match command {
            Some("forward") => x += value.unwrap(),
            Some("up") => depth -= value.unwrap(),
            Some("down") => depth += value.unwrap(),
            _ => continue
        };
    }
    println!("{}", x * depth)
}
