use crate::direction::Direction;

#[derive(Debug, Clone, Copy)]
pub enum CornerStyle {
    Straight,
    Smooth,
}

#[derive(Debug, Clone)]
pub(super) struct BasicCorner {
    pub(super) x: usize,
    pub(super) y: usize,
    pub(super) x_offset: i32,
    pub(super) y_offset: i32,
    pub(super) from: Direction,
    pub(super) to: Direction,
}

impl BasicCorner {
    fn offset_group(&self) -> i32 {
        match self.diagonal() {
            Diagonal::DownRight | Diagonal::UpLeft => self.u(),
            Diagonal::UpRight | Diagonal::DownLeft => self.v(),
        }
    }

    /// Opposite of `offset_group`
    pub(super) fn offset_group_2(&self) -> i32 {
        match self.diagonal() {
            Diagonal::DownRight | Diagonal::UpLeft => self.v(),
            Diagonal::UpRight | Diagonal::DownLeft => self.u(),
        }
    }

    // Diagonal coordinates
    fn u(&self) -> i32 {
        self.x_offset - self.y_offset
    }

    fn v(&self) -> i32 {
        self.x_offset + self.y_offset
    }

    fn diagonal(&self) -> Diagonal {
        match (self.from.opposite(), self.to) {
            (Direction::Left, Direction::Up) | (Direction::Up, Direction::Left) => Diagonal::UpLeft,
            (Direction::Up, Direction::Right) | (Direction::Right, Direction::Up) => {
                Diagonal::UpRight
            }
            (Direction::Left, Direction::Down) | (Direction::Down, Direction::Left) => {
                Diagonal::DownLeft
            }
            (Direction::Right, Direction::Down) | (Direction::Down, Direction::Right) => {
                Diagonal::DownRight
            }
            (Direction::Left | Direction::Right, Direction::Left | Direction::Right) => {
                unreachable!()
            }
            (Direction::Up | Direction::Down, Direction::Up | Direction::Down) => unreachable!(),
        }
    }

    pub(super) fn group_category(&self) -> (i32, Diagonal) {
        (self.offset_group(), self.diagonal())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Diagonal {
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

pub(super) struct Corner {
    pub(super) from: (f64, f64),
    pub(super) to: (f64, f64),
    pub(super) clockwise: bool,
    pub(super) radius: f64,
}

impl Corner {
    // Parameters which can be used to create an "elliptical_arc" in SVG
    pub(super) fn params(&self) -> (f64, f64, i32, i32, i32, f64, f64) {
        (self.radius, self.radius, 0, 0, i32::from(self.clockwise), self.to.0, self.to.1)
    }
}
