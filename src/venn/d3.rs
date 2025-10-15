use super::{check_diagram, to_polymonio};
use crate::ConstPolyomino;

pub const THREE: [ConstPolyomino<5, 2>; 3] = to_polymonio(STR);

const_assert!(check_diagram(THREE));

#[rustfmt::skip]
const STR: [[&str; 5]; 2] = [
    ["A", "AB", "ABC", "B",  "",  ],
    ["",  "",   "AC",  "BC", "C", ],
];
