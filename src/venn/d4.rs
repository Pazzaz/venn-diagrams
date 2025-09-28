use crate::Polyomino;

use super::{to_polymonio, check_diagram};

pub const FOUR: [Polyomino<5, 5>; 4] = to_polymonio(STR);

const_assert!(check_diagram(FOUR));

#[rustfmt::skip]
const STR: [[&str; 5]; 5] = [
    ["",   "A",   "",     "B",   "",  ],
    ["",   "AC",  "ABC",  "BC",  "C", ],
    ["CD", "ACD", "ABCD", "BCD", "",  ],
    ["",   "AD",  "ABD",  "BD",  "D", ],
    ["",   "",    "AB",   "",    "",  ],
];