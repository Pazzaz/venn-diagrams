//! Drawing Venn diagrams as SVGs.

mod circles;
mod config;
mod corner;
mod offset;

mod layout;

pub use config::{CornerStyle, DiagramConfig};
use itertools::Itertools;
pub use layout::{Layout, LayoutConst};

use self::{
    circles::{Coalition, draw_circle},
    corner::{BasicCorner, Corner, Diagonal, get_rounded_paths},
    offset::InnerOffset,
};
use super::direction::{DirectedEdge, Edge};
use crate::{
    direction::Direction, matrix::Matrix, polyomino::Polyomino, svg::offset::inner_offset,
};

fn get_combined_paths(paths: Vec<Vec<DirectedEdge>>) -> Vec<Vec<DirectedEdge>> {
    let mut combined_paths: Vec<Vec<DirectedEdge>> = Vec::new();
    for path in paths {
        let mut out: Vec<DirectedEdge> = Vec::new();
        let mut current: Option<DirectedEdge> = None;
        for edge in path {
            current = match current {
                Some(current_edge) => current_edge.combine_directed(&edge).or_else(|| {
                    out.push(current_edge);
                    Some(edge)
                }),
                None => Some(edge),
            };
        }

        if let Some(current) = current {
            if let Some(combined) = out[0].combine_directed(&current) {
                out[0] = combined;
            } else {
                out.push(current);
            }
        }

        combined_paths.push(out);
    }
    combined_paths
}

fn get_paths(polys: &[Vec<Edge>]) -> Vec<Vec<DirectedEdge>> {
    let mut paths: Vec<Vec<DirectedEdge>> = Vec::new();

    for edges in polys {
        // 1. Create adjancy matrix
        let l = edges.len();
        let mut adj = Matrix::new(l, l, false);

        for i in 0..l {
            for j in 0..l {
                if i == j {
                    continue;
                }
                if edges[i].connected(&edges[j]) {
                    adj[(j, i)] = true;
                }
            }
        }

        let mut path: Vec<DirectedEdge> = Vec::new();

        // current edge we're examining
        let mut i: usize = 0;
        let mut pre: Option<(usize, usize)> = None;
        while let Some(j) = adj.row(i).iter().position(|x| *x) {
            for k in 0..l {
                adj[(k, i)] = false;
                adj[(i, k)] = false;
            }

            let directed: Option<DirectedEdge>;
            (directed, pre) = if let Some(op) = pre {
                let edge = &edges[i];
                let (e1, e2) = edge.endpoints();
                if op == e1 {
                    (DirectedEdge::from_endpoints(e1, e2), Some(e2))
                } else if op == e2 {
                    (DirectedEdge::from_endpoints(e2, e1), Some(e1))
                } else {
                    unreachable!();
                }
            } else {
                let edge1 = &edges[i];
                let edge2 = &edges[j];
                let (a1, a2) = edge1.endpoints();
                let (b1, b2) = edge2.endpoints();
                if a1 == b1 || a1 == b2 {
                    (DirectedEdge::from_endpoints(a2, a1), Some(a1))
                } else if a2 == b1 || a2 == b2 {
                    (DirectedEdge::from_endpoints(a1, a2), Some(a2))
                } else {
                    unreachable!();
                }
            };

            path.push(directed.unwrap());
            i = j;
        }
        let directed = if let Some(op) = pre {
            let edge = &edges[i];
            let (e1, e2) = edge.endpoints();
            if op == e1 {
                DirectedEdge::from_endpoints(e1, e2)
            } else if op == e2 {
                DirectedEdge::from_endpoints(e2, e1)
            } else {
                unreachable!();
            }
        } else {
            unreachable!();
        };

        path.push(directed.unwrap());

        // I don't think we need to handle holes or disjoint yet
        // Let's just check we used every edge in this path
        for i in 0..l {
            for j in 0..l {
                debug_assert!(!adj[(j, i)]);
            }
        }
        paths.push(path);
    }
    paths
}

fn get_polys(width: usize, height: usize, polyominos: &[Polyomino]) -> Vec<Vec<Edge>> {
    let mut polys: Vec<Vec<Edge>> = Vec::new();

    for poly in polyominos {
        let mut edges: Vec<Edge> = Vec::new();

        for i in 0..width {
            for j in 0..height {
                if poly[(i, j)] {
                    // Left
                    if i == 0 || !poly[(i - 1, j)] {
                        edges.push(Edge::new_vertical(i, j, j + 1));
                    }
                    // Up
                    if j == 0 || !poly[(i, j - 1)] {
                        edges.push(Edge::new_horizontal(j, i, i + 1));
                    }
                    // Right
                    if i == (width - 1) || !poly[(i + 1, j)] {
                        edges.push(Edge::new_vertical(i + 1, j, j + 1));
                    }
                    // Down
                    if j == (height - 1) || !poly[(i, j + 1)] {
                        edges.push(Edge::new_horizontal(j + 1, i, i + 1));
                    }
                }
            }
        }

        polys.push(edges);
    }
    polys
}

fn get_points(
    width: usize,
    height: usize,
    combined_paths: &[Vec<DirectedEdge>],
    offsets: &[Vec<i32>],
    line_width: f64,
    corner_offset: f64,
) -> Vec<Vec<Corner>> {
    // We will convert to just points, with offsets applied
    let mut points: Vec<Vec<BasicCorner>> = Vec::new();
    let mut group_offsets: Vec<Vec<Option<i32>>> = Vec::new();

    let mut positioned_corners: Matrix<Vec<(usize, usize)>> =
        Matrix::new(width + 1, height + 1, Vec::new());

    for (i, (path_edges, path_offsets)) in combined_paths.iter().zip(offsets).enumerate() {
        let mut out = Vec::new();
        let mut path_group_offsets = Vec::new();
        let last_edge = path_edges.last().unwrap();
        let last_offset = path_offsets.last().unwrap();

        let path_edges = std::iter::once(last_edge).chain(path_edges);
        let path_offsets = std::iter::once(last_offset).chain(path_offsets);
        let parts: Vec<(&DirectedEdge, &i32)> = path_edges.zip(path_offsets).collect();
        for (j, aa) in parts.windows(2).enumerate() {
            let ((e1, o1), (e2, o2)) = (&aa[0], &aa[1]);
            let (shared_x, shared_y) = e1.to();
            debug_assert!(e1.to() == e2.from());

            let (ox, oy) = match e1 {
                DirectedEdge::Horizontal { .. } => (o2, o1),
                DirectedEdge::Vertical { .. } => (o1, o2),
            };

            let corner = BasicCorner {
                x: shared_x,
                y: shared_y,
                x_offset: **ox,
                y_offset: **oy,
                from: e1.direction(),
                to: e2.direction(),
            };

            // We store them in the order of the path
            out.push(corner);

            // We also store the corsers grouped by their x and y position
            positioned_corners[(shared_x, shared_y)].push((i, j));

            // We'll fill these in later
            path_group_offsets.push(None);
        }
        points.push(out);
        group_offsets.push(path_group_offsets);
    }

    // Then we look at each corner and align the turns
    for j in 0..=height {
        for i in 0..=width {
            // Sort corners according to their category
            positioned_corners[(i, j)].sort_by_key(|(i, j)| points[*i][*j].group_category());
            // Group them according to their position
            for ((_, diag), chunk) in &positioned_corners[(i, j)]
                .iter()
                .chunk_by(|(i, j)| points[*i][*j].group_category())
            {
                let values: Vec<(usize, usize)> = chunk.cloned().collect();
                if values.len() == 1 {
                    continue;
                }

                // We invert some coordinates in two of the quadrants
                let invert = matches!(diag, Diagonal::UpLeft | Diagonal::DownLeft);

                let (p, q) = values
                    .iter()
                    .min_by_key(|(p, q)| {
                        let v = points[*p][*q].offset_group_2();
                        if invert { v } else { -v }
                    })
                    .unwrap();
                let default_pos = points[*p][*q].offset_group_2();

                for (i, j) in &values {
                    let corner_pos = points[*i][*j].offset_group_2();
                    let offset = corner_pos - default_pos;
                    debug_assert!(offset % 2 == 0);
                    group_offsets[*i][*j] =
                        if invert { Some(offset / 2) } else { Some(-(offset / 2)) }
                }
            }
        }
    }

    let mut other_points = Vec::new();
    for (path, path_offsets) in points.iter().zip(&group_offsets) {
        let mut other_out = Vec::new();
        for (corner, offset) in path.iter().zip(path_offsets) {
            let offset = (offset.unwrap_or(0) as f64).mul_add(line_width, corner_offset);

            let meet_x: f64 = (corner.x_offset as f64).mul_add(line_width, corner.x as f64);
            let meet_y: f64 = (corner.y_offset as f64).mul_add(line_width, corner.y as f64);

            let from: (f64, f64) = match corner.from {
                Direction::Left => (meet_x + offset, meet_y),
                Direction::Right => (meet_x - offset, meet_y),
                Direction::Up => (meet_x, meet_y + offset),
                Direction::Down => (meet_x, meet_y - offset),
            };

            let to: (f64, f64) = match corner.to {
                Direction::Left => (meet_x - offset, meet_y),
                Direction::Right => (meet_x + offset, meet_y),
                Direction::Up => (meet_x, meet_y - offset),
                Direction::Down => (meet_x, meet_y + offset),
            };

            // We're moving clockwise or counter-clockwise
            let clockwise = Direction::clockwise(corner.from, corner.to).unwrap();

            let corner = Corner { from, to, clockwise, radius: offset };
            other_out.push(corner);
        }
        other_points.push(other_out);
    }
    other_points
}
