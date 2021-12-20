use std::{fmt::Display, ops::Add, str::FromStr};

#[derive(Debug, Clone)]
pub struct Snailfish {
    left: Node,
    right: Node,
}

#[derive(Debug, Clone)]
enum Node {
    Snailfish(Box<Snailfish>),
    Value(u32),
}

impl FromStr for Snailfish {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coma = None;

        let mut lvl = 0;
        for (index, c) in s.char_indices() {
            match c {
                '[' => lvl += 1,
                ']' => lvl -= 1,
                ',' => {
                    if lvl == 1 {
                        coma = Some(index)
                    }
                }
                _ => {}
            }
        }

        let index = coma.unwrap();

        let snailfish = Snailfish {
            left: Node::from_str(&s[1..index])?,
            right: Node::from_str(&s[(index + 1)..(s.len() - 1)])?,
        };

        Ok(snailfish)
    }
}

impl FromStr for Node {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if &s[0..1] == "[" {
            Ok(Self::Snailfish(Box::new(s.parse().unwrap())))
        } else {
            Ok(Self::Value(s.parse().unwrap()))
        }
    }
}

impl Display for Snailfish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.left, self.right)
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Snailfish(s) => write!(f, "{}", s),
            Node::Value(v) => write!(f, "{}", v),
        }
    }
}

impl Add<Snailfish> for Snailfish {
    type Output = Snailfish;

    fn add(self, rhs: Snailfish) -> Self::Output {
        Snailfish {
            left: self.into(),
            right: rhs.into(),
        }
    }
}

impl From<Snailfish> for Node {
    fn from(snail: Snailfish) -> Self {
        Self::Snailfish(Box::new(snail))
    }
}

impl From<u32> for Node {
    fn from(value: u32) -> Self {
        Self::Value(value)
    }
}

impl Snailfish {
    pub fn magnitude(&self) -> u32 {
        let left = match &self.left {
            Node::Snailfish(s) => s.magnitude(),
            Node::Value(v) => *v,
        };

        let right = match &self.right {
            Node::Snailfish(s) => s.magnitude(),
            Node::Value(v) => *v,
        };

        (3 * left) + (2 * right)
    }

    pub fn reduce(&mut self) {
        loop {
            let mut reduce = self.explode(0);
            if matches!(reduce, Reduce::None) {
                reduce = self.split(0);
            }
            if matches!(reduce, Reduce::None) {
                break;
            }
        }
    }

    fn explode(&mut self, depth: usize) -> Reduce {
        if depth >= 4 {
            if let Snailfish {
                left: Node::Value(left),
                right: Node::Value(right),
            } = self
            {
                return Reduce::Explode(*left, *right);
            }
        }

        let reduced = match &mut self.left {
            Node::Snailfish(s) => s.explode(depth + 1),
            _ => Reduce::None
        };
        match reduced {
            Reduce::Explode(left, right) => {
                self.left = Node::Value(0);
                match &mut self.right {
                    Node::Snailfish(s) => s.add_left(right),
                    Node::Value(v) => *v += right,
                }
                return Reduce::Exploded(Some(left), None);
            }
            Reduce::Exploded(left, Some(right)) => {
                match &mut self.right {
                    Node::Snailfish(s) => s.add_left(right),
                    Node::Value(v) => *v += right,
                }
                return Reduce::Exploded(left, None);
            }
            Reduce::Exploded(left, right) => {
                return Reduce::Exploded(left, right);
            }
            Reduce::Split => return Reduce::Split,
            Reduce::None => {}
        }

        let reduced = match &mut self.right {
            Node::Snailfish(s) => s.explode(depth + 1),
            _ => Reduce::None
        };
        match reduced {
            Reduce::Explode(left, right) => {
                self.right = Node::Value(0);
                match &mut self.left {
                    Node::Snailfish(s) => s.add_right(left),
                    Node::Value(v) => *v += left,
                }
                Reduce::Exploded(None, Some(right))
            }
            Reduce::Exploded(Some(left), right) => {
                match &mut self.left {
                    Node::Snailfish(s) => s.add_right(left),
                    Node::Value(v) => *v += left,
                }
                Reduce::Exploded(None, right)
            }
            _ => reduced,
        }

    }

    fn split(&mut self, depth: usize) -> Reduce {
        let reduced = match &mut self.left {
            Node::Snailfish(s) => s.split(depth + 1),
            Node::Value(v) => {
                if *v >= 10 {
                    let left = Node::Value(*v / 2);
                    let right = Node::Value(if (*v) & 1 != 0 { *v / 2 + 1 } else { *v / 2 });
                    self.left = Snailfish { left, right }.into();
                    Reduce::Split
                } else {
                    Reduce::None
                }
            }
        };
        match reduced {
            Reduce::Split => return Reduce::Split,
            _ => {}
        }

        let reduced = match &mut self.right {
            Node::Snailfish(s) => s.split(depth + 1),
            Node::Value(v) => {
                if *v >= 10 {
                    let left = Node::Value(*v / 2);
                    let right = Node::Value(if (*v) & 1 != 0 { *v / 2 + 1 } else { *v / 2 });
                    self.right = Snailfish { left, right }.into();
                    Reduce::Split
                } else {
                    Reduce::None
                }
            }
        };
        reduced
    }


    fn add_left(&mut self, value: u32) {
        match &mut self.left {
            Node::Snailfish(s) => s.add_left(value),
            Node::Value(v) => *v += value,
        }
    }

    fn add_right(&mut self, value: u32) {
        match &mut self.right {
            Node::Snailfish(s) => s.add_right(value),
            Node::Value(v) => *v += value,
        }
    }
}

#[derive(Debug)]
enum Reduce {
    Explode(u32, u32),
    Exploded(Option<u32>, Option<u32>),
    Split,
    None,
}
