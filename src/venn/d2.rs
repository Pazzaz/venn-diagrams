use super::{check_diagram, to_polymonio};
use crate::ConstPolyomino;

pub const TWO: [ConstPolyomino<3, 1>; 2] = to_polymonio(STR);

const_assert!(check_diagram(TWO));

#[rustfmt::skip]
const STR: [[&str; 3]; 1] = [
    ["A",  "AB", "B",],
];
