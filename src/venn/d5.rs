use super::venn_diagram::ConstVennDiagram;

/// A Venn diagram for 5 groups.
///
/// # Example
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/snapshots/examples__five.snap.svg"))]
pub const FIVE: ConstVennDiagram<5, 7, 7> = ConstVennDiagram::from_letters(STR);

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
