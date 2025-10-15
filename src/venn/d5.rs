use super::to_polymonio;
use crate::diagram::ConstVennDiagram;

pub const FIVE: ConstVennDiagram<5, 7, 7> = to_polymonio(STR);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_complete() {
        assert!(FIVE.complete());
    }
}
