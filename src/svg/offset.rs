use std::cmp::Ordering;

use z3::{
    Optimize, SatResult,
    ast::{Bool, Int},
};

use crate::{
    direction::{DirectedEdge, Direction, Edge},
    matrix::Matrix,
    svg::Diagonal,
};

#[derive(Debug, Default, Clone, Copy)]
pub(super) struct InnerOffset {
    pub(super) above: f64,
    pub(super) below: f64,
    pub(super) right: f64,
    pub(super) left: f64,
}

pub enum OffsetMethod {
    Greedy,
    Optimizing,
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

pub(super) fn get_offsets_greedy(
    x: usize,
    y: usize,
    combined_paths: &[Vec<DirectedEdge>],
    line_width: f64,
) -> (Vec<Vec<i32>>, Matrix<InnerOffset>) {
    let mut offsets: Vec<Vec<i32>> =
        combined_paths.iter().map(|x| vec![i32::MIN; x.len()]).collect();
    let mut columns = vec![Vec::new(); x + 1];
    let mut rows = vec![Vec::new(); y + 1];

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

    for (p_i, es) in combined_paths.iter().enumerate() {
        for (e_i, &e) in es.iter().enumerate() {
            let direction = directions[p_i][e_i];

            let (from, to, out) = match e.into() {
                Edge::Horizontal { x1, x2, y } => (x1, x2, &mut rows[y]),
                Edge::Vertical { y1, y2, x } => (y1, y2, &mut columns[x]),
            };

            out.push(EdgeInfo { from, to, direction, len: e.len(), p_i, e_i });
        }
    }

    // We choose the position in each column seperately
    for i in 0..=x {
        // We sort each edge that's contained is this column such that we start by
        // placing the longest edges
        columns[i].sort_by(|a, b| a.len.cmp(&b.len).reverse());

        // the current column
        let column = &columns[i];

        let len = column.len();

        let middle = len / 2;

        let mut occupied = Matrix::new(len, y, false);

        for &EdgeInfo { from, to, direction, p_i, e_i, .. } in column {
            let first_possible_left =
                (0..=middle).rev().find(|j| !(from..to).any(|i| occupied[(*j, i)]));
            let first_possible_right =
                (middle..len).find(|j| !(from..to).any(|i| occupied[(*j, i)]));

            let j = match (first_possible_left, first_possible_right) {
                (None, None) => unreachable!(),
                (None, Some(r)) => r,
                (Some(l), None) => l,
                (Some(l), Some(r)) => {
                    let prioritize_left = matches!(direction, Some(Direction::Left));

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

            for i in from..to {
                debug_assert!(!occupied[(j, i)]);
                occupied[(j, i)] = true;
            }
            offsets[p_i][e_i] = j as i32 - middle as i32;
        }
    }

    // We choose the position in each row seperately
    for i in 0..=y {
        // We sort each edge that's contained is this row such that we start by
        // placing the longest edges
        rows[i].sort_by(|a, b| a.len.cmp(&b.len).reverse());

        let row = &rows[i];

        let len = row.len();

        let middle = len / 2;

        let mut occupied = Matrix::new(len, x, false);

        for &EdgeInfo { from, to, direction, p_i, e_i, .. } in row {
            let first_possible_left =
                (0..=middle).rev().find(|j| !(from..to).any(|i| occupied[(*j, i)]));
            let first_possible_right =
                (middle..len).find(|j| !(from..to).any(|i| occupied[(*j, i)]));

            let j = match (first_possible_left, first_possible_right) {
                (None, None) => unreachable!(),
                (None, Some(r)) => r,
                (Some(l), None) => l,
                (Some(l), Some(r)) => {
                    let prioritize_left = matches!(direction, Some(Direction::Up));

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

            for i in from..to {
                debug_assert!(!occupied[(j, i)]);
                occupied[(j, i)] = true;
            }
            offsets[p_i][e_i] = j as i32 - middle as i32;
        }
    }

    let inner_offset = inner_offset(x, y, &offsets, combined_paths, line_width);

    (offsets, inner_offset)
}

pub(super) fn get_offsets_optimize(
    x: usize,
    y: usize,
    combined_paths: &[Vec<DirectedEdge>],
    line_width: f64,
) -> (Vec<Vec<i32>>, Matrix<InnerOffset>) {
    let n = combined_paths.len();

    let mut offsets: Vec<Vec<i32>> =
        combined_paths.iter().map(|x| vec![i32::MIN; x.len()]).collect();
    let mut row_edges: Matrix<Vec<Int>> = Matrix::new(x, y + 1, Vec::new());
    let mut column_edges: Matrix<Vec<Int>> = Matrix::new(x + 1, y, Vec::new());

    let solver = Optimize::new();

    // Create a variable for each edge
    let mut offset_variables: Vec<Vec<Int>> = Vec::new();

    #[derive(Debug, Clone)]
    struct Corner {
        int_vertical: Int,
        diagonal: Diagonal,
        int_horizontal: Int,
    }

    #[derive(Debug, Clone, Default)]
    struct CornerCrossings {
        corners: Vec<Corner>,
        crossing_vertical: Vec<Int>,
        crossing_horizontal: Vec<Int>,
    }

    let mut crossings = Matrix::new(x + 1, y + 1, CornerCrossings::default());

    // Create a variable for each edge
    let path_variables: Vec<Vec<Int>> = combined_paths
        .iter()
        .map(|x| x.iter().map(|_| Int::fresh_const("edge")).collect::<Vec<Int>>())
        .collect();

    for (path, variables) in combined_paths.iter().zip(&path_variables) {
        let mut path_variables = Vec::new();
        for (&edge, edge_variable) in path.iter().zip(variables) {
            let each_side = (n / 2) as i32;

            solver.assert(&edge_variable.le(each_side));
            solver.assert(&edge_variable.ge(-each_side));
            path_variables.push(edge_variable.clone());

            for i in (-each_side)..=each_side {
                let bb = edge_variable.ne(i);
                solver.assert_soft(&bb, (i.unsigned_abs() as usize) * edge.len(), None);
            }

            match edge.into() {
                Edge::Horizontal { y, x1, x2 } => {
                    for i in x1..x2 {
                        row_edges[(i, y)].push(edge_variable.clone());
                    }

                    // Add to crossing info
                    for i in (x1 + 1)..x2 {
                        crossings[(i, y)].crossing_horizontal.push(edge_variable.clone());
                    }
                }
                Edge::Vertical { x, y1, y2 } => {
                    for j in y1..y2 {
                        column_edges[(x, j)].push(edge_variable.clone());
                    }

                    // Add to crossing info
                    for j in (y1 + 1)..y2 {
                        crossings[(x, j)].crossing_vertical.push(edge_variable.clone());
                    }
                }
            }
        }
        offset_variables.push(path_variables);
    }

    // Include penalty for gaps
    for j in 0..=y {
        for i in 0..x {
            let values = &row_edges[(i, j)];
            debug_assert!(values.len() <= n);
            let each_side = (n / 2) as i32;

            for range in values.len()..n {
                let mut parts: Vec<Bool> = Vec::new();
                for start in -each_side..=each_side {
                    if start + range as i32 - 1 > each_side {
                        break;
                    }
                    let end = start + range as i32;
                    let mut edges_contained: Vec<Bool> = Vec::new();
                    for edge in values {
                        edges_contained.extend_from_slice(&[edge.ge(start), edge.lt(end)]);
                    }
                    parts.push(Bool::and(&edges_contained));
                }
                if !parts.is_empty() {
                    let b = Bool::or(&parts);
                    solver.assert_soft(&b, 200 * (range - values.len()), None);
                }
            }
        }
    }

    for (path, variables) in combined_paths.iter().zip(&path_variables) {
        let mut edges: Vec<_> = path.iter().zip(variables).collect();
        edges.push(edges[0]);

        for window in edges.windows(2) {
            if let [(edge_from, variable_from), (edge_to, variable_to)] = *window {
                debug_assert!(edge_from.to() == edge_to.from());
                let from_vertical: bool = match edge_from.direction() {
                    Direction::Left | Direction::Right => false,
                    Direction::Up | Direction::Down => true,
                };

                let diagonal = Diagonal::from_directions(
                    edge_from.direction().opposite(),
                    edge_to.direction(),
                )
                .unwrap();

                let (int_vertical, int_horizontal) = if from_vertical {
                    (variable_from.clone(), variable_to.clone())
                } else {
                    (variable_to.clone(), variable_from.clone())
                };

                let corner = Corner { int_vertical, diagonal, int_horizontal };

                let meets = edge_from.to();
                crossings[meets].corners.push(corner);
            } else {
                unreachable!();
            }
        }
    }

    for j in 0..=y {
        for i in 0..=x {
            let crossing = &crossings[(i, j)];

            const CORNER_WEIGHT: usize = 10;
            for corner in &crossing.corners {
                for edge in &crossing.crossing_horizontal {
                    let h = &corner.int_horizontal;
                    if corner.diagonal.down() {
                        solver.assert_soft(&h.ge(edge), CORNER_WEIGHT, None);
                    } else {
                        solver.assert_soft(&h.le(edge), CORNER_WEIGHT, None);
                    }
                }

                for edge in &crossing.crossing_vertical {
                    let v = &corner.int_vertical;
                    if corner.diagonal.right() {
                        solver.assert_soft(&v.ge(edge), CORNER_WEIGHT, None);
                    } else {
                        solver.assert_soft(&v.le(edge), CORNER_WEIGHT, None);
                    }
                }
            }

            let l = crossing.corners.len();
            for p in 0..l {
                for q in 0..p {
                    let corner1 = &crossing.corners[p];
                    let corner2 = &crossing.corners[q];

                    enum Case {
                        Same,
                        VSame,
                        HSame,
                    }

                    let case: Case = match (corner1.diagonal, corner2.diagonal) {
                        (Diagonal::UpLeft, Diagonal::UpLeft) => Case::Same,
                        (Diagonal::UpRight, Diagonal::UpRight) => Case::Same,
                        (Diagonal::DownLeft, Diagonal::DownLeft) => Case::Same,
                        (Diagonal::DownRight, Diagonal::DownRight) => Case::Same,
                        (Diagonal::UpLeft, Diagonal::UpRight) => Case::VSame,
                        (Diagonal::UpRight, Diagonal::UpLeft) => Case::VSame,
                        (Diagonal::DownLeft, Diagonal::DownRight) => Case::VSame,
                        (Diagonal::DownRight, Diagonal::DownLeft) => Case::VSame,
                        (Diagonal::UpLeft, Diagonal::DownLeft) => Case::HSame,
                        (Diagonal::UpRight, Diagonal::DownRight) => Case::HSame,
                        (Diagonal::DownLeft, Diagonal::UpLeft) => Case::HSame,
                        (Diagonal::DownRight, Diagonal::UpRight) => Case::HSame,

                        // We assume that corners pointing in opposite directions don't intersect
                        (Diagonal::UpRight, Diagonal::DownLeft)
                        | (Diagonal::DownLeft, Diagonal::UpRight)
                        | (Diagonal::UpLeft, Diagonal::DownRight)
                        | (Diagonal::DownRight, Diagonal::UpLeft) => continue,
                    };

                    let v1 = &corner1.int_vertical;
                    let v2 = &corner2.int_vertical;
                    let h1 = &corner1.int_horizontal;
                    let h2 = &corner2.int_horizontal;
                    let b = match case {
                        Case::Same => {
                            let aligned = match corner1.diagonal {
                                Diagonal::UpLeft | Diagonal::DownRight => true,
                                Diagonal::UpRight | Diagonal::DownLeft => false,
                            };

                            if aligned {
                                let b_le = Bool::and(&[v1.le(v2), h1.le(h2)]);
                                let b_ge = Bool::and(&[v1.ge(v2), h1.ge(h2)]);
                                Bool::or(&[b_le, b_ge])
                            } else {
                                let b_le_ge = Bool::and(&[v1.le(v2), h1.ge(h2)]);
                                let b_ge_le = Bool::and(&[v1.ge(v2), h1.le(h2)]);
                                Bool::or(&[b_le_ge, b_ge_le])
                            }
                        }
                        Case::HSame => {
                            if corner1.diagonal.down() {
                                h2.le(h1)
                            } else {
                                h1.le(h2)
                            }
                        }
                        Case::VSame => {
                            if corner1.diagonal.right() {
                                v2.le(v1)
                            } else {
                                v1.le(v2)
                            }
                        }
                    };
                    solver.assert_soft(&b, 2 * CORNER_WEIGHT, None);
                }
            }
        }
    }

    // None of the column edges overlap
    for i in 0..=x {
        for j in 0..y {
            let edges = &column_edges[(i, j)];
            let k = edges.len();
            for p in 0..k {
                for q in 0..p {
                    let edge1 = &edges[p];
                    let edge2 = &edges[q];
                    solver.assert(&edge1.ne(edge2));
                }
            }
        }
    }

    // None of the row edges overlap
    for i in 0..x {
        for j in 0..=y {
            let edges = &row_edges[(i, j)];
            let k = edges.len();
            for p in 0..k {
                for q in 0..p {
                    let edge1 = &edges[p];
                    let edge2 = &edges[q];
                    solver.assert(&edge1.ne(edge2));
                }
            }
        }
    }

    // Find the optimial solution, if there is one
    assert!(solver.check(&[]) == SatResult::Sat);
    if let Some(model) = solver.get_model() {
        for (i, path) in offset_variables.iter().enumerate() {
            for (j, edge) in path.iter().enumerate() {
                let value = model.get_const_interp(edge).unwrap().as_i64().unwrap() as i32;
                offsets[i][j] = value;
            }
        }
    } else {
        unreachable!();
    }

    let inner_offset = inner_offset(x, y, &offsets, combined_paths, line_width);

    (offsets, inner_offset)
}

fn inner_offset(
    max_x: usize,
    max_y: usize,
    path_offsets: &[Vec<i32>],
    combined_paths: &[Vec<DirectedEdge>],
    line_width: f64,
) -> Matrix<InnerOffset> {
    let min_offset: InnerOffset =
        InnerOffset { above: f64::MIN, below: f64::MIN, right: f64::MIN, left: f64::MIN };
    let mut inner_offset: Matrix<InnerOffset> = Matrix::new(max_x, max_y, min_offset);

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
                        if y != max_y {
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
                        if x != max_x {
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
