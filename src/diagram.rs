pub struct Diagram<const N: usize, const X: usize, const Y: usize> {
    pub venns: ConstVennDiagram<N, X, Y>,
    pub values: [f64; N],
    pub colors: [String; N],
    pub radius: f64,
    pub circle_below: CircleConfig,
    pub circle_edge: CircleConfig,
    pub circle_above: CircleConfig,
    pub circle_placement: CirclePlacement,
    pub corner_style: CornerStyle,
}

pub struct ConstVennDiagram<const N: usize, const X: usize, const Y: usize> {
    pub(crate) polyominos: [ConstPolyomino<X, Y>; N],
}

impl<const N: usize, const X: usize, const Y: usize> ConstVennDiagram<N, X, Y> {
    pub const fn new(polyominos: [ConstPolyomino<X, Y>; N]) -> Self {
        Self { polyominos }
    }

    /// The venn diagram consists of all subsets (except the empty subset)
    pub const fn complete(&self) -> bool {
        // We check that each group is the right size
        let count_goal: u64 = 1 << (N - 1);
        let mut i = 0;
        while i != N {
            let mut count = 0;
            let part = &self.polyominos[i];
            let mut x = 0;
            while x != X {
                let mut y = 0;
                while y != Y {
                    if part.values[y][x] {
                        count += 1;
                    }
                    y += 1;
                }
                x += 1;
            }

            if count != count_goal {
                return false;
            }
            i += 1;
        }

        // Each cell is different
        let mut x1 = 0;
        while x1 != X {
            let mut y1 = 0;
            while y1 != Y {
                let mut x2 = 0;
                while x2 != X {
                    let mut y2 = 0;
                    while y2 != Y {
                        let eq_coord = x1 == x2 && y1 == y2;
                        let all_false_1 = empty_at(&self.polyominos, x1, y1);
                        let all_false_2 = empty_at(&self.polyominos, x2, y2);
                        if !eq_coord
                            && !all_false_1
                            && !all_false_2
                            && !different_at(&self.polyominos, x1, y1, x2, y2)
                        {
                            return false;
                        }
                        y2 += 1;
                    }
                    x2 += 1;
                }
                y1 += 1;
            }
            x1 += 1;
        }

        true
    }
}

const fn empty_at<const N: usize, const X: usize, const Y: usize>(
    polys: &[ConstPolyomino<X, Y>; N],
    x: usize,
    y: usize,
) -> bool {
    let mut i = 0;
    while i != N {
        let p2 = polys[i].values[y][x];
        if p2 {
            return false;
        }
        i += 1;
    }
    true
}

const fn different_at<const N: usize, const X: usize, const Y: usize>(
    polys: &[ConstPolyomino<X, Y>; N],
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
) -> bool {
    let mut i = 0;
    while i != N {
        let p1 = polys[i].values[y1][x1];
        let p2 = polys[i].values[y2][x2];
        if p1 != p2 {
            return true;
        }
        i += 1;
    }
    false
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

pub enum CornerStyle {
    Straight,
    Smooth,
}

pub enum CirclePlacement {
    Basic,
    SquareCenter,
}

const SCALE: usize = 20;

use std::{cmp::Ordering, mem, vec};

use svg::{
    Document,
    node::element::{Circle, Definitions, Group, Mask, Path, Rectangle, SVG, path::Data},
};

use super::{
    ConstPolyomino,
    direction::{DirectedEdge, Edge},
};
use crate::direction::Direction;

#[derive(Debug, Default, Clone, Copy)]
struct InnerOffset {
    above: i32,
    below: i32,
    right: i32,
    left: i32,
}

const CORNER_OFFSET: i32 = 3;

struct Corner {
    from: (i32, i32),
    to: (i32, i32),
    clockwise: bool,
}

impl Corner {
    // Parameters which can be used to create an "elliptical_arc" in SVG
    fn params(&self) -> (i32, i32, i32, i32, i32, i32, i32) {
        (CORNER_OFFSET, CORNER_OFFSET, 0, 0, i32::from(self.clockwise), self.to.0, self.to.1)
    }
}

impl<const N: usize, const X: usize, const Y: usize> Diagram<N, X, Y> {
    fn get_offsets(combined_paths: &[Vec<DirectedEdge>]) -> (Vec<Vec<i32>>, Vec<Vec<InnerOffset>>) {
        let mut offsets: Vec<Vec<i32>> =
            combined_paths.iter().map(|x| vec![i32::MIN; x.len()]).collect();
        let mut columns = vec![Vec::new(); X + 1];
        let mut rows = vec![Vec::new(); Y + 1];

        let mut inner_offset: Vec<Vec<InnerOffset>> = vec![vec![InnerOffset::default(); X]; Y];

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

        for i in 0..=X {
            columns[i].sort_by(|a, b| {
                let ap = combined_paths[a.0][a.1].len();
                let bp = combined_paths[b.0][b.1].len();
                ap.cmp(&bp).reverse()
            });

            let column = &columns[i];

            let l = column.len();
            let middle = l / 2;

            let mut occupied = vec![vec![false; l]; Y];

            for &(p_i, e_i) in column {
                let edge = &combined_paths[p_i][e_i];
                let edge_direction = directions[p_i][e_i];
                if let &DirectedEdge::Vertical { mut y_from, mut y_to, .. } = edge {
                    if y_to < y_from {
                        mem::swap(&mut y_from, &mut y_to);
                    }
                    debug_assert!(y_from < y_to);
                    let mut first_possible_left = middle;
                    while first_possible_left != 0 {
                        if (y_from..y_to).any(|i| occupied[i][first_possible_left]) {
                            first_possible_left -= 1;
                        } else {
                            break;
                        }
                    }
                    let mut first_possible_right = middle;
                    while first_possible_right != l {
                        if (y_from..y_to).any(|i| occupied[i][first_possible_right]) {
                            first_possible_right += 1;
                        } else {
                            break;
                        }
                    }

                    let prioritize_left = match edge_direction {
                        Some(Direction::Left) => true,
                        Some(Direction::Right) | None => false,
                        Some(Direction::Up | Direction::Down) => unreachable!(),
                    };

                    let left_dist = middle.abs_diff(first_possible_left);
                    let right_dist = middle.abs_diff(first_possible_right);

                    let choose_left = match left_dist.cmp(&right_dist) {
                        Ordering::Less => true,
                        Ordering::Equal => prioritize_left,
                        Ordering::Greater => false,
                    };

                    let j = if choose_left { first_possible_left } else { first_possible_right };

                    for i in y_from..y_to {
                        occupied[i][j] = true;
                    }
                    offsets[p_i][e_i] = (j as i32) - middle as i32;
                } else {
                    unreachable!();
                }
            }

            for j in 0..Y {
                let mut min_pos: usize = usize::MAX;
                let mut max_pos: usize = usize::MIN;
                for k in 0..l {
                    if occupied[j][k] {
                        if k < min_pos {
                            min_pos = k;
                        }
                        if k > max_pos {
                            max_pos = k;
                        }
                    }
                }
                if i != X {
                    inner_offset[j][i].left = (max_pos as i32) - middle as i32;
                }
                if i != 0 {
                    inner_offset[j][i - 1].right = (min_pos as i32) - middle as i32;
                }
            }
        }

        for i in 0..=Y {
            rows[i].sort_by(|a, b| {
                let ap = combined_paths[a.0][a.1].len();
                let bp = combined_paths[b.0][b.1].len();
                ap.cmp(&bp).reverse()
            });

            let row = &rows[i];

            let l = row.len();
            let middle = l / 2;

            let mut occupied = vec![vec![false; l]; X];

            for &(p_i, e_i) in row {
                let edge = &combined_paths[p_i][e_i];
                let edge_direction = directions[p_i][e_i];
                if let &DirectedEdge::Horizontal { mut x_from, mut x_to, .. } = edge {
                    if x_to < x_from {
                        mem::swap(&mut x_from, &mut x_to);
                    }
                    debug_assert!(x_from < x_to);
                    let mut first_possible_left = middle;
                    while first_possible_left != 0 {
                        if (x_from..x_to).any(|i| occupied[i][first_possible_left]) {
                            first_possible_left -= 1;
                        } else {
                            break;
                        }
                    }
                    let mut first_possible_right = middle;
                    while first_possible_right != l {
                        if (x_from..x_to).any(|i| occupied[i][first_possible_right]) {
                            first_possible_right += 1;
                        } else {
                            break;
                        }
                    }

                    let prioritize_left = match edge_direction {
                        Some(Direction::Up) => true,
                        Some(Direction::Down) | None => false,
                        Some(Direction::Left | Direction::Right) => unreachable!(),
                    };

                    let left_dist = middle.abs_diff(first_possible_left);
                    let right_dist = middle.abs_diff(first_possible_right);

                    let choose_left = match left_dist.cmp(&right_dist) {
                        Ordering::Less => true,
                        Ordering::Equal => prioritize_left,
                        Ordering::Greater => false,
                    };

                    let j = if choose_left { first_possible_left } else { first_possible_right };

                    for i in x_from..x_to {
                        occupied[i][j] = true;
                    }
                    offsets[p_i][e_i] = (j as i32) - middle as i32;
                } else {
                    unreachable!();
                }
            }

            for j in 0..X {
                let mut min_pos: usize = usize::MAX;
                let mut max_pos: usize = usize::MIN;
                for k in 0..l {
                    if occupied[j][k] {
                        if k < min_pos {
                            min_pos = k;
                        }
                        if k > max_pos {
                            max_pos = k;
                        }
                    }
                }
                if i != Y {
                    inner_offset[i][j].above = (max_pos as i32) - middle as i32;
                }
                if i != 0 {
                    inner_offset[i - 1][j].below = (min_pos as i32) - middle as i32;
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
            let mut adj: Vec<Vec<bool>> = vec![vec![false; l]; l];

            for i in 0..l {
                for j in 0..l {
                    if i == j {
                        continue;
                    }
                    if edges[i].connected(&edges[j]) {
                        adj[i][j] = true;
                    }
                }
            }

            let mut path: Vec<DirectedEdge> = Vec::new();

            // current edge we're examining
            let mut i: usize = 0;
            let mut pre: Option<(usize, usize)> = None;
            while let Some(j) = adj[i].iter().position(|x| *x) {
                for k in 0..l {
                    adj[i][k] = false;
                    adj[k][i] = false;
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
                    debug_assert!(!adj[i][j]);
                }
            }
            paths.push(path);
        }
        paths
    }

    fn get_polys(&self) -> Vec<Vec<Edge>> {
        let mut polys: Vec<Vec<Edge>> = Vec::new();

        for poly in self.venns.polyominos {
            let mut edges: Vec<Edge> = Vec::new();

            for x in 0..X {
                for y in 0..Y {
                    if poly.values[y][x] {
                        // Left
                        if x == 0 || !poly.values[y][x - 1] {
                            edges.push(Edge::new_vertical(x, y, y + 1));
                        }
                        // Up
                        if y == 0 || !poly.values[y - 1][x] {
                            edges.push(Edge::new_horizontal(y, x, x + 1));
                        }
                        // Right
                        if x == (X - 1) || !poly.values[y][x + 1] {
                            edges.push(Edge::new_vertical(x + 1, y, y + 1));
                        }
                        // Down
                        if y == (Y - 1) || !poly.values[y + 1][x] {
                            edges.push(Edge::new_horizontal(y + 1, x, x + 1));
                        }
                    }
                }
            }

            polys.push(edges);
        }
        polys
    }

    fn get_points(
        combined_paths: Vec<Vec<DirectedEdge>>,
        offsets: Vec<Vec<i32>>,
    ) -> Vec<Vec<Corner>> {
        // We will convert to just points, with offsets applied
        let mut points: Vec<Vec<Corner>> = Vec::new();
        for (path_edges, path_offsets) in combined_paths.into_iter().zip(offsets) {
            let mut out = Vec::new();
            let last_edge = path_edges.last().unwrap();
            let last_offset = *path_offsets.last().unwrap();

            let path_edges = std::iter::once(last_edge).chain(&path_edges);
            let path_offsets = std::iter::once(last_offset).chain(path_offsets);
            let parts: Vec<(&DirectedEdge, i32)> = path_edges.zip(path_offsets).collect();
            for aa in parts.windows(2) {
                let ((e1, o1), (e2, o2)) = (&aa[0], &aa[1]);
                let (shared_x, shared_y) = e1.to();
                debug_assert!(e1.to() == e2.from());

                let (ox, oy) = match e1 {
                    DirectedEdge::Horizontal { .. } => (o2, o1),
                    DirectedEdge::Vertical { .. } => (o1, o2),
                };

                let meet_x = (shared_x * SCALE) as i32 + ox;
                let meet_y = (shared_y * SCALE) as i32 + oy;

                let from = match e1.direction() {
                    Direction::Left => (meet_x + CORNER_OFFSET, meet_y),
                    Direction::Right => (meet_x - CORNER_OFFSET, meet_y),
                    Direction::Up => (meet_x, meet_y + CORNER_OFFSET),
                    Direction::Down => (meet_x, meet_y - CORNER_OFFSET),
                };

                let to = match e2.direction() {
                    Direction::Left => (meet_x - CORNER_OFFSET, meet_y),
                    Direction::Right => (meet_x + CORNER_OFFSET, meet_y),
                    Direction::Up => (meet_x, meet_y - CORNER_OFFSET),
                    Direction::Down => (meet_x, meet_y + CORNER_OFFSET),
                };

                let clockwise = match (e1.direction(), e2.direction()) {
                    (Direction::Left, Direction::Down)
                    | (Direction::Right, Direction::Up)
                    | (Direction::Up, Direction::Left)
                    | (Direction::Down, Direction::Right) => false,
                    (Direction::Left, Direction::Up)
                    | (Direction::Right, Direction::Down)
                    | (Direction::Up, Direction::Right)
                    | (Direction::Down, Direction::Left) => true,
                    (Direction::Left | Direction::Right, Direction::Left | Direction::Right)
                    | (Direction::Up | Direction::Down, Direction::Up | Direction::Down) => {
                        unreachable!()
                    }
                };

                let corner = Corner { from, to, clockwise };
                out.push(corner);
            }
            points.push(out);
        }
        points
    }

    fn get_rounded_paths(&self, points: Vec<Vec<Corner>>) -> Vec<Path> {
        let mut paths = Vec::new();

        match self.corner_style {
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
    pub fn to_svg(&self) -> SVG {
        // First we do calculations
        let polys = self.get_polys();
        let paths = Self::get_paths(&polys);

        let combined_paths = Self::get_combined_paths(paths);

        let (offsets, internal_offsets) = Self::get_offsets(&combined_paths);

        let points = Self::get_points(combined_paths, offsets);

        let paths = self.get_rounded_paths(points);

        // Then we create the svg
        let min_x = -((SCALE / 2) as i32);
        let max_x = (X + 1) * SCALE;

        let min_y = 0;
        let max_y = Y * SCALE;
        let mut out = Document::new()
            .set("viewBox", (min_x, min_y, max_x, max_y))
            .set("width", format!("{}px", 2 * X * SCALE))
            .set("height", format!("{}px", 2 * Y * SCALE));

        let mut mask = Mask::new().set("id", "background_mask");
        for path in &paths {
            let part = path.clone().set("fill", "white").set("stroke", "none");
            mask = mask.add(part);
        }

        let defs = Definitions::new().add(mask);

        out = out.add(defs);

        let rect = Rectangle::new()
            .set("width", (X + 1) * SCALE)
            .set("height", (Y + 1) * SCALE)
            .set("x", -((SCALE / 2) as i32))
            .set("y", -((SCALE / 2) as i32))
            .set("mask", "url(#background_mask)");

        out = out.add(rect);

        for (path, color) in paths.iter().zip(&self.colors) {
            let path = path
                .clone()
                .set("fill", color.clone())
                .set("fill-opacity", 0.2)
                .set("stroke", "none")
                .set("stroke-width", 1);
            out = out.add(path);
        }

        for (path, color) in paths.iter().zip(&self.colors) {
            let path = path
                .clone()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 1);
            out = out.add(path);
        }

        let mut pairs = [false; N];
        for x in 0..X {
            for y in 0..Y {
                let mut any_true = false;
                for i in 0..N {
                    let v = self.venns.polyominos[i].values[y][x];
                    any_true |= v;
                    pairs[i] = v;
                }
                if any_true {
                    let (x_pos, y_pos) = self.get_circle_pos(x, y, &internal_offsets);
                    out = self.draw_circle(x_pos, y_pos, &pairs, out);
                }
            }
        }
        out
    }

    fn get_circle_pos(
        &self,
        x: usize,
        y: usize,
        internal_offsets: &[Vec<InnerOffset>],
    ) -> (f64, f64) {
        match self.circle_placement {
            CirclePlacement::Basic => (
                ((x * SCALE) as f64) + (SCALE as f64) / 2.0,
                ((y * SCALE) as f64) + (SCALE as f64) / 2.0,
            ),
            CirclePlacement::SquareCenter => {
                let internal_offset = internal_offsets[y][x];
                let cx = (x * SCALE) as f64;
                let cy = (y * SCALE) as f64;

                let whole = SCALE as f64;

                let above_y = cy + internal_offset.above as f64;
                let below_y = cy + whole + internal_offset.below as f64;
                let left_x = cx + internal_offset.left as f64;
                let right_x = cx + whole + internal_offset.right as f64;

                let cy = f64::midpoint(above_y, below_y);
                let cx = f64::midpoint(left_x, right_x);
                (cx, cy)
            }
        }
    }

    fn draw_circle(&self, cx: f64, cy: f64, values: &[bool; N], out: SVG) -> SVG {
        enum Coalition {
            Below,
            Edge,
            Above,
        }

        let r = self.radius;
        let c = std::f64::consts::TAU * r;
        let mut group = Group::new().set("transform", format!("rotate(-90 {cx} {cy})"));
        let mut total: f64 = 0.0;
        for i in 0..N {
            if !values[i] {
                continue;
            }
            total += self.values[i];
        }

        let mut on_edge = true;
        for i in 0..N {
            if !values[i] {
                continue;
            }
            if total - self.values[i] >= 0.5 {
                on_edge = false;
            }
        }

        let coalition: Coalition = if total < 0.5 {
            Coalition::Below
        } else if on_edge {
            Coalition::Edge
        } else {
            Coalition::Above
        };

        let mut added = 0.0;
        for i in 0..N {
            if !values[i] {
                continue;
            }
            let size = &self.values[i];
            let color = &self.colors[i];

            let mut circle = Circle::new()
                .set("r", r)
                .set("cx", cx)
                .set("cy", cy)
                .set("fill", "transparent")
                .set("stroke", (*color).clone())
                .set("stroke-width", r * 2.0)
                .set("stroke-dasharray", format!("{}, {}", c * size, c));
            if added != 0.0 {
                circle = circle.set("stroke-dashoffset", -added);
            }

            added += c * size;
            group = group.add(circle);
        }

        let mut circle = Circle::new()
            .set("r", r * 2.0)
            .set("cx", cx)
            .set("cy", cy)
            .set("fill", "transparent")
            .set("stroke", "white")
            .set("stroke-width", 0.5);

        let config: &CircleConfig = match coalition {
            Coalition::Below => &self.circle_below,
            Coalition::Edge => &self.circle_edge,
            Coalition::Above => &self.circle_above,
        };
        circle = circle.set("stroke", config.color.as_str());

        if config.opacity != 1.0 {
            group = group.set("opacity", config.opacity);
        }
        group = group.add(circle);

        out.add(group)
    }
}
