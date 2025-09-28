use crate::Polyomino;

use super::{to_polymonio, check_diagram};


pub const THREE: [Polyomino<5, 2>; 3] = to_polymonio(STR);

const_assert!(check_diagram(THREE));

#[rustfmt::skip]
const STR: [[&str; 5]; 2] = [
    ["A", "AB", "ABC", "B",  "",  ],
    ["",  "",   "AC",  "BC", "C", ],
];