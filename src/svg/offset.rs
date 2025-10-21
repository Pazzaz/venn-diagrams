use std::{cmp::Ordering, mem};

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

#[derive(Debug, Default, Clone)]
struct EdgeInfo {
    from: usize,
    to: usize,
    direction: Option<Direction>,
    len: usize,
    p_i: usize,
    e_i: usize,
}

pub(super) fn get_offsets(
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

        for j in 0..y {
            let mut min_pos: usize = usize::MAX;
            let mut max_pos: usize = usize::MIN;
            for k in 0..len {
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

        for j in 0..x {
            let mut min_pos: usize = usize::MAX;
            let mut max_pos: usize = usize::MIN;
            for k in 0..len {
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
