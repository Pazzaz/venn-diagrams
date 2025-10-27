#![cfg(feature = "optimize")]

use z3::{
    Optimize, SatResult,
    ast::{Bool, Int},
};

use crate::{
    direction::{DirectedEdge, Direction, Edge},
    matrix::Matrix,
    svg::Diagonal,
};

pub(super) fn get_offsets(
    x: usize,
    y: usize,
    combined_paths: &[Vec<DirectedEdge>],
) -> Vec<Vec<i32>> {
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

    let f = |values: &Vec<Int>| {
        debug_assert!(values.len() <= n);
        let each_side = (n / 2) as i32;

        for range in values.len()..n {
            let mut parts: Vec<Bool> = Vec::new();
            for start in -each_side..=each_side {
                let end = start + range as i32 - 1;
                if end > each_side {
                    break;
                }
                let mut edges_contained: Vec<Bool> = Vec::new();
                for edge in values {
                    edges_contained.extend_from_slice(&[edge.ge(start), edge.le(end)]);
                }
                if !edges_contained.is_empty() {
                    parts.push(Bool::and(&edges_contained));
                }
            }
            if !parts.is_empty() {
                let b = Bool::or(&parts);
                solver.assert_soft(&b, 200 * (n - range), None);
            }
        }
    };

    // Include penalty for gaps
    for j in 0..=y {
        for i in 0..x {
            f(&row_edges[(i, j)]);
        }
    }

    for j in 0..y {
        for i in 0..=x {
            f(&column_edges[(i, j)]);
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

    offsets
}
