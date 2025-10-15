use crate::{diagram::ConstVennDiagram, polyomino::ConstPolyomino};

mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d8;

pub use d2::TWO;
pub use d3::THREE;
pub use d4::FOUR;
pub use d5::FIVE;
pub use d6::SIX;
pub use d8::EIGHT;

const fn to_polymonio<const N: usize, const X: usize, const Y: usize>(
    boxes: [[&str; X]; Y],
) -> ConstVennDiagram<N, X, Y> {
    let mut out = [ConstPolyomino::empty(); N];
    let mut y = 0;
    while y != Y {
        let mut x = 0;
        while x != X {
            let s = boxes[y][x].as_bytes();
            let mut c_i = 0;
            while c_i != s.len() {
                let c = s[c_i];

                let p = c - b'A';
                out[p as usize].values[y][x] = true;
                c_i += 1;
            }
            x += 1;
        }
        y += 1;
    }
    ConstVennDiagram::new(out)
}
