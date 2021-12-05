use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl FromStr for Position {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = s.split(',').map(str::trim).map(str::parse);

        let x = match values.next() {
            Some(Ok(v)) => v,
            _ => return Err(format!("X value parsing failed for position: {}", s)),
        };
        let y = match values.next() {
            Some(Ok(v)) => v,
            _ => return Err(format!("Y value parsing failed for position: {}", s)),
        };
        Ok(Self { x, y })
    }
}
