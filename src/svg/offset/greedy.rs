use std::cmp::Ordering;

use crate::{
    direction::{DirectedEdge, Direction, Edge},
    matrix::Matrix,
    svg::offset::EdgeInfo,
};

pub(super) fn get_offsets(
    width: usize,
    height: usize,
    combined_paths: &[Vec<DirectedEdge>],
) -> Vec<Vec<i32>> {
    let mut offsets: Vec<Vec<i32>> =
        combined_paths.iter().map(|x| vec![i32::MIN; x.len()]).collect();
    let mut columns = vec![Vec::new(); width + 1];
    let mut rows = vec![Vec::new(); height + 1];

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
    for i in 0..=width {
        // We sort each edge that's contained is this column such that we start by
        // placing the longest edges
        columns[i].sort_by(|a, b| a.len.cmp(&b.len).reverse());

        // the current column
        let column = &columns[i];

        let len = column.len();

        let middle = len / 2;

        let mut occupied = Matrix::new(len, height, false);

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
    for i in 0..=height {
        // We sort each edge that's contained is this row such that we start by
        // placing the longest edges
        rows[i].sort_by(|a, b| a.len.cmp(&b.len).reverse());

        let row = &rows[i];

        let len = row.len();

        let middle = len / 2;

        let mut occupied = Matrix::new(len, width, false);

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

    offsets
}
