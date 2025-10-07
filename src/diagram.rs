pub struct Diagram<const N: usize, const X: usize, const Y: usize> {
    pub venns: [Polyomino<X, Y>; N],
    pub values: [f64; N],
    pub colors: [String; N],
    pub radius: f64,
    pub opacity_below: f64,
    pub opacity_edge: f64,
    pub opacity_above: f64,
    pub circle_placement: CirclePlacement,
}

pub enum CirclePlacement {
    Basic,
    SquareCenter,
}

const SCALE: usize = 20;

use std::{mem, vec};

use svg::{
    Document,
    node::element::{Circle, Group, Path, Rectangle, SVG, path::Data},
};

use super::{
    Polyomino,
    direction::{DirectedEdge, Edge},
};

#[derive(Debug, Default, Clone, Copy)]
struct InnerOffset {
    above: i32,
    below: i32,
    right: i32,
    left: i32,
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

        enum Put {
            Left(usize),
            Right(usize),
        }

        impl Put {
            fn choose_smallest(a: Option<usize>, b: Option<usize>) -> Put {
                match (a, b) {
                    (None, None) => unreachable!(),
                    (None, Some(pr)) => Put::Right(pr),
                    (Some(pl), None) => Put::Left(pl),
                    (Some(pl), Some(pr)) => {
                        if pl < pr {
                            Put::Left(pl)
                        } else {
                            Put::Right(pr)
                        }
                    }
                }
            }
        }

        for i in 0..=X {
            columns[i].sort_by(|a, b| {
                let ap = combined_paths[a.0][a.1].len();
                let bp = combined_paths[b.0][b.1].len();
                ap.cmp(&bp).reverse()
            });

            let column = &columns[i];

            let l = column.len().div_ceil(2);

            let mut occupied_left = vec![vec![false; l]; Y];
            let mut occupied_right = vec![vec![false; l]; Y];

            for &(p_i, e_i) in column {
                let edge = &combined_paths[p_i][e_i];
                if let &DirectedEdge::Vertical { mut y_from, mut y_to, .. } = edge {
                    if y_to < y_from {
                        mem::swap(&mut y_from, &mut y_to);
                    }
                    debug_assert!(y_from < y_to);
                    let first_possible_left =
                        (0..l).position(|x| !(y_from..y_to).any(|i| occupied_left[i][x]));
                    let first_possible_right =
                        (0..l).position(|x| !(y_from..y_to).any(|i| occupied_right[i][x]));

                    let put = Put::choose_smallest(first_possible_left, first_possible_right);

                    match put {
                        Put::Left(j) => {
                            for i in y_from..y_to {
                                occupied_left[i][j] = true;
                            }
                            offsets[p_i][e_i] = -(j as i32) - 1;
                        }
                        Put::Right(j) => {
                            for i in y_from..y_to {
                                occupied_right[i][j] = true;
                            }
                            offsets[p_i][e_i] = j as i32;
                        }
                    }
                } else {
                    unreachable!();
                }
            }

            for j in 0..Y {
                let mut min_pos: i32 = i32::MAX;
                let mut max_pos: i32 = i32::MIN;
                for k in 0..l {
                    if occupied_right[j][k] {
                        let kk = k as i32;
                        if kk < min_pos {
                            min_pos = kk;
                        }
                        if kk > max_pos {
                            max_pos = kk;
                        }
                    }
                    if occupied_left[j][k] {
                        let kk = -(k as i32) - 1;
                        if kk < min_pos {
                            min_pos = kk;
                        }
                        if kk > max_pos {
                            max_pos = kk;
                        }
                    }
                }
                if i != X {
                    inner_offset[j][i].left = max_pos;
                }
                if i != 0 {
                    inner_offset[j][i - 1].right = min_pos;
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
            let l = row.len().div_ceil(2);

            let mut occupied_left = vec![vec![false; l]; X];
            let mut occupied_right = vec![vec![false; l]; X];

            for &(p_i, e_i) in row {
                let edge = &combined_paths[p_i][e_i];
                if let &DirectedEdge::Horizontal { mut x_from, mut x_to, .. } = edge {
                    if x_to < x_from {
                        mem::swap(&mut x_from, &mut x_to);
                    }
                    debug_assert!(x_from < x_to);
                    let first_possible_left =
                        (0..l).position(|x| !(x_from..x_to).any(|i| occupied_left[i][x]));
                    let first_possible_right =
                        (0..l).position(|x| !(x_from..x_to).any(|i| occupied_right[i][x]));

                    let put = Put::choose_smallest(first_possible_left, first_possible_right);

                    match put {
                        Put::Left(j) => {
                            for i in x_from..x_to {
                                occupied_left[i][j] = true;
                            }
                            offsets[p_i][e_i] = -(j as i32) - 1;
                        }
                        Put::Right(j) => {
                            for i in x_from..x_to {
                                occupied_right[i][j] = true;
                            }
                            offsets[p_i][e_i] = j as i32;
                        }
                    }
                } else {
                    unreachable!();
                }
            }

            for j in 0..X {
                let mut min_pos: i32 = i32::MAX;
                let mut max_pos: i32 = i32::MIN;
                for k in 0..l {
                    if occupied_right[j][k] {
                        let kk = k as i32;
                        if kk < min_pos {
                            min_pos = kk;
                        }
                        if kk > max_pos {
                            max_pos = kk;
                        }
                    }
                    if occupied_left[j][k] {
                        let kk = -(k as i32) - 1;
                        if kk < min_pos {
                            min_pos = kk;
                        }
                        if kk > max_pos {
                            max_pos = kk;
                        }
                    }
                }
                if i != Y {
                    inner_offset[i][j].above = max_pos;
                }
                if i != 0 {
                    inner_offset[i - 1][j].below = min_pos;
                }
            }
        }

        println!("{:?}", &offsets);
        println!("{:?}", &inner_offset);

        (offsets, inner_offset)
    }

    fn get_combined_paths(paths: Vec<Vec<DirectedEdge>>) -> Vec<Vec<DirectedEdge>> {
        let mut combined_paths: Vec<Vec<DirectedEdge>> = Vec::new();
        for path in paths {
            let mut out: Vec<DirectedEdge> = Vec::new();
            let mut current: Option<DirectedEdge> = None;
            for edge in path {
                current = match current {
                    Some(current_edge) => match current_edge.combine_directed(&edge) {
                        Some(combined_edge) => Some(combined_edge),
                        None => {
                            out.push(current_edge);
                            Some(edge)
                        }
                    },
                    None => Some(edge),
                };
            }

            if let Some(current) = current {
                if let Some(combined) = out[0].combine_directed(&current) {
                    out[0] = combined;
                } else {
                    out.push(current)
                }
            }

            combined_paths.push(out);
        }
        combined_paths
    }

    fn get_paths(&self, polys: &[Vec<Edge>]) -> Vec<Vec<DirectedEdge>> {
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

        for i in 0..N {
            // 1. List all the edges
            let poly = self.venns[i];
            let mut edges: Vec<Edge> = Vec::new();

            for x in 0..X {
                for y in 0..Y {
                    if poly[y][x] {
                        // Left
                        if x == 0 || !poly[y][x - 1] {
                            edges.push(Edge::new_vertical(x, y, y + 1));
                        }
                        // Up
                        if y == 0 || !poly[y - 1][x] {
                            edges.push(Edge::new_horizontal(y, x, x + 1));
                        }
                        // Right
                        if x == (X - 1) || !poly[y][x + 1] {
                            edges.push(Edge::new_vertical(x + 1, y, y + 1));
                        }
                        // Down
                        if y == (Y - 1) || !poly[y + 1][x] {
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
        &self,
        combined_paths: Vec<Vec<DirectedEdge>>,
        offsets: Vec<Vec<i32>>,
    ) -> Vec<Vec<(i32, i32)>> {
        // We will convert to just points, with offsets applied
        let mut points: Vec<Vec<(i32, i32)>> = Vec::new();
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
                out.push(((shared_x * SCALE) as i32 + ox, (shared_y * SCALE) as i32 + oy));
            }
            points.push(out);
        }
        points
    }

    pub fn to_svg(&self) -> SVG {
        let min_x = -((SCALE / 2) as i32);
        let max_x = (X + 1) * SCALE;

        let min_y = 0;
        let max_y = Y * SCALE;
        let mut out = Document::new()
            .set("viewBox", (min_x, min_y, max_x, max_y))
            .set("width", format!("{}px", 2 * X * SCALE))
            .set("height", format!("{}px", 2 * Y * SCALE));

        let rect = Rectangle::new()
            .set("width", (X + 1) * SCALE)
            .set("height", (Y + 1) * SCALE)
            .set("x", -((SCALE / 2) as i32))
            .set("y", -((SCALE / 2) as i32));

        out = out.add(rect);

        let polys = self.get_polys();
        let paths = self.get_paths(&polys);

        let combined_paths = Self::get_combined_paths(paths);

        let (offsets, internal_offsets) = Self::get_offsets(&combined_paths);

        let points = self.get_points(combined_paths, offsets);

        for (points, color) in points.iter().zip(&self.colors) {
            let mut data = Data::new().move_to(points[0]);
            for coord in &points[1..] {
                data = data.line_to(*coord);
            }
            data = data.close();
            let path = Path::new()
                .set("fill", color.clone())
                .set("fill-opacity", 0.2)
                .set("stroke", "none")
                .set("stroke-width", 1)
                .set("d", data);
            out = out.add(path);
        }

        for (points, color) in points.iter().zip(&self.colors) {
            let mut data = Data::new().move_to(points[0]);
            for coord in &points[1..] {
                data = data.line_to(*coord);
            }
            data = data.close();
            let path = Path::new()
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 1)
                .set("d", data);
            out = out.add(path);
        }

        let mut pairs = [false; N];
        for x in 0..X {
            for y in 0..Y {
                let mut any_true = false;
                for i in 0..N {
                    let v = self.venns[i][y][x];
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

                let cy = (above_y + below_y) / 2.0;
                let cx = (left_x + right_x) / 2.0;
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
        let mut group = Group::new().set("transform", format!("rotate(-90 {} {})", cx, cy));
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

        match coalition {
            Coalition::Below => {
                circle = circle.set("stroke", "red");
                if self.opacity_below != 1.0 {
                    group = group.set("opacity", self.opacity_below);
                }
            }
            Coalition::Edge => {
                circle = circle.set("stroke", "white");
                if self.opacity_edge != 1.0 {
                    group = group.set("opacity", self.opacity_edge);
                }
            }
            Coalition::Above => {
                circle = circle.set("stroke", "green");
                if self.opacity_above != 1.0 {
                    group = group.set("opacity", self.opacity_above);
                }
            }
        }
        group = group.add(circle);

        out.add(group)
    }
}
