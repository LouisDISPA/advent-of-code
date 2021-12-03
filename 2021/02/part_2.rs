fn main() {
    let lines = include_str!("input.txt").split('\n');

    let mut depth = 0;
    let mut x = 0;
    let mut aim = 0;

    for line in lines {
        let mut words = line.split(' ');

        let command = words.next();
        let value: Option<isize> = words.next().map(str::parse).map(Result::unwrap);
        match command {
            Some("down") => aim += value.unwrap(),
            Some("up") => aim -= value.unwrap(),
            Some("forward") => {
                let value = value.unwrap();
                x += value;
                depth += aim * value;
            },
            _ => continue
        };
    }
    println!("{}", x * depth)
}
