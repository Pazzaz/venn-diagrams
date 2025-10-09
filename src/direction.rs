#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Edge {
    Horizontal { y: usize, x1: usize, x2: usize },
    Vertical { x: usize, y1: usize, y2: usize },
}

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

#[derive(Debug, PartialEq, Eq, Clone)]
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
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
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

    pub fn combine(&self, other: &Edge) -> Option<DirectedEdge> {
        let from = self.from();
        let to = self.to();

        let (o1, o2) = other.endpoints();
        if o1 == from {
            Self::from_endpoints(o2, to)
        } else if o1 == to {
            Self::from_endpoints(from, o2)
        } else if o2 == from {
            Self::from_endpoints(o1, to)
        } else if o2 == to {
            Self::from_endpoints(from, o1)
        } else {
            None
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
