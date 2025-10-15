use super::{check_diagram, to_polymonio};
use crate::ConstPolyomino;

pub const FIVE: [ConstPolyomino<7, 7>; 5] = to_polymonio(STR);

const_assert!(check_diagram(FIVE));

#[rustfmt::skip]
const STR: [[&str; 7]; 7] = [
    ["",  "",    "",      "",     "D",    "CD",   "",   ],
    ["E", "BCE", "AE",    "ACDE", "DE",   "CE",   "",   ],
    ["",  "BC",  "ACE",   "ACBE", "CDE",  "BCDE", "",   ],
    ["",  "BDE", "ABCDE", "ABD",  "BCD",  "ABC",  "",   ],
    ["",  "BD",  "ABDE",  "ADE",  "ABCD", "ACD",  "AD", ],
    ["",  "BE",  "ABE",   "",     "AC",   "C",    "A",  ],
    ["",  "B",   "AB",    "",     "",     "",     "",   ],
];
