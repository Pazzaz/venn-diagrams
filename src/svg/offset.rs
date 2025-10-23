use std::{cmp::Ordering, mem};

use z3::{Optimize, SatResult, ast::Int};

use crate::{
    direction::{DirectedEdge, Direction},
    matrix::Matrix,
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
        for (e_i, e) in es.iter().enumerate() {
            let direction = directions[p_i][e_i];

            let (mut from, mut to, out) = match *e {
                DirectedEdge::Horizontal { x_from, x_to, y } => (x_from, x_to, &mut rows[y]),
                DirectedEdge::Vertical { y_from, y_to, x } => (y_from, y_to, &mut columns[x]),
            };

            if from > to {
                mem::swap(&mut from, &mut to);
            }

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
    let mut offsets: Vec<Vec<i32>> =
        combined_paths.iter().map(|x| vec![i32::MIN; x.len()]).collect();
    let mut row_edges: Matrix<(usize, Vec<Int>)> = Matrix::new(x, y + 1, (0, Vec::new()));
    let mut column_edges: Matrix<(usize, Vec<Int>)> = Matrix::new(x + 1, y, (0, Vec::new()));

    let solver = Optimize::new();

    // Create a variable for each edge
    let mut offset_variables: Vec<Vec<Int>> = Vec::new();

    // TODO:
    // 1. Add each edge to the corner which it passes (seperate vertical and
    //    horizontal)
    // 2. Co through each corner, and add penalties when it crosses an edge
    // 3. But wait, sometimes edges turn on the same edge
    // 4. There are gonna be a lot of cases...

    // Find how many edges are on each part
    for path in combined_paths {
        for edge in path {
            match *edge {
                DirectedEdge::Horizontal { y, mut x_from, mut x_to } => {
                    if x_from > x_to {
                        mem::swap(&mut x_from, &mut x_to);
                    }
                    for i in x_from..x_to {
                        row_edges[(i, y)].0 += 1;
                    }
                }
                DirectedEdge::Vertical { x, mut y_from, mut y_to } => {
                    if y_from > y_to {
                        mem::swap(&mut y_from, &mut y_to);
                    }
                    for j in y_from..y_to {
                        column_edges[(x, j)].0 += 1;
                    }
                }
            }
        }
    }

    for path in combined_paths {
        let mut path_variables = Vec::new();
        for edge in path {
            let edge_variable = Int::fresh_const("edge");

            let max_count = match *edge {
                DirectedEdge::Horizontal { y, mut x_from, mut x_to } => {
                    if x_from > x_to {
                        mem::swap(&mut x_from, &mut x_to);
                    }
                    (x_from..x_to).map(|i| row_edges[(i, y)].0).max().unwrap()
                }
                DirectedEdge::Vertical { x, mut y_from, mut y_to } => {
                    if y_from > y_to {
                        mem::swap(&mut y_from, &mut y_to);
                    }
                    (y_from..y_to).map(|j| column_edges[(x, j)].0).max().unwrap()
                }
            };

            let each_side = (max_count / 2) as i32;

            solver.assert(&edge_variable.le(each_side));
            solver.assert(&edge_variable.ge(-each_side));
            path_variables.push(edge_variable.clone());

            for i in (-each_side)..=each_side {
                let bb = edge_variable.ne(i);
                solver.assert_soft(&bb, (i.unsigned_abs() as usize) * edge.len(), None);
            }

            match *edge {
                DirectedEdge::Horizontal { y, mut x_from, mut x_to } => {
                    if x_from > x_to {
                        mem::swap(&mut x_from, &mut x_to);
                    }
                    for i in x_from..x_to {
                        row_edges[(i, y)].1.push(edge_variable.clone());
                    }
                }
                DirectedEdge::Vertical { x, mut y_from, mut y_to } => {
                    if y_from > y_to {
                        mem::swap(&mut y_from, &mut y_to);
                    }
                    for j in y_from..y_to {
                        column_edges[(x, j)].1.push(edge_variable.clone());
                    }
                }
            }
        }
        offset_variables.push(path_variables);
    }

    for i in 0..=x {
        for j in 0..y {
            let edges = &column_edges[(i, j)].1;
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

    for i in 0..x {
        for j in 0..=y {
            let edges = &row_edges[(i, j)].1;
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
        for (edge, offset) in path.iter().zip(offsets) {
            let offset = *offset as f64 * line_width;
            match *edge {
                DirectedEdge::Horizontal { y, mut x_from, mut x_to } => {
                    if x_from > x_to {
                        mem::swap(&mut x_from, &mut x_to);
                    }
                    for i in x_from..x_to {
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
                DirectedEdge::Vertical { x, mut y_from, mut y_to } => {
                    if y_from > y_to {
                        mem::swap(&mut y_from, &mut y_to);
                    }
                    for j in y_from..y_to {
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
