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

    pub fn connected(&self, other: &Edge) -> bool {
        assert!(self != other);
        let (p1, p2) = self.endpoints();
        let (p3, p4) = other.endpoints();
        p1 == p3 || p1 == p4 || p2 == p3 || p2 == p4
    }

    pub fn combine(&self, other: &Edge) -> Option<Edge> {
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

pub enum DirectedEdge {
    Horizontal { y: usize, x_from: usize, x_to: usize },
    Vertical { x: usize, y_from: usize, y_to: usize },
}

impl DirectedEdge {
    pub fn from_endpoints(
        (x_from, y_from): (usize, usize),
        (x_to, y_to): (usize, usize),
    ) -> DirectedEdge {
        if x_from == x_to {
            DirectedEdge::Vertical { x: x_from, y_from, y_to }
        } else if y_from == y_to {
            DirectedEdge::Horizontal { y: y_from, x_from, x_to }
        } else {
            panic!("Invalid endpoints");
        }
    }

    pub fn from(&self) -> (usize, usize) {
        match self {
            &DirectedEdge::Horizontal { y, x_from, .. } => (x_from, y),
            &DirectedEdge::Vertical { x, y_from, .. } => (x, y_from),
        }
    }

    pub fn to(&self) -> (usize, usize) {
        match self {
            &DirectedEdge::Horizontal { y, x_to, .. } => (x_to, y),
            &DirectedEdge::Vertical { x, y_to, .. } => (x, y_to),
        }
    }
}
