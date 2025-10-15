use super::to_polymonio;
use crate::diagram::ConstVennDiagram;

pub const TWO: ConstVennDiagram<2, 3, 1> = to_polymonio(STR);

const_assert!(TWO.complete());

#[rustfmt::skip]
const STR: [[&str; 3]; 1] = [
    ["A",  "AB", "B",],
];
