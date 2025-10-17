use super::venn_diagram::ConstVennDiagram;

/// A Venn diagram for 3 groups.
///
/// # Example
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/snapshots/examples__three.snap.svg"))]
pub const THREE: ConstVennDiagram<3, 5, 2> = ConstVennDiagram::from_letters(STR);

#[rustfmt::skip]
const STR: [[&str; 5]; 2] = [
    ["A", "AB", "ABC", "B",  "",  ],
    ["",  "",   "AC",  "BC", "C", ],
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_complete() {
        assert!(THREE.complete());
    }
}
