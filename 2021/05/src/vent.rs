use std::{cmp::Ordering, str::FromStr};

use crate::position::Position;

#[derive(Debug)]
pub struct Vent {
    pub start: Position,
    pub end: Position,
}


impl FromStr for Vent {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) =  s.split_once("->").ok_or(format!("Missing '->' for parsing position: '{}'", s))?;
        let start = start.trim().parse()?;
        let end = end.trim().parse()?;
        Ok(Self { start, end })
    }
}

impl Vent {
    pub fn positions(&self) -> impl Iterator<Item = Position> {
        let Vent { start, end } = *self;

        let start_x = start.x.min(end.x);
        let end_x = start.x.max(end.x);
        let delta_x = start.x.cmp(&end.x);

        let start_y = start.y.min(end.y);
        let end_y = start.y.max(end.y);
        let delta_y = start.y.cmp(&end.y);

        let len = (end_x - start_x).max(end_y - start_y);

        (0..=len).map(move |delta| {
            let x = match delta_x {
                Ordering::Equal => start_x,
                Ordering::Greater => end_x - delta,
                Ordering::Less => start_x + delta,
            };
            let y = match delta_y {
                Ordering::Equal => start_y,
                Ordering::Greater => end_y - delta,
                Ordering::Less => start_y + delta,
            };
            Position { x, y }
        })
    }

    pub fn is_diag(&self) -> bool {
        let Vent { start, end } = *self;
        start.x != end.x && start.y != end.y
    }
}
