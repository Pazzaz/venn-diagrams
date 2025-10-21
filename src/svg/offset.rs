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
