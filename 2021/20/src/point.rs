#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

const CYCLE: [(isize, isize); 9] = [
    (1, 1),
    (0, 1),
    (-1, 1),
    (1, 0),
    (0, 0),
    (-1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

impl Point {
    pub fn kernel(&self) -> impl Iterator<Item = Point> {
        let x = self.x;
        let y = self.y;
        CYCLE.iter().map(move |&(delta_x, delta_y)| Point {
            x: x + delta_x,
            y: y + delta_y,
        })
    }
}
