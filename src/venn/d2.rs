use crate::Polyomino;

use super::{to_polymonio, check_diagram};

pub const TWO: [Polyomino<3, 1>; 2] = to_polymonio(STR);

const_assert!(check_diagram(TWO));

#[rustfmt::skip]
const STR: [[&str; 3]; 1] = [
    ["A",  "AB", "B",],
];