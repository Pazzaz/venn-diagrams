use std::{cmp::Ordering, mem, vec};

use itertools::Itertools;
use svg::{
    Document,
    node::element::{Circle, Definitions, Group, Mask, Path, Rectangle, SVG, path::Data},
};

use super::direction::{DirectedEdge, Edge};
use crate::{direction::Direction, matrix::Matrix, polyomino::Polyomino, venn::VennDiagram};

const SCALE: f64 = 20.0;

pub struct Diagram {}

pub struct DiagramConfig {
    pub line_width: f64,
    pub radius: f64,
    pub circle_below: CircleConfig,
    pub circle_edge: CircleConfig,
    pub circle_above: CircleConfig,
    pub circle_placement: CirclePlacement,
    pub corner_style: CornerStyle,
    pub corner_offset: f64,
    pub width_mul: Option<f64>,
    pub height_mul: Option<f64>,
}

impl Default for DiagramConfig {
    fn default() -> Self {
        Self {
            line_width: 1.0,
            radius: 3.5,
            circle_below: CircleConfig::new(0.3, String::from("red")),
            circle_edge: CircleConfig::new(1.0, String::from("white")),
            circle_above: CircleConfig::new(0.3, String::from("green")),
            circle_placement: CirclePlacement::SquareCenter,
            corner_style: CornerStyle::Smooth,
            corner_offset: 3.0,
            width_mul: Some(4.0),
            height_mul: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CircleConfig {
    opacity: f64,
    color: String,
}

impl CircleConfig {
    pub fn new(opacity: f64, color: String) -> Self {
        Self { opacity, color }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CornerStyle {
    Straight,
    Smooth,
}

pub enum CirclePlacement {
    Basic,
    SquareCenter,
}

impl CirclePlacement {
    fn get_circle_pos(&self, x: usize, y: usize, internal_offset: InnerOffset) -> (f64, f64) {
        match self {
            CirclePlacement::Basic => {
                ((x as f64 * SCALE) + SCALE / 2.0, (y as f64 * SCALE) + SCALE / 2.0)
            }
            CirclePlacement::SquareCenter => {
                let cx = x as f64 * SCALE;
                let cy = y as f64 * SCALE;

                let above_y = cy + internal_offset.above;
                let below_y = cy + SCALE + internal_offset.below;
                let left_x = cx + internal_offset.left;
                let right_x = cx + SCALE + internal_offset.right;

                let cy = f64::midpoint(above_y, below_y);
                let cx = f64::midpoint(left_x, right_x);
                (cx, cy)
            }
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct InnerOffset {
    above: f64,
    below: f64,
    right: f64,
    left: f64,
}

#[derive(Debug, Clone)]
struct BasicCorner {
    x: usize,
    y: usize,
    x_offset: i32,
    y_offset: i32,
    from: Direction,
    to: Direction,
}

impl BasicCorner {
    fn offset_group(&self) -> i32 {
        match self.diagonal() {
            Diagonal::DownRight | Diagonal::UpLeft => self.u(),
            Diagonal::UpRight | Diagonal::DownLeft => self.v(),
        }
    }

    /// Opposite of `offset_group`
    fn offset_group_2(&self) -> i32 {
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

    fn group_category(&self) -> (i32, Diagonal) {
        (self.offset_group(), self.diagonal())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Diagonal {
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

struct Corner {
    from: (f64, f64),
    to: (f64, f64),
    clockwise: bool,
    radius: f64,
}

impl Corner {
    // Parameters which can be used to create an "elliptical_arc" in SVG
    fn params(&self) -> (f64, f64, i32, i32, i32, f64, f64) {
        (self.radius, self.radius, 0, 0, i32::from(self.clockwise), self.to.0, self.to.1)
    }
}

impl Diagram {
    fn get_offsets(
        x: usize,
        y: usize,
        combined_paths: &[Vec<DirectedEdge>],
        line_width: f64,
    ) -> (Vec<Vec<i32>>, Matrix<InnerOffset>) {
        let mut offsets: Vec<Vec<i32>> =
            combined_paths.iter().map(|x| vec![i32::MIN; x.len()]).collect();
        let mut columns = vec![Vec::new(); x + 1];
        let mut rows = vec![Vec::new(); y + 1];

        let mut inner_offset: Matrix<InnerOffset> = Matrix::new(x, y, InnerOffset::default());

        for (p_i, es) in combined_paths.iter().enumerate() {
            for (e_i, e) in es.iter().enumerate() {
                match *e {
                    DirectedEdge::Horizontal { y, .. } => rows[y].push((p_i, e_i)),
                    DirectedEdge::Vertical { x, .. } => columns[x].push((p_i, e_i)),
                }
            }
        }

        let mut directions: Vec<Vec<Option<Direction>>> = Vec::new();

        for path in combined_paths {
            let mut path_directions: Vec<Option<Direction>> = Vec::new();
            let mut edges_extra: Vec<Direction> = Vec::with_capacity(path.len() + 2);
            edges_extra.push(path.last().unwrap().direction());
            edges_extra.extend(path.iter().map(DirectedEdge::direction));
            edges_extra.push(path.last().unwrap().direction());

            for window in edges_extra.windows(3) {
                if let [e0, _, e2] = window {
                    let res = if *e0 == e2.opposite() { Some(*e2) } else { None };
                    path_directions.push(res);
                }
            }

            directions.push(path_directions);
        }

        // We choose the position in each column seperately
        for i in 0..=x {
            // We sort each edge that's contained is this column such that we start by
            // placing the longest edges
            columns[i].sort_by(|a, b| {
                let ap = combined_paths[a.0][a.1].len();
                let bp = combined_paths[b.0][b.1].len();
                ap.cmp(&bp).reverse()
            });

            // the current column
            let column = &columns[i];

            let l = column.len();

            let middle = l / 2;

            let mut occupied = Matrix::new(l, y, false);

            for &(p_i, e_i) in column {
                let edge = &combined_paths[p_i][e_i];
                let edge_direction = directions[p_i][e_i];
                if let &DirectedEdge::Vertical { mut y_from, mut y_to, .. } = edge {
                    if y_to < y_from {
                        mem::swap(&mut y_from, &mut y_to);
                    }
                    debug_assert!(y_from < y_to);

                    let first_possible_left =
                        (0..=middle).rev().find(|j| !(y_from..y_to).any(|i| occupied[(*j, i)]));
                    let first_possible_right =
                        (middle..l).find(|j| !(y_from..y_to).any(|i| occupied[(*j, i)]));

                    let j = match (first_possible_left, first_possible_right) {
                        (None, None) => unreachable!(),
                        (None, Some(r)) => r,
                        (Some(l), None) => l,
                        (Some(l), Some(r)) => {
                            let prioritize_left = match edge_direction {
                                Some(Direction::Left) => true,
                                Some(Direction::Right) | None => false,
                                Some(Direction::Up | Direction::Down) => unreachable!(),
                            };

                            let left_dist = middle.abs_diff(l);
                            let right_dist = middle.abs_diff(r);

                            let choose_left = match left_dist.cmp(&right_dist) {
                                Ordering::Less => true,
                                Ordering::Equal => prioritize_left,
                                Ordering::Greater => false,
                            };

                            if choose_left { l } else { r }
                        }
                    };

                    for i in y_from..y_to {
                        debug_assert!(!occupied[(j, i)]);
                        occupied[(j, i)] = true;
                    }
                    offsets[p_i][e_i] = (j as i32) - middle as i32;
                } else {
                    unreachable!();
                }
            }

            for j in 0..y {
                let mut min_pos: usize = usize::MAX;
                let mut max_pos: usize = usize::MIN;
                for k in 0..l {
                    if occupied[(k, j)] {
                        if k < min_pos {
                            min_pos = k;
                        }
                        if k > max_pos {
                            max_pos = k;
                        }
                    }
                }
                if i != x {
                    inner_offset[(i, j)].left = (max_pos as f64 - middle as f64) * line_width;
                }
                if i != 0 {
                    inner_offset[(i - 1, j)].right = (min_pos as f64 - middle as f64) * line_width;
                }
            }
        }

        // We choose the position in each row seperately
        for i in 0..=y {
            // We sort each edge that's contained is this row such that we start by
            // placing the longest edges
            rows[i].sort_by(|a, b| {
                let ap = combined_paths[a.0][a.1].len();
                let bp = combined_paths[b.0][b.1].len();
                ap.cmp(&bp).reverse()
            });

            let row = &rows[i];

            let l = row.len();

            let middle = l / 2;

            let mut occupied = Matrix::new(l, x, false);

            for &(p_i, e_i) in row {
                let edge = &combined_paths[p_i][e_i];
                let edge_direction = directions[p_i][e_i];
                if let &DirectedEdge::Horizontal { mut x_from, mut x_to, .. } = edge {
                    if x_to < x_from {
                        mem::swap(&mut x_from, &mut x_to);
                    }
                    debug_assert!(x_from < x_to);

                    let first_possible_left =
                        (0..=middle).rev().find(|j| !(x_from..x_to).any(|i| occupied[(*j, i)]));
                    let first_possible_right =
                        (middle..l).find(|j| !(x_from..x_to).any(|i| occupied[(*j, i)]));

                    let j = match (first_possible_left, first_possible_right) {
                        (None, None) => unreachable!(),
                        (None, Some(r)) => r,
                        (Some(l), None) => l,
                        (Some(l), Some(r)) => {
                            let prioritize_left = match edge_direction {
                                Some(Direction::Up) => true,
                                Some(Direction::Down) | None => false,
                                Some(Direction::Left | Direction::Right) => unreachable!(),
                            };

                            let left_dist = middle.abs_diff(l);
                            let right_dist = middle.abs_diff(r);

                            let choose_left = match left_dist.cmp(&right_dist) {
                                Ordering::Less => true,
                                Ordering::Equal => prioritize_left,
                                Ordering::Greater => false,
                            };

                            if choose_left { l } else { r }
                        }
                    };

                    for i in x_from..x_to {
                        debug_assert!(!occupied[(j, i)]);
                        occupied[(j, i)] = true;
                    }
                    offsets[p_i][e_i] = (j as i32) - middle as i32;
                } else {
                    unreachable!();
                }
            }

            for j in 0..x {
                let mut min_pos: usize = usize::MAX;
                let mut max_pos: usize = usize::MIN;
                for k in 0..l {
                    if occupied[(k, j)] {
                        if k < min_pos {
                            min_pos = k;
                        }
                        if k > max_pos {
                            max_pos = k;
                        }
                    }
                }
                if i != y {
                    inner_offset[(j, i)].above = (max_pos as f64 - middle as f64) * line_width;
                }
                if i != 0 {
                    inner_offset[(j, i - 1)].below = (min_pos as f64 - middle as f64) * line_width;
                }
            }
        }

        (offsets, inner_offset)
    }

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
        combined_paths: Vec<Vec<DirectedEdge>>,
        offsets: Vec<Vec<i32>>,
        line_width: f64,
        corner_offset: f64,
    ) -> Vec<Vec<Corner>> {
        // We will convert to just points, with offsets applied
        let mut points: Vec<Vec<BasicCorner>> = Vec::new();
        let mut group_offsets: Vec<Vec<Option<i32>>> = Vec::new();

        let mut positioned_corners: Matrix<Vec<(usize, usize)>> =
            Matrix::new(x + 1, y + 1, Vec::new());

        for (i, (path_edges, path_offsets)) in combined_paths.into_iter().zip(offsets).enumerate() {
            let mut out = Vec::new();
            let mut path_group_offsets = Vec::new();
            let last_edge = path_edges.last().unwrap();
            let last_offset = *path_offsets.last().unwrap();

            let path_edges = std::iter::once(last_edge).chain(&path_edges);
            let path_offsets = std::iter::once(last_offset).chain(path_offsets);
            let parts: Vec<(&DirectedEdge, i32)> = path_edges.zip(path_offsets).collect();
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
                    x_offset: *ox,
                    y_offset: *oy,
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
        for j in 0..y {
            for i in 0..x {
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
                let corner_offset = corner_offset * (SCALE / 20.0);

                let offset = offset.unwrap_or(0) as f64 * line_width + corner_offset;

                let meet_x: f64 = (corner.x as f64 * SCALE) + corner.x_offset as f64 * line_width;
                let meet_y: f64 = (corner.y as f64 * SCALE) + corner.y_offset as f64 * line_width;

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

    fn get_rounded_paths(points: Vec<Vec<Corner>>, corner_style: CornerStyle) -> Vec<Path> {
        let mut paths = Vec::new();

        match corner_style {
            CornerStyle::Straight => {
                for corners in &points {
                    let first = &corners[0];
                    let mut data = Data::new().move_to(first.from);
                    data = data.line_to(first.to);

                    for corner in &corners[1..] {
                        data = data.line_to(corner.from).line_to(corner.to);
                    }
                    data = data.close();
                    let path = Path::new().set("d", data);
                    paths.push(path);
                }
            }
            CornerStyle::Smooth => {
                for corners in &points {
                    if let Some((first, rest)) = corners.split_first() {
                        let mut data =
                            Data::new().move_to(first.from).elliptical_arc_to(first.params());
                        for corner in rest {
                            data = data.line_to(corner.from).elliptical_arc_to(corner.params());
                        }
                        data = data.close();
                        let path = Path::new().set("d", data);
                        paths.push(path);
                    } else {
                        unreachable!();
                    }
                }
            }
        }

        paths
    }

    #[must_use]
    pub fn to_svg(
        venn_diagram: &VennDiagram,
        values: &[f64],
        colors: &[String],
        config: &mut DiagramConfig,
    ) -> SVG {
        config.line_width *= SCALE / 20.0;
        let x = venn_diagram.x();
        let y = venn_diagram.y();
        // First we do calculations
        let polys = Self::get_polys(x, y, &venn_diagram.polyominos);
        let paths = Self::get_paths(&polys);

        let combined_paths = Self::get_combined_paths(paths);

        let (offsets, internal_offsets) =
            Self::get_offsets(x, y, &combined_paths, config.line_width);

        let points = Self::get_points(
            x,
            y,
            combined_paths,
            offsets,
            config.line_width,
            config.corner_offset,
        );

        let paths = Self::get_rounded_paths(points, config.corner_style);

        // Then we create the svg
        let min_x = -SCALE / 2.0;
        let width = (x + 1) as f64 * SCALE;

        let min_y = -SCALE / 2.0;
        let height = (y + 1) as f64 * SCALE;

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
                .set("stroke-width", 1.0 * (SCALE / 20.0));
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

        let n = venn_diagram.n();

        let mut pairs = vec![false; n];
        for x in 0..x {
            for y in 0..y {
                let mut any_true = false;
                for i in 0..n {
                    let v = venn_diagram.polyominos[i][(x, y)];
                    any_true |= v;
                    pairs[i] = v;
                }
                if any_true {
                    let (x_pos, y_pos) =
                        config.circle_placement.get_circle_pos(x, y, internal_offsets[(x, y)]);
                    out = Self::draw_circle(x_pos, y_pos, &pairs, out, config, values, colors);
                }
            }
        }
        out
    }

    fn draw_circle(
        cx: f64,
        cy: f64,
        mask: &[bool],
        out: SVG,
        config: &DiagramConfig,
        values: &[f64],
        colors: &[String],
    ) -> SVG {
        let n = mask.len();
        debug_assert!(values.len() == n && colors.len() == n);
        let radius = config.radius * (SCALE / 20.0);
        let c = std::f64::consts::TAU * radius;
        let mut group = Group::new().set("transform", format!("rotate(-90 {cx} {cy})"));

        let coalition: Coalition = Coalition::from_values(mask, values);

        let mut added = 0.0;
        for i in 0..n {
            if !mask[i] {
                continue;
            }
            let size = values[i];
            let color = &colors[i];

            let mut circle = Circle::new()
                .set("r", radius)
                .set("cx", cx)
                .set("cy", cy)
                .set("fill", "transparent")
                .set("stroke", color.as_str())
                .set("stroke-width", radius * 2.0)
                .set("stroke-dasharray", format!("{}, {}", c * size, c));
            if added != 0.0 {
                circle = circle.set("stroke-dashoffset", -added);
            }

            added += c * size;
            group = group.add(circle);
        }

        let circle_config = config.circle_config(coalition);

        let circle = Circle::new()
            .set("r", radius * 2.0)
            .set("cx", cx)
            .set("cy", cy)
            .set("fill", "transparent")
            .set("stroke", circle_config.color.as_str())
            .set("stroke-width", 0.5 * (SCALE / 20.0));

        if circle_config.opacity != 1.0 {
            group = group.set("opacity", circle_config.opacity);
        }
        group = group.add(circle);

        out.add(group)
    }
}

enum Coalition {
    Below,
    Edge,
    Above,
}

impl Coalition {
    fn from_values(mask: &[bool], values: &[f64]) -> Coalition {
        let n = mask.len();
        debug_assert!(n == values.len());
        let mut total: f64 = 0.0;
        for i in 0..n {
            if !mask[i] {
                continue;
            }
            total += values[i];
        }

        let mut on_edge = true;
        for i in 0..n {
            if !mask[i] {
                continue;
            }
            if total - values[i] >= 0.5 {
                on_edge = false;
            }
        }

        if total < 0.5 {
            Coalition::Below
        } else if on_edge {
            Coalition::Edge
        } else {
            Coalition::Above
        }
    }
}

impl DiagramConfig {
    fn circle_config(&self, coalition: Coalition) -> &CircleConfig {
        match coalition {
            Coalition::Below => &self.circle_below,
            Coalition::Edge => &self.circle_edge,
            Coalition::Above => &self.circle_above,
        }
    }
}
