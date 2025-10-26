use super::venn_diagram::ConstVennDiagram;

/// A Venn diagram for 2 groups.
///
/// # Example
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/common/snapshots/normal__common__two.snap.svg"))]
pub const TWO: ConstVennDiagram<2, 3, 1> = ConstVennDiagram::from_letters(STR);

#[rustfmt::skip]
const STR: [[&str; 3]; 1] = [
    ["A",  "AB", "B",],
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_complete() {
        assert!(TWO.complete());
    }
}
