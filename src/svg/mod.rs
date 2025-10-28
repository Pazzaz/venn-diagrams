mod circles;
mod config;
mod corner;
mod offset;

pub use config::{CornerStyle, DiagramConfig, OffsetMethod};
use itertools::Itertools;
use svg::{
    Document,
    node::element::{Definitions, Mask, Rectangle, SVG},
};

use self::{
    circles::{Coalition, draw_circle},
    corner::{BasicCorner, Corner, Diagonal, get_rounded_paths},
    offset::InnerOffset,
};
use super::direction::{DirectedEdge, Edge};
use crate::{
    direction::Direction,
    matrix::Matrix,
    polyomino::{ConstPolyomino, Polyomino},
    svg::offset::inner_offset,
    constants::VennDiagram,
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

fn get_polys(x: usize, y: usize, polyominos: &[Polyomino]) -> Vec<Vec<Edge>> {
    let mut polys: Vec<Vec<Edge>> = Vec::new();

    for poly in polyominos {
        let mut edges: Vec<Edge> = Vec::new();

        for i in 0..x {
            for j in 0..y {
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
                    if i == (x - 1) || !poly[(i + 1, j)] {
                        edges.push(Edge::new_vertical(i + 1, j, j + 1));
                    }
                    // Down
                    if j == (y - 1) || !poly[(i, j + 1)] {
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
    x: usize,
    y: usize,
    combined_paths: &[Vec<DirectedEdge>],
    offsets: &[Vec<i32>],
    line_width: f64,
    corner_offset: f64,
) -> Vec<Vec<Corner>> {
    // We will convert to just points, with offsets applied
    let mut points: Vec<Vec<BasicCorner>> = Vec::new();
    let mut group_offsets: Vec<Vec<Option<i32>>> = Vec::new();

    let mut positioned_corners: Matrix<Vec<(usize, usize)>> = Matrix::new(x + 1, y + 1, Vec::new());

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
    for j in 0..=y {
        for i in 0..=x {
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
            let offset = offset.unwrap_or(0) as f64 * line_width + corner_offset;

            let meet_x: f64 = corner.x as f64 + corner.x_offset as f64 * line_width;
            let meet_y: f64 = corner.y as f64 + corner.y_offset as f64 * line_width;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathLayout {
    x: usize,
    y: usize,
    combined_paths: Vec<Vec<DirectedEdge>>,
    offsets: Vec<Vec<i32>>,
    polyominoes: Vec<Polyomino>,
}

#[derive(Debug, Clone)]
pub(crate) struct PathLayoutConst<const L: usize, const K: usize, const X: usize, const Y: usize> {
    pub combined_paths: [DirectedEdge; L],
    pub offset: [i32; L],
    pub parts_len: [usize; K],
    pub polyominoes: [ConstPolyomino<X, Y>; K],
}

struct PartsIterator<'a, T> {
    part: usize,
    start_i: usize,
    values: &'a [T],
    parts_len: &'a [usize],
}

fn iterate<'a, T>(values: &'a [T], parts_len: &'a [usize]) -> PartsIterator<'a, T> {
    debug_assert!(values.len() == parts_len.len());
    PartsIterator { part: 0, start_i: 0, values, parts_len }
}

impl<'a, T> Iterator for PartsIterator<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.part >= self.parts_len.len() {
            return None;
        }
        let part_len = self.parts_len[self.part];

        let next = &self.values[self.start_i..(self.start_i + part_len)];
        self.start_i += part_len;
        self.part += 1;

        Some(next)
    }
}

impl<const L: usize, const K: usize, const X: usize, const Y: usize>
    From<PathLayoutConst<L, K, X, Y>> for PathLayout
{
    fn from(value: PathLayoutConst<L, K, X, Y>) -> Self {
        let combined_paths =
            iterate(&value.combined_paths, &value.parts_len).map(|x| x.to_vec()).collect();
        let offsets = iterate(&value.offset, &value.parts_len).map(|x| x.to_vec()).collect();
        let polyominoes = value.polyominoes.into_iter().map(Into::into).collect();

        PathLayout { x: X, y: Y, combined_paths, offsets, polyominoes }
    }
}

impl PathLayout {
    pub const fn n(&self) -> usize {
        self.combined_paths.len()
    }

    pub fn from_diagram(diagram: VennDiagram, offset_method: OffsetMethod) -> Self {
        let polys = get_polys(diagram.x(), diagram.y(), &diagram.polyominos);
        let paths = get_paths(&polys);
        let combined_paths = get_combined_paths(paths);
        let offsets = offset_method.get_offsets(diagram.x(), diagram.y(), &combined_paths);

        Self {
            x: diagram.x(),
            y: diagram.y(),
            combined_paths,
            offsets,
            polyominoes: diagram.polyominos,
        }
    }

    // This function is used to generate const versions, but we store the result
    // from that so we don't actually use it
    #[allow(unused)]
    fn flattened(self) -> (Vec<DirectedEdge>, Vec<i32>, Vec<usize>) {
        let parts_len = self.combined_paths.iter().map(|x| x.len()).collect();
        let combined_paths = self.combined_paths.iter().flatten().copied().collect();
        let offsets = self.offsets.iter().flatten().copied().collect();

        (combined_paths, offsets, parts_len)
    }

    #[must_use]
    pub fn to_svg(&self, values: &[f64], colors: &[String], config: &DiagramConfig) -> SVG {
        let PathLayout { x, y, combined_paths, offsets, polyominoes } = self;
        let internal_offsets = inner_offset(*x, *y, offsets, combined_paths, config.line_width);

        let points =
            get_points(*x, *y, combined_paths, offsets, config.line_width, config.corner_offset);

        let paths = get_rounded_paths(&points, config.corner_style).unwrap();

        // Then we create the svg
        let min_x = -0.5;
        let width = (x + 1) as f64;

        let min_y = -0.5;
        let height = (y + 1) as f64;

        let mut out = Document::new().set("viewBox", (min_x, min_y, width, height));

        if let Some(width_mul) = config.width_mul {
            out = out.set("width", format!("{}px", width_mul * width));
        }

        if let Some(height_mul) = config.height_mul {
            out = out.set("height", format!("{}px", height_mul * width));
        }

        let mut mask = Mask::new().set("id", "background_mask");
        for path in &paths {
            let part = path.clone().set("fill", "white").set("stroke", "none");
            mask = mask.add(part);
        }

        let defs = Definitions::new().add(mask);

        out = out.add(defs);

        let rect = Rectangle::new()
            .set("width", width)
            .set("height", height)
            .set("x", min_x)
            .set("y", min_y)
            .set("mask", "url(#background_mask)");

        out = out.add(rect);

        for (path, color) in paths.iter().zip(colors) {
            let path = path
                .clone()
                .set("fill", color.clone())
                .set("fill-opacity", 0.2)
                .set("stroke", "none")
                .set("stroke-width", 0.05);
            out = out.add(path);
        }

        for (path, color) in paths.iter().zip(colors) {
            let path = path
                .clone()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", config.line_width);
            out = out.add(path);
        }

        let n = self.n();

        let mut pairs = vec![false; n];
        for x in 0..*x {
            for y in 0..*y {
                let mut any_true = false;
                for i in 0..n {
                    let v = polyominoes[i][(x, y)];
                    any_true |= v;
                    pairs[i] = v;
                }
                if any_true {
                    let (x_pos, y_pos) =
                        config.circle_placement.get_circle_pos(x, y, internal_offsets[(x, y)]);
                    out = draw_circle(x_pos, y_pos, &pairs, out, config, values, colors);
                }
            }
        }

        out
    }
}
