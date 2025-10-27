#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Edge {
    Horizontal { y: usize, x1: usize, x2: usize },
    Vertical { x: usize, y1: usize, y2: usize },
}

use std::mem;

use Edge::*;

impl Edge {
    pub fn new_horizontal(y: usize, x1: usize, x2: usize) -> Edge {
        debug_assert!(x1 <= x2);
        Edge::Horizontal { y, x1, x2 }
    }

    pub fn new_vertical(x: usize, y1: usize, y2: usize) -> Edge {
        debug_assert!(y1 <= y2);
        Edge::Vertical { x, y1, y2 }
    }

    pub fn endpoints(&self) -> ((usize, usize), (usize, usize)) {
        match *self {
            Horizontal { y, x1, x2 } => ((x1, y), (x2, y)),
            Vertical { x, y1, y2 } => ((x, y1), (x, y2)),
        }
    }

    pub fn connected(&self, other: &Edge) -> bool {
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
    pub fn opposite(self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }

    /// Returns
    /// - `Some(true)` if moving in direction `e1` followed by `e2` is clockwise
    /// - `Some(false)` if it's counter-clockwise
    /// - `None` if it's neither
    pub(crate) const fn clockwise(e1: Direction, e2: Direction) -> Option<bool> {
        match (e1, e2) {
            (Direction::Left, Direction::Down)
            | (Direction::Right, Direction::Up)
            | (Direction::Up, Direction::Left)
            | (Direction::Down, Direction::Right) => Some(false),
            (Direction::Left, Direction::Up)
            | (Direction::Right, Direction::Down)
            | (Direction::Up, Direction::Right)
            | (Direction::Down, Direction::Left) => Some(true),
            (Direction::Left | Direction::Right, Direction::Left | Direction::Right)
            | (Direction::Up | Direction::Down, Direction::Up | Direction::Down) => None,
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
                Edge::new_horizontal(y, x_from, x_to)
            }
            DirectedEdge::Vertical { x, mut y_from, mut y_to } => {
                if y_from > y_to {
                    mem::swap(&mut y_from, &mut y_to);
                }
                Edge::new_vertical(x, y_from, y_to)
            }
        }
    }
}

impl DirectedEdge {
    pub fn from_endpoints(
        (x_from, y_from): (usize, usize),
        (x_to, y_to): (usize, usize),
    ) -> Option<DirectedEdge> {
        if x_from == x_to {
            Some(DirectedEdge::Vertical { x: x_from, y_from, y_to })
        } else if y_from == y_to {
            Some(DirectedEdge::Horizontal { y: y_from, x_from, x_to })
        } else {
            None
        }
    }

    pub fn direction(&self) -> Direction {
        match self {
            DirectedEdge::Horizontal { x_from, x_to, .. } => {
                if x_from < x_to {
                    Direction::Right
                } else {
                    Direction::Left
                }
            }
            DirectedEdge::Vertical { y_from, y_to, .. } => {
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
            DirectedEdge::Horizontal { y, x_from, .. } => (x_from, y),
            DirectedEdge::Vertical { x, y_from, .. } => (x, y_from),
        }
    }

    pub fn to(&self) -> (usize, usize) {
        match *self {
            DirectedEdge::Horizontal { y, x_to, .. } => (x_to, y),
            DirectedEdge::Vertical { x, y_to, .. } => (x, y_to),
        }
    }

    pub fn len(&self) -> usize {
        match *self {
            DirectedEdge::Horizontal { x_from, x_to, .. } => x_from.abs_diff(x_to),
            DirectedEdge::Vertical { y_from, y_to, .. } => y_from.abs_diff(y_to),
        }
    }

    pub fn combine_directed(&self, other: &DirectedEdge) -> Option<DirectedEdge> {
        let from1 = self.from();
        let to1 = self.to();
        let from2 = other.from();
        let to2 = other.to();
        if from1 == to2 {
            DirectedEdge::from_endpoints(from2, to1)
        } else if from2 == to1 {
            DirectedEdge::from_endpoints(from1, to2)
        } else {
            None
        }
    }
}
