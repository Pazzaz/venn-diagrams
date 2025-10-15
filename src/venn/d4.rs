use super::{check_diagram, to_polymonio};
use crate::diagram::ConstVennDiagram;

pub const FOUR: ConstVennDiagram<4, 5, 5> = to_polymonio(STR);

const_assert!(check_diagram(FOUR));

#[rustfmt::skip]
const STR: [[&str; 5]; 5] = [
    ["",   "A",   "",     "B",   "",  ],
    ["",   "AC",  "ABC",  "BC",  "C", ],
    ["CD", "ACD", "ABCD", "BCD", "",  ],
    ["",   "AD",  "ABD",  "BD",  "D", ],
    ["",   "",    "AB",   "",    "",  ],
];
