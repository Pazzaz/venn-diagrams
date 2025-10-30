use crate::polyomino::{ConstPolyomino, Polyomino};

#[derive(Debug, Clone)]
pub struct ConstVennDiagram<const N: usize, const X: usize, const Y: usize> {
    pub(crate) polyominos: [ConstPolyomino<X, Y>; N],
}

impl<const N: usize, const X: usize, const Y: usize> ConstVennDiagram<N, X, Y> {
    #[must_use]
    pub const fn new(polyominos: [ConstPolyomino<X, Y>; N]) -> Self {
        Self { polyominos }
    }

    /// The venn diagram consists of all subsets (except the empty subset)
    #[must_use]
    pub const fn complete(&self) -> bool {
        // We check that each group is the right size
        let count_goal: u64 = 1 << (N - 1);
        let mut i = 0;
        while i != N {
            let mut count = 0;
            let part = &self.polyominos[i];
            let mut x = 0;
            while x != X {
                let mut y = 0;
                while y != Y {
                    if part.values[y][x] {
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
                        let all_false_1 = self.empty_at(x1, y1);
                        let all_false_2 = self.empty_at(x2, y2);
                        if !eq_coord
                            && !all_false_1
                            && !all_false_2
                            && !self.different_at(x1, y1, x2, y2)
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

    #[must_use]
    pub const fn from_binary_str(grids: [[&str; Y]; N]) -> ConstVennDiagram<N, X, Y> {
        let mut out = [ConstPolyomino::empty(); N];
        let mut i = 0;
        while i != N {
            out[i] = ConstPolyomino::from_binary_str(grids[i]);
            i += 1;
        }

        ConstVennDiagram::new(out)
    }

    #[must_use]
    pub const fn from_letters(boxes: [[&str; X]; Y]) -> ConstVennDiagram<N, X, Y> {
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

    const fn empty_at(&self, x: usize, y: usize) -> bool {
        let mut i = 0;
        while i != N {
            let p2 = self.polyominos[i].values[y][x];
            if p2 {
                return false;
            }
            i += 1;
        }
        true
    }

    const fn different_at(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
        let mut i = 0;
        while i != N {
            let p1 = self.polyominos[i].values[y1][x1];
            let p2 = self.polyominos[i].values[y2][x2];
            if p1 != p2 {
                return true;
            }
            i += 1;
        }
        false
    }
}

impl<const N: usize, const X: usize, const Y: usize> From<ConstVennDiagram<N, X, Y>>
    for VennDiagram
{
    fn from(value: ConstVennDiagram<N, X, Y>) -> Self {
        Self {
            width: X,
            height: Y,
            polyominos: value.polyominos.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct VennDiagram {
    width: usize,
    height: usize,
    pub(crate) polyominos: Vec<Polyomino>,
}

impl VennDiagram {
    #[must_use]
    pub fn width(&self) -> usize {
        self.width
    }

    #[must_use]
    pub fn height(&self) -> usize {
        self.height
    }

    #[must_use]
    pub fn n(&self) -> usize {
        self.polyominos.len()
    }
}

impl Clone for VennDiagram {
    fn clone(&self) -> Self {
        Self { width: self.width, height: self.height, polyominos: self.polyominos.clone() }
    }

    fn clone_from(&mut self, source: &Self) {
        self.width = source.width;
        self.height = source.height;
        self.polyominos.clone_from(&source.polyominos);
    }
}
