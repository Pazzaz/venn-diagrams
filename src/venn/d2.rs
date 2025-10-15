use super::venn_diagram::ConstVennDiagram;

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
