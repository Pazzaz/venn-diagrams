use super::{check_diagram, to_polymonio};
use crate::ConstPolyomino;

pub const FOUR: [ConstPolyomino<5, 5>; 4] = to_polymonio(STR);

const_assert!(check_diagram(FOUR));

#[rustfmt::skip]
const STR: [[&str; 5]; 5] = [
    ["",   "A",   "",     "B",   "",  ],
    ["",   "AC",  "ABC",  "BC",  "C", ],
    ["CD", "ACD", "ABCD", "BCD", "",  ],
    ["",   "AD",  "ABD",  "BD",  "D", ],
    ["",   "",    "AB",   "",    "",  ],
];
