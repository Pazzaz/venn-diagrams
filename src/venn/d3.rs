use super::to_polymonio;
use crate::diagram::ConstVennDiagram;

pub const THREE: ConstVennDiagram<3, 5, 2> = to_polymonio(STR);

const_assert!(THREE.complete());

#[rustfmt::skip]
const STR: [[&str; 5]; 2] = [
    ["A", "AB", "ABC", "B",  "",  ],
    ["",  "",   "AC",  "BC", "C", ],
];
