use super::venn_diagram::ConstVennDiagram;

pub const FOUR: ConstVennDiagram<4, 5, 5> = ConstVennDiagram::from_letters(STR);

#[rustfmt::skip]
const STR: [[&str; 5]; 5] = [
    ["",   "A",   "",     "B",   "",  ],
    ["",   "AC",  "ABC",  "BC",  "C", ],
    ["CD", "ACD", "ABCD", "BCD", "",  ],
    ["",   "AD",  "ABD",  "BD",  "D", ],
    ["",   "",    "AB",   "",    "",  ],
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_complete() {
        assert!(FOUR.complete());
    }
}
