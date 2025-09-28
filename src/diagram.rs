pub struct Diagram<const N: usize, const X: usize, const Y: usize> {
    pub venns: [Polyomino<X, Y>; N],
    pub values: [f64; N],
    pub colors: [String; N],
}

const SCALE: usize = 20;

use std::vec;

use svg::{
    Document,
    node::element::{Circle, Group, Path, Rectangle, SVG, path::Data},
};

use super::{
    Polyomino,
    direction::{
        Direction::{self, *},
        combine, connected, endpoints, from_endpoints,
    },
};

impl<const N: usize, const X: usize, const Y: usize> Diagram<N, X, Y> {
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

        let mut polys: Vec<Vec<Direction>> = Vec::new();

        for i in 0..N {
            // 1. List all the edges
            let poly = self.venns[i];
            let mut edges: Vec<Direction> = Vec::new();

            for x in 0..X {
                for y in 0..Y {
                    if poly[y][x] {
                        // Left
                        if x == 0 || !poly[y][x - 1] {
                            edges.push(Vertical { x, y1: y, y2: y + 1 });
                        }
                        // Up
                        if y == 0 || !poly[y - 1][x] {
                            edges.push(Horizontal { y, x1: x, x2: x + 1 });
                        }
                        // Right
                        if x == (X - 1) || !poly[y][x + 1] {
                            edges.push(Vertical { x: x + 1, y1: y, y2: y + 1 });
                        }
                        // Down
                        if y == (Y - 1) || !poly[y + 1][x] {
                            edges.push(Horizontal { y: y + 1, x1: x, x2: x + 1 });
                        }
                    }
                }
            }

            polys.push(edges);
        }

        let mut paths: Vec<Vec<Direction>> = Vec::new();

        for edges in polys {
            // 1. Create adjancy matrix
            let l = edges.len();
            let mut adj: Vec<Vec<bool>> = vec![vec![false; l]; l];

            for i in 0..l {
                for j in 0..l {
                    if i == j {
                        continue;
                    }
                    if connected(&edges[i], &edges[j]) {
                        adj[i][j] = true;
                    }
                }
            }

            let mut path: Vec<Direction> = Vec::new();

            // current edge we're examining
            let mut i: usize = 0;
            path.push(edges[0].clone());
            while let Some(j) = adj[i].iter().position(|x| *x) {
                // Remove from adjacency matrix
                for k in 0..l {
                    adj[i][k] = false;
                    adj[k][i] = false;
                }

                path.push(edges[j].clone());
                i = j;
            }

            for k in 0..l {
                adj[i][k] = false;
                adj[k][i] = false;
            }

            // I don't think we need to handle holes or disjoint yet
            // Let's just check we used every edge in this path
            for i in 0..l {
                for j in 0..l {
                    assert!(!adj[i][j]);
                }
            }
            paths.push(path);
        }

        // TODO: Combine edges here
        let mut combined_paths: Vec<Vec<Direction>> = Vec::new();
        for path in paths {
            let mut out = Vec::new();
            let mut current = None;
            for edge in path {
                current = match current {
                    Some(current_edge) => match combine(&current_edge, &edge) {
                        Some(combined_edge) => Some(combined_edge),
                        None => {
                            out.push(current_edge);
                            Some(edge)
                        }
                    },
                    None => Some(edge),
                };
            }

            out.push(current.unwrap());

            if let (Horizontal { .. }, Horizontal { .. }) | (Vertical { .. }, Vertical { .. }) =
                (out[0].clone(), out.last().unwrap().clone())
            {
                let last = out.pop().unwrap();
                out[0] = combine(&out[0], &last).unwrap();
            }

            combined_paths.push(out);
        }

        let mut offsets: Vec<Vec<i32>> =
            combined_paths.iter().map(|x| vec![i32::MIN; x.len()]).collect();
        let mut columns = vec![Vec::new(); X + 1];
        let mut rows = vec![Vec::new(); Y + 1];

        for (p_i, es) in combined_paths.iter().enumerate() {
            for (e_i, e) in es.iter().enumerate() {
                match e {
                    &Horizontal { y, .. } => rows[y].push((p_i, e_i)),
                    &Vertical { x, .. } => columns[x].push((p_i, e_i)),
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

            let mut occupied_left = vec![vec![false; column.len()]; Y];
            let mut occupied_right = vec![vec![false; column.len()]; Y];

            for &(p_i, e_i) in column {
                let edge = &combined_paths[p_i][e_i];
                if let &Vertical { y1, y2, .. } = edge {
                    assert!(y1 < y2);
                    let first_possible_left = (0..column.len())
                        .position(|x| !(y1..y2).any(|i| occupied_left[i][x]))
                        .unwrap();
                    let first_possible_right = (0..column.len())
                        .position(|x| !(y1..y2).any(|i| occupied_right[i][x]))
                        .unwrap();

                    if first_possible_left < first_possible_right {
                        for i in y1..y2 {
                            occupied_left[i][first_possible_left] = true;
                        }
                        offsets[p_i][e_i] = -(first_possible_left as i32) - 1;
                    } else {
                        for i in y1..y2 {
                            occupied_right[i][first_possible_right] = true;
                        }
                        offsets[p_i][e_i] = first_possible_right as i32;
                    }
                } else {
                    unreachable!();
                }
            }
        }

        for i in 0..=Y {
            rows[i].sort_by(|a, b| {
                let ap = combined_paths[a.0][a.1].len();
                let bp = combined_paths[b.0][b.1].len();
                ap.cmp(&bp).reverse()
            });

            let column = &rows[i];

            let mut occupied_left = vec![vec![false; column.len()]; X];
            let mut occupied_right = vec![vec![false; column.len()]; X];

            for &(p_i, e_i) in column {
                let edge = &combined_paths[p_i][e_i];
                if let &Horizontal { x1, x2, .. } = edge {
                    assert!(x1 < x2);
                    let first_possible_left = (0..column.len())
                        .position(|x| !(x1..x2).any(|i| occupied_left[i][x]))
                        .unwrap();
                    let first_possible_right = (0..column.len())
                        .position(|x| !(x1..x2).any(|i| occupied_right[i][x]))
                        .unwrap();

                    if first_possible_left < first_possible_right {
                        for i in x1..x2 {
                            occupied_left[i][first_possible_left] = true;
                        }
                        offsets[p_i][e_i] = -(first_possible_left as i32) - 1;
                    } else {
                        for i in x1..x2 {
                            occupied_right[i][first_possible_right] = true;
                        }
                        offsets[p_i][e_i] = first_possible_right as i32;
                    }
                } else {
                    unreachable!();
                }
            }
        }

        println!("{:?}", &offsets);

        // Rotate the edges to the right direction
        for path in &mut combined_paths {
            let (first, second) = (&path[0], &path[1]);
            let (a1, a2) = endpoints(first);
            let (b1, b2) = endpoints(second);
            let mut start_point = if a1 == b1 || a1 == b2 {
                a2
            } else if a2 == b1 || a2 == b2 {
                a1
            } else {
                unreachable!();
            };

            for e in path {
                let (a1, a2) = endpoints(e);
                (*e, start_point) = if a1 == start_point {
                    (from_endpoints(start_point, a2), a2)
                } else if a2 == start_point {
                    (from_endpoints(start_point, a1), a1)
                } else {
                    unreachable!();
                }
            }
        }

        // We will convert to just points, with offsets applied
        let mut points: Vec<Vec<(i32, i32)>> = Vec::new();
        for (path_edges, path_offsets) in combined_paths.into_iter().zip(offsets) {
            let mut out = Vec::new();
            let last_edge = path_edges.last().unwrap().clone();
            let last_offset = path_offsets.last().unwrap().clone();

            let path_edges = std::iter::once(last_edge).chain(path_edges);
            let path_offsets = std::iter::once(last_offset).chain(path_offsets);
            let parts: Vec<(Direction, i32)> = path_edges.zip(path_offsets).collect();
            for aa in parts.windows(2) {
                let ((e1, o1), (e2, o2)) = (&aa[0], &aa[1]);
                let (_, (shared_x, shared_y)) = endpoints(e1);
                assert!((shared_x, shared_y) == endpoints(e2).0);

                let (ox, oy) = match e1 {
                    Horizontal { .. } => (o2, o1),
                    Vertical { .. } => (o1, o2),
                };
                out.push(((shared_x * SCALE) as i32 + ox, (shared_y * SCALE) as i32 + oy));
            }
            points.push(out);
        }

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

        for x in 0..X {
            for y in 0..Y {
                let pairs: Vec<(f64, &String)> = (0..N)
                    .filter(|&i| self.venns[i][y][x])
                    .map(|i| (self.values[i], &self.colors[i]))
                    .collect();
                out = draw_circle(x * SCALE + SCALE / 2, y * SCALE + SCALE / 2, &pairs, out);
            }
        }

        out
    }
}

fn draw_circle(cx: usize, cy: usize, values: &[(f64, &String)], out: SVG) -> SVG {
    let r = 3.5;
    let c = std::f64::consts::TAU * r as f64;
    let mut added = 0.0;
    let total: f64 = values.iter().map(|x| x.0).sum();
    let mut group = Group::new();
    let on_edge: bool = values.iter().all(|x| total - x.0 < 0.5);
    if total <= 0.5 {
        group = group.set("opacity", 0.2);
    } else if !on_edge {
        group = group.set("opacity", 0.6);
    }
    for (size, color) in values {
        let mut circle = Circle::new()
            .set("r", r)
            .set("cx", cx)
            .set("cy", cy)
            .set("fill", "transparent")
            .set("stroke", (*color).clone())
            .set("stroke-width", r * 2.0)
            .set("transform", format!("rotate(-90 {} {})", cx, cy))
            .set("stroke-dasharray", format!("{}, {}", c * size, c));
        if added != 0.0 {
            circle = circle.set("stroke-dashoffset", -added);
        }

        added += c * size;
        group = group.add(circle);
    }

    if total >= 0.5 && on_edge {
        let circle = Circle::new()
            .set("r", r * 2.0)
            .set("cx", cx)
            .set("cy", cy)
            .set("fill", "transparent")
            .set("stroke", "white")
            .set("stroke-width", 0.5);
        group = group.add(circle);
    }

    out.add(group)
}
