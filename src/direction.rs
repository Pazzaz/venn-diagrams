#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Direction {
    Horizontal { y: usize, x1: usize, x2: usize },
    Vertical { x: usize, y1: usize, y2: usize },
}

use Direction::*;

impl Direction {
    pub fn len(&self) -> usize {
        match self {
            &Horizontal { x1, x2, .. } => x1.abs_diff(x2),
            &Vertical { y1, y2, .. } => y1.abs_diff(y2),
        }
    }
    pub fn endpoints(&self) -> ((usize, usize), (usize, usize)) {
        match self {
            &Horizontal { y, x1, x2 } => ((x1, y), (x2, y)),
            &Vertical { x, y1, y2 } => ((x, y1), (x, y2)),
        }
    }

    pub fn from_endpoints((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> Direction {
        if x1 == x2 {
            Vertical { x: x1, y1, y2 }
        } else if y1 == y2 {
            Horizontal { y: y1, x1, x2 }
        } else {
            panic!("Invalid endpoints");
        }
    }

    pub fn connected(&self, other: &Direction) -> bool {
        assert!(self != other);
        let (p1, p2) = self.endpoints();
        let (p3, p4) = other.endpoints();
        p1 == p3 || p1 == p4 || p2 == p3 || p2 == p4
    }

    pub fn combine(&self, other: &Direction) -> Option<Direction> {
        if !self.connected(other) {
            return None;
        }

        match (self, other) {
            (&Horizontal { x1, x2, y }, &Horizontal { x1: x3, x2: x4, .. }) => {
                Some(Horizontal { y, x1: x1.min(x3), x2: x2.max(x4) })
            }
            (Horizontal { .. }, Vertical { .. }) => None,
            (Vertical { .. }, Horizontal { .. }) => None,
            (&Vertical { x, y1, y2 }, &Vertical { y1: y3, y2: y4, .. }) => {
                Some(Vertical { x, y1: y1.min(y3), y2: y2.max(y4) })
            }
        }
    }
}
