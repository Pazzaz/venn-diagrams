pub use super::corner::CornerStyle;
use crate::svg::{
    Coalition,
    circles::{CircleConfig, CirclePlacement},
};

pub struct DiagramConfig {
    pub line_width: f64,
    pub radius: f64,
    pub circle_below: CircleConfig,
    pub circle_edge: CircleConfig,
    pub circle_above: CircleConfig,
    pub circle_placement: CirclePlacement,
    pub corner_style: CornerStyle,
    pub corner_offset: f64,
    pub width_mul: Option<f64>,
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
            circle_placement: CirclePlacement::SquareCenter,
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
