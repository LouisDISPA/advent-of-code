use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

fn parse_to_usize<T: FromStr>(value: Option<&str>, point: &str, axis: char) -> Result<T, String> {
    value
        .ok_or_else(|| format!("missing {} value for Point: {}", axis, point))?
        .trim()
        .parse()
        .map_err(|_| format!("missing {} value for Point: {}", axis, point))
}

impl FromStr for Point {
    type Err = String;

    fn from_str(point: &str) -> Result<Self, Self::Err> {
        let mut values = point.split(',');
        let x = parse_to_usize(values.next(), point, 'X')?;
        let y = parse_to_usize(values.next(), point, 'Y')?;
        Ok(Self { x, y })
    }
}
