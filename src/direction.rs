#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Edge {
    Horizontal { y: usize, x1: usize, x2: usize },
    Vertical { x: usize, y1: usize, y2: usize },
}

use std::mem;

use Edge::*;

impl Edge {
    pub fn new_horizontal(y: usize, x1: usize, x2: usize) -> Self {
        debug_assert!(x1 <= x2);
        Self::Horizontal { y, x1, x2 }
    }

    pub fn new_vertical(x: usize, y1: usize, y2: usize) -> Self {
        debug_assert!(y1 <= y2);
        Self::Vertical { x, y1, y2 }
    }

    pub fn endpoints(&self) -> ((usize, usize), (usize, usize)) {
        match *self {
            Horizontal { y, x1, x2 } => ((x1, y), (x2, y)),
            Vertical { x, y1, y2 } => ((x, y1), (x, y2)),
        }
    }

    pub fn connected(&self, other: &Self) -> bool {
        assert!(self != other);
        let (p1, p2) = self.endpoints();
        let (p3, p4) = other.endpoints();
        p1 == p3 || p1 == p4 || p2 == p3 || p2 == p4
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DirectedEdge {
    Horizontal { y: usize, x_from: usize, x_to: usize },
    Vertical { x: usize, y_from: usize, y_to: usize },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }

    /// Returns
    /// - `Some(true)` if moving in direction `e1` followed by `e2` is clockwise
    /// - `Some(false)` if it's counter-clockwise
    /// - `None` if it's neither
    pub(crate) const fn clockwise(e1: Self, e2: Self) -> Option<bool> {
        match (e1, e2) {
            (Self::Left, Self::Down)
            | (Self::Right, Self::Up)
            | (Self::Up, Self::Left)
            | (Self::Down, Self::Right) => Some(false),
            (Self::Left, Self::Up)
            | (Self::Right, Self::Down)
            | (Self::Up, Self::Right)
            | (Self::Down, Self::Left) => Some(true),
            (Self::Left | Self::Right, Self::Left | Self::Right)
            | (Self::Up | Self::Down, Self::Up | Self::Down) => None,
        }
    }
}

impl From<DirectedEdge> for Edge {
    fn from(value: DirectedEdge) -> Self {
        match value {
            DirectedEdge::Horizontal { y, mut x_from, mut x_to } => {
                if x_from > x_to {
                    mem::swap(&mut x_from, &mut x_to);
                }
                Self::new_horizontal(y, x_from, x_to)
            }
            DirectedEdge::Vertical { x, mut y_from, mut y_to } => {
                if y_from > y_to {
                    mem::swap(&mut y_from, &mut y_to);
                }
                Self::new_vertical(x, y_from, y_to)
            }
        }
    }
}

impl DirectedEdge {
    pub fn from_endpoints(
        (x_from, y_from): (usize, usize),
        (x_to, y_to): (usize, usize),
    ) -> Option<Self> {
        if x_from == x_to {
            Some(Self::Vertical { x: x_from, y_from, y_to })
        } else if y_from == y_to {
            Some(Self::Horizontal { y: y_from, x_from, x_to })
        } else {
            None
        }
    }

    pub fn direction(&self) -> Direction {
        match self {
            Self::Horizontal { x_from, x_to, .. } => {
                if x_from < x_to {
                    Direction::Right
                } else {
                    Direction::Left
                }
            }
            Self::Vertical { y_from, y_to, .. } => {
                if y_from < y_to {
                    Direction::Down
                } else {
                    Direction::Up
                }
            }
        }
    }

    pub fn from(&self) -> (usize, usize) {
        match *self {
            Self::Horizontal { y, x_from, .. } => (x_from, y),
            Self::Vertical { x, y_from, .. } => (x, y_from),
        }
    }

    pub fn to(&self) -> (usize, usize) {
        match *self {
            Self::Horizontal { y, x_to, .. } => (x_to, y),
            Self::Vertical { x, y_to, .. } => (x, y_to),
        }
    }

    pub fn len(&self) -> usize {
        match *self {
            Self::Horizontal { x_from, x_to, .. } => x_from.abs_diff(x_to),
            Self::Vertical { y_from, y_to, .. } => y_from.abs_diff(y_to),
        }
    }

    pub fn combine_directed(&self, other: &Self) -> Option<Self> {
        let from1 = self.from();
        let to1 = self.to();
        let from2 = other.from();
        let to2 = other.to();
        if from1 == to2 {
            Self::from_endpoints(from2, to1)
        } else if from2 == to1 {
            Self::from_endpoints(from1, to2)
        } else {
            None
        }
    }
}
