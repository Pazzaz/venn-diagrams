use crate::Polyomino;

mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d8;

pub use d8::EIGHT;

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
