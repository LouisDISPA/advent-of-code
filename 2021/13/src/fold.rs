use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Fold {
    pub axis: Axis,
    pub position: usize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Axis {
    X,
    Y,
}

impl FromStr for Fold {
    type Err = String;

    fn from_str(fold: &str) -> Result<Self, Self::Err> {
        if !fold.starts_with("fold along") {
            Err(format!("Not a fold: {}", fold))
        } else {
            let (axis, position) = fold
                .split_once('=')
                .ok_or_else(|| format!("No equal sign in fold: {}", fold))?;
            let axis = match axis.trim() {
                "fold along x" => Axis::X,
                "fold along y" => Axis::Y,
                _ => return Err(format!("Fold axis unknown: {}", fold)),
            };
            let position = position
                .trim()
                .parse()
                .map_err(|_| format!("Failed to parse fold count: {}", fold))?;
            Ok(Self { axis, position })
        }
    }
}
