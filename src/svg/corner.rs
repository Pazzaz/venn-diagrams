use svg::node::element::{Path, path::Data};

use crate::direction::Direction;

/// How each corner of a polyomino border should be drawn.
#[derive(Debug, Clone, Copy)]
pub enum CornerStyle {
    /// Each corner consists of two 45° turns
    Straight,

    /// Each corner is circular; a quarter turn.
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
        Diagonal::from_directions(self.from.opposite(), self.to).unwrap()
    }

    pub(super) fn group_category(&self) -> (i32, Diagonal) {
        (self.offset_group(), self.diagonal())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub(super) enum Diagonal {
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Diagonal {
    pub(super) fn from_directions(a: Direction, b: Direction) -> Option<Self> {
        match (a, b) {
            (Direction::Left, Direction::Up) | (Direction::Up, Direction::Left) => {
                Some(Self::UpLeft)
            }
            (Direction::Up, Direction::Right) | (Direction::Right, Direction::Up) => {
                Some(Self::UpRight)
            }
            (Direction::Left, Direction::Down) | (Direction::Down, Direction::Left) => {
                Some(Self::DownLeft)
            }
            (Direction::Right, Direction::Down) | (Direction::Down, Direction::Right) => {
                Some(Self::DownRight)
            }
            (Direction::Left | Direction::Right, Direction::Left | Direction::Right)
            | (Direction::Up | Direction::Down, Direction::Up | Direction::Down) => None,
        }
    }
}

#[cfg(feature = "optimize")]
impl Diagonal {
    pub(super) fn down(self) -> bool {
        match self {
            Self::UpLeft | Self::UpRight => false,
            Self::DownLeft | Self::DownRight => true,
        }
    }

    pub(super) fn right(self) -> bool {
        match self {
            Self::UpRight | Self::DownRight => true,
            Self::UpLeft | Self::DownLeft => false,
        }
    }
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

/// Combine corners into an SVG path. Returns `None` if `corners` is empty.
fn get_rounded_path(corners: &[Corner], corner_style: CornerStyle) -> Option<Path> {
    corners.split_first().map(|(first, rest)| {
        let mut data = Data::new().move_to(first.from);
        match corner_style {
            CornerStyle::Straight => {
                data = data.line_to(first.to);
                for corner in rest {
                    data = data.line_to(corner.from).line_to(corner.to);
                }
            }
            CornerStyle::Smooth => {
                data = data.elliptical_arc_to(first.params());
                for corner in rest {
                    data = data.line_to(corner.from).elliptical_arc_to(corner.params());
                }
            }
        }
        data = data.close();
        Path::new().set("d", data)
    })
}

pub(super) fn get_rounded_paths(
    corners: &[Vec<Corner>],
    corner_style: CornerStyle,
) -> Option<Vec<Path>> {
    corners.iter().map(|v| get_rounded_path(v, corner_style)).collect()
}
