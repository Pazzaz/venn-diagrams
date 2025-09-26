#[macro_use]
extern crate static_assertions;

fn main() {
    let colors_s = ["green", "blue", "red", "yellow", "purple", "hotpink", "white", "brown"];
    let colors = colors_s.map(|x| x.to_string());
    let diagram = Diagram { venns: EIGHT, values: [0.0; 8], colors };

    diagram.to_svg();
}

#[derive(Debug, Clone, Copy)]
struct Polyomino<const X: usize, const Y: usize>([[bool; X]; Y]);

const fn to_polymonio<const N: usize, const X: usize, const Y: usize>(
    boxes: [[&str; X]; Y],
) -> [Polyomino<X, Y>; N] {
    let mut out = [Polyomino([[false; X]; Y]); N];
    let mut y = 0;
    while y != Y {
        let mut x = 0;
        while x != X {
            let s = boxes[y][x].as_bytes();
            let mut c_i = 0;
            while c_i != s.len() {
                let c = s[c_i];

                let p = c - b'A';
                out[p as usize].0[y][x] = true;
                c_i += 1;
            }
            x += 1;
        }
        y += 1;
    }
    out
}

#[rustfmt::skip]
mod strings {
    pub const TWO_STR: [[&str; 3]; 1] = [
        ["A",  "AB", "B",],
    ];
    
    pub const THREE_STR: [[&str; 5]; 2] = [
        ["A", "AB", "ABC", "B",  "",  ],
        ["",  "",   "AC",  "BC", "C", ],
    ];
    
    pub const FOUR_STR: [[&str; 5]; 5] = [
        ["",   "A",   "",     "B",   "",  ],
        ["",   "AC",  "ABC",  "BC",  "C", ],
        ["CD", "ACD", "ABCD", "BCD", "",  ],
        ["",   "AD",  "ABD",  "BD",  "D", ],
        ["",   "",    "AB",   "",    "",  ],
    ];
    
    pub const FIVE_STR: [[&str; 7]; 7] = [
        ["",  "",    "",      "",     "D",    "CD",   "",   ],
        ["E", "BCE", "AE",    "ACDE", "DE",   "CE",   "",   ],
        ["",  "BC",  "ACE",   "ACBE", "CDE",  "BCDE", "",   ],
        ["",  "BDE", "ABCDE", "ABD",  "BCD",  "ABC",  "",   ],
        ["",  "BD",  "ABDE",  "ADE",  "ABCD", "ACD",  "AD", ],
        ["",  "BE",  "ABE",   "",     "AC",   "C",    "A",  ],
        ["",  "B",   "AB",    "",     "",     "",     "",   ],
    ];
}

const EIGHT_GRIDS: [[&str; 15]; 8] = [
    [
        "01110111101110111",
        "00010001001000100",
        "11011101111011100",
        "01000100100010000",
        "01110111101110111",
        "00010001001000100",
        "11011101111011100",
        "01110111101110111",
        "01000100100010000",
        "11011101111011100",
        "00010001001000100",
        "01110111101110111",
        "01000100100010000",
        "11011101111011100",
        "00010001001000100",
    ],
    [
        "11011101111011100",
        "01000100100010000",
        "01110111101110111",
        "00010001001000100",
        "11011101111011100",
        "01000100100010000",
        "01110111101110111",
        "11011101111011100",
        "00010001001000100",
        "01110111101110111",
        "01000100100010000",
        "11011101111011100",
        "00010001001000100",
        "01110111101110111",
        "01000100100010000",
    ],
    [
        "00110000110000000",
        "00111111110011111",
        "00111111110011111",
        "00000011000011000",
        "00000011000011000",
        "11110011111111000",
        "11110011111111000",
        "00111111110011111",
        "00111111110011111",
        "00110000110000000",
        "00110000110000000",
        "11110011111111000",
        "11110011111111000",
        "00000011000011000",
        "00000011000011000",
    ],
    [
        "00000011000011000",
        "11110011111111000",
        "11110011111111000",
        "00110000110000000",
        "00110000110000000",
        "00111111110011111",
        "00111111110011111",
        "11110011111111000",
        "11110011111111000",
        "00000011000011000",
        "00000011000011000",
        "00111111110011111",
        "00111111110011111",
        "00110000110000000",
        "00110000110000000",
    ],
    [
        "00001111000000000",
        "00001111000000000",
        "00001111000000000",
        "00001111111111111",
        "00001111111111111",
        "00001111111111111",
        "00001111111111111",
        "11111111111100000",
        "11111111111100000",
        "11111111111100000",
        "11111111111100000",
        "00000000111100000",
        "00000000111100000",
        "00000000111100000",
        "00000000111100000",
    ],
    [
        "00000000111100000",
        "00000000111100000",
        "00000000111100000",
        "11111111111100000",
        "11111111111100000",
        "11111111111100000",
        "11111111111100000",
        "00001111111111111",
        "00001111111111111",
        "00001111111111111",
        "00001111111111111",
        "00001111000000000",
        "00001111000000000",
        "00001111000000000",
        "00001111000000000",
    ],
    [
        "00000000111111110",
        "00000000111111110",
        "00000000111111110",
        "00000000111111110",
        "00000000111111110",
        "00000000111111110",
        "00000000111111110",
        "00000000111111111",
        "00000000111111111",
        "00000000111111111",
        "00000000111111111",
        "00000000111111111",
        "00000000111111111",
        "00000000111111111",
        "00000000111111111",
    ],
    [
        "00000000000000000",
        "00000000000000000",
        "00000000000000000",
        "00000000000000000",
        "00000000000000000",
        "00000000000000000",
        "00000000000000000",
        "11111111111111110",
        "11111111111111110",
        "11111111111111110",
        "11111111111111110",
        "11111111111111110",
        "11111111111111110",
        "11111111111111110",
        "11111111111111110",
    ],
];

const SCALE: usize = 20;

struct Diagram<const N: usize, const X: usize, const Y: usize> {
    venns: [Polyomino<X, Y>; N],
    values: [f64; N],
    colors: [String; N],
}

use std::{path, vec};

use svg::{
    Document,
    node::element::{Path, path::Data},
};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Direction {
    Horizontal { y: usize, x1: usize, x2: usize },
    Vertical { x: usize, y1: usize, y2: usize },
}

impl Direction {
    fn len(&self) -> usize {
        match self {
            &Horizontal { x1, x2, .. } => x1.abs_diff(x2),
            &Vertical { y1, y2, .. } => y1.abs_diff(y2),
        }
    }
}

fn endpoints(d: &Direction) -> ((usize, usize), (usize, usize)) {
    match d {
        &Horizontal { y, x1, x2 } => ((x1, y), (x2, y)),
        &Vertical { x, y1, y2 } => ((x, y1), (x, y2)),
    }
}

fn from_endpoints((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> Direction {
    if x1 == x2 {
        Vertical { x: x1, y1, y2 }
    } else if y1 == y2 {
        Horizontal { y: y1, x1, x2 }
    } else {
        panic!("Invalid endpoints");
    }
}

fn connected(a: &Direction, b: &Direction) -> bool {
    assert!(a != b);
    let (p1, p2) = endpoints(a);
    let (p3, p4) = endpoints(b);
    p1 == p3 || p1 == p4 || p2 == p3 || p2 == p4
}

fn combine(a: &Direction, b: &Direction) -> Option<Direction> {
    if !connected(a, b) {
        return None;
    }

    match (a, b) {
        (&Horizontal { x1, x2, y }, &Horizontal { x1: x3, x2: x4, .. }) => {
            Some(Horizontal { y, x1: x1.min(x3), x2: x2.max(x4) })
        }
        (Horizontal { .. }, Vertical { .. }) => None,
        (Vertical { .. }, Horizontal { .. }) => None,
        (&Vertical { x, y1, y2 }, &Vertical { y1: y3, y2: y4, .. }) => {
            Some(Vertical { x, y1: y1.min(y3), y2: y2.max(y4) })
        }
    }
}

use Direction::*;

impl<const N: usize, const X: usize, const Y: usize> Diagram<N, X, Y> {
    fn to_svg(&self) {
        let mut out = Document::new()
            .set(
                "viewBox",
                (-((SCALE / 2) as i32), -((SCALE / 2) as i32), (X + 1) * SCALE, (Y + 1) * SCALE),
            )
            .set("width", "200px")
            .set("height", "200px");

        let mut polys: Vec<Vec<Direction>> = Vec::new();

        for i in 0..N {
            // 1. List all the edges
            let poly = self.venns[i].0;
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
                .set("fill", "none")
                .set("stroke", color.clone())
                .set("stroke-width", 1)
                .set("d", data);
            out = out.add(path);
        }

        // for x in 0..X {
        //     for y in 0..Y {
        //         // 2. Remove "double edges"
        //         // 3. Create paths
        //         // 4. Draw paths

        //         let v: f64 = self
        //             .venns
        //             .iter()
        //             .enumerate()
        //             .map(|(i, p)| if p.0[y][x] { self.values[i] } else { 0.0 })
        //             .sum();

        //         let clamped_v = v.clamp(0.0, 1.0);

        //         let data = Data::new()
        //             .move_to((x * 10, y * 10))
        //             .line_by((0, 10))
        //             .line_by((10, 0))
        //             .line_by((0, -10))
        //             .close();

        //         let path = Path::new()
        //             .set("fill", "none")
        //             .set("stroke", "black")
        //             .set("stroke-width", 1)
        //             .set("d", data);

        //         out = out.add(path);
        //     }
        // }

        svg::save("image.svg", &out).unwrap();
    }
}

const fn grid_to_polyomino<const X: usize, const Y: usize>(grid: [&str; Y]) -> Polyomino<X, Y> {
    let mut out = [[false; X]; Y];

    let mut y = 0;
    while y != Y {
        let row = grid[y].as_bytes();
        let mut x = 0;
        while x != X {
            if row[x] == b'1' {
                out[y][x] = true;
            }
            x += 1;
        }
        y += 1;
    }
    Polyomino(out)
}

const fn to_polymonio_2<const N: usize, const X: usize, const Y: usize>(
    grids: [[&str; Y]; N],
) -> [Polyomino<X, Y>; N] {
    let mut out = [Polyomino([[false; X]; Y]); N];
    let mut i = 0;
    while i != N {
        out[i] = grid_to_polyomino(grids[i]);
        i += 1;
    }

    out
}

const TWO: [Polyomino<3, 1>; 2] = to_polymonio(strings::TWO_STR);
const THREE: [Polyomino<5, 2>; 3] = to_polymonio(strings::THREE_STR);
const FOUR: [Polyomino<5, 5>; 4] = to_polymonio(strings::FOUR_STR);
const FIVE: [Polyomino<7, 7>; 5] = to_polymonio(strings::FIVE_STR);
const EIGHT: [Polyomino<17, 15>; 8] = to_polymonio_2(EIGHT_GRIDS);

const_assert!(check_diagram(TWO));
const_assert!(check_diagram(THREE));
const_assert!(check_diagram(FOUR));
const_assert!(check_diagram(FIVE));
const_assert!(check_diagram(EIGHT));

const fn empty_at<const N: usize, const X: usize, const Y: usize>(
    polys: &[Polyomino<X, Y>; N],
    x: usize,
    y: usize,
) -> bool {
    let mut i = 0;
    while i != N {
        let p2 = polys[i].0[y][x];
        if p2 {
            return false;
        }
        i += 1;
    }
    true
}

const fn different_at<const N: usize, const X: usize, const Y: usize>(
    polys: &[Polyomino<X, Y>; N],
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
) -> bool {
    let mut i = 0;
    while i != N {
        let p1 = polys[i].0[y1][x1];
        let p2 = polys[i].0[y2][x2];
        if p1 != p2 {
            return true;
        }
        i += 1;
    }
    false
}

// TODO: Can we check connectivity too?
const fn check_diagram<const N: usize, const X: usize, const Y: usize>(
    parts: [Polyomino<X, Y>; N],
) -> bool {
    // We check that each group is the right size
    let count_goal: u64 = 1 << (N - 1);
    let mut i = 0;
    while i != N {
        let mut count = 0;
        let part = parts[i].0;
        let mut x = 0;
        while x != X {
            let mut y = 0;
            while y != Y {
                if part[y][x] {
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
                    let all_false_1 = empty_at(&parts, x1, y1);
                    let all_false_2 = empty_at(&parts, x2, y2);
                    if !eq_coord
                        && !all_false_1
                        && !all_false_2
                        && !different_at(&parts, x1, y1, x2, y2)
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
