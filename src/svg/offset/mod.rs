mod greedy;
mod optimizing;

use crate::{
    direction::{DirectedEdge, Direction, Edge},
    matrix::Matrix,
    svg::{PathLayout, get_combined_paths, get_paths, get_polys},
    venn_diagram::VennDiagram,
};

#[derive(Debug, Default, Clone, Copy)]
pub(super) struct InnerOffset {
    pub(super) above: f64,
    pub(super) below: f64,
    pub(super) right: f64,
    pub(super) left: f64,
}

#[derive(Debug, Default, Clone)]
struct EdgeInfo {
    from: usize,
    to: usize,
    direction: Option<Direction>,
    len: usize,
    p_i: usize,
    e_i: usize,
}

pub(super) fn inner_offset(
    width: usize,
    height: usize,
    path_offsets: &[Vec<i32>],
    combined_paths: &[Vec<DirectedEdge>],
    line_width: f64,
) -> Matrix<InnerOffset> {
    let min_offset: InnerOffset =
        InnerOffset { above: f64::MIN, below: f64::MIN, right: f64::MIN, left: f64::MIN };
    let mut inner_offset: Matrix<InnerOffset> = Matrix::new(width, height, min_offset);

    for (path, offsets) in combined_paths.iter().zip(path_offsets) {
        for (&edge, offset) in path.iter().zip(offsets) {
            let offset = *offset as f64 * line_width;
            match edge.into() {
                Edge::Horizontal { y, x1, x2 } => {
                    for i in x1..x2 {
                        if y != 0 {
                            let box_above = &mut inner_offset[(i, y - 1)];
                            if -offset > box_above.below {
                                box_above.below = -offset;
                            }
                        }
                        if y != height {
                            let box_below = &mut inner_offset[(i, y)];
                            if offset > box_below.above {
                                box_below.above = offset;
                            }
                        }
                    }
                }
                Edge::Vertical { x, y1, y2 } => {
                    for j in y1..y2 {
                        if x != 0 {
                            let box_left = &mut inner_offset[(x - 1, j)];
                            if -offset > box_left.right {
                                box_left.right = -offset;
                            }
                        }
                        if x != width {
                            let box_right = &mut inner_offset[(x, j)];
                            if offset > box_right.left {
                                box_right.left = offset;
                            }
                        }
                    }
                }
            }
        }
    }

    inner_offset
}

impl VennDiagram {
    /// Decide offsets greedily, placing larger edges before smaller edges.
    /// Positions are calculated seperately for each column and each row.
    pub fn layout_greedy(self) -> PathLayout {
        let polys = get_polys(self.width(), self.height(), &self.polyominos);
        let paths = get_paths(&polys);
        let combined_paths = get_combined_paths(paths);
        let offsets = greedy::get_offsets(self.width(), self.height(), &combined_paths);

        PathLayout {
            width: self.width(),
            height: self.height(),
            combined_paths,
            offsets,
            diagram: self,
        }
    }

    /// Decide offsets by optimization, minimizing edge overlaps and gaps.
    #[cfg(feature = "optimize")]
    pub fn layout_optimize(self) -> PathLayout {
        let polys = get_polys(self.width(), self.height(), &self.polyominos);
        let paths = get_paths(&polys);
        let combined_paths = get_combined_paths(paths);
        let offsets = optimizing::get_offsets(self.width(), self.height(), &combined_paths);

        PathLayout {
            width: self.width(),
            height: self.height(),
            combined_paths,
            offsets,
            diagram: self,
        }
    }
}
