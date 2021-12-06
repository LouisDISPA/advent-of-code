use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl FromStr for Position {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) =  s.split_once(',').ok_or(format!("Missing ',' for parsing position: '{}'", s))?;
        let x = x.trim().parse().map_err(|_| format!("X value parsing failed: '{}'", s))?;
        let y = y.trim().parse().map_err(|_| format!("Y value parsing failed: '{}'", s))?;
        Ok(Self { x, y })
    }
}
