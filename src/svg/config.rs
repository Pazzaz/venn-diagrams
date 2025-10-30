pub use super::corner::CornerStyle;
use super::{
    Coalition,
    circles::{CircleConfig, CirclePlacement},
};

/// Configuration of a Venn diagram.
#[derive(Debug, Clone)]
pub struct DiagramConfig {
    /// Width of each polyomino border.
    pub line_width: f64,

    /// Circle radius.
    pub radius: f64,

    /// Circular border of the pie chart, when sum of sets in intersection is
    /// less than 50%.
    pub circle_below: CircleConfig,

    /// Circular border of the pie chart, when sum of sets in intersection is
    /// more than 50%, but removing any set would cause the sum to be less than
    /// 50%.
    pub circle_edge: CircleConfig,

    /// Circular border of the pie chart, when sum of sets in intersection is
    /// more than 50%.
    pub circle_above: CircleConfig,

    /// Method to decide where each pie chart should be placed.
    pub circle_placement: CirclePlacement,

    /// How each corner should be drawn.
    pub corner_style: CornerStyle,

    /// Length from the corner that the border should start turning.
    pub corner_offset: f64,

    /// Scale factor for the width (in pixels) of the SVG, if any.
    pub width_mul: Option<f64>,

    /// Scale factor for the height (in pixels) of the SVG, if any.
    pub height_mul: Option<f64>,
}

impl Default for DiagramConfig {
    fn default() -> Self {
        Self {
            line_width: 0.05,
            radius: 0.175,
            circle_below: CircleConfig::new(0.3, String::from("red")),
            circle_edge: CircleConfig::new(1.0, String::from("white")),
            circle_above: CircleConfig::new(0.3, String::from("green")),
            circle_placement: CirclePlacement::default(),
            corner_style: CornerStyle::Smooth,
            corner_offset: 0.15,
            width_mul: Some(80.0),
            height_mul: None,
        }
    }
}

impl DiagramConfig {
    pub(super) fn circle_config(&self, coalition: Coalition) -> &CircleConfig {
        match coalition {
            Coalition::Below => &self.circle_below,
            Coalition::Edge => &self.circle_edge,
            Coalition::Above => &self.circle_above,
        }
    }
}
