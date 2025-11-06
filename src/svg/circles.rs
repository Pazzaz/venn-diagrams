use std::f64::{self, consts::PI};

use svg::node::element::{Circle, Group, Path, SVG, path::Data};

use super::{InnerOffset, config::DiagramConfig};

/// Configuration for a circle around a pie chart, see [`DiagramConfig`].
#[derive(Debug, Clone)]
pub struct CircleConfig {
    opacity: f64,
    color: String,
}

impl CircleConfig {
    pub fn new(opacity: f64, color: String) -> Self {
        Self { opacity, color }
    }
}

fn x_pos(angle: f64) -> f64 {
    (-angle + PI / 2.0).cos()
}

fn y_pos(angle: f64) -> f64 {
    (-angle + PI / 2.0).sin()
}

// Outline piece of pie
fn piece(cx: f64, cy: f64, r: f64, start: f64, end: f64) -> Path {
    // They are both positive
    debug_assert!(0.0 <= start);
    debug_assert!(0.0 <= end);
    debug_assert!(start <= end);

    // We clamp start and end, sometimes end is slightly larger than TAU
    let start = start.clamp(0.0, f64::consts::TAU);
    let end = end.clamp(0.0, f64::consts::TAU);

    let start_x = cx + x_pos(start) * r;
    let start_y = cy - y_pos(start) * r;

    let end_x = cx + x_pos(end) * r;
    let end_y = cy - y_pos(end) * r;

    let large_arc = if end - start <= PI { 0 } else { 1 };

    let elliptical_params = (r, r, 0, large_arc, 1, end_x, end_y);

    let data = Data::new()
        .move_to((cx, cy))
        .line_to((start_x, start_y))
        .elliptical_arc_to(elliptical_params)
        .close();

    Path::new().set("d", data)
}

pub(super) fn draw_circle(
    cx: f64,
    cy: f64,
    mask: &[bool],
    out: SVG,
    config: &DiagramConfig,
    values: &[f64],
    colors: &[&str],
) -> SVG {
    let n = mask.len();
    debug_assert!(values.len() == n && colors.len() == n);
    let radius = config.radius;
    let mut group = Group::new();

    let coalition: Coalition = Coalition::from_values(mask, values);

    let mut added = 0.0;
    for i in 0..n {
        if !mask[i] {
            continue;
        }
        let size = values[i];
        let color = colors[i];

        let end = added + size;

        let piece = piece(cx, cy, 2.0 * radius, f64::consts::TAU * added, f64::consts::TAU * end);

        added = end;

        let out = piece.set("fill", color).set("stroke", "none");
        group = group.add(out);
    }

    let circle_config = config.circle_config(coalition);

    let circle = Circle::new()
        .set("r", radius * 2.0)
        .set("cx", cx)
        .set("cy", cy)
        .set("fill", "transparent")
        .set("stroke", circle_config.color.as_str())
        .set("stroke-width", 0.025);

    if circle_config.opacity != 1.0 {
        group = group.set("opacity", circle_config.opacity);
    }
    group = group.add(circle);

    out.add(group)
}

#[derive(Debug, Clone, Copy)]
pub(super) enum Coalition {
    Below,
    Edge,
    Above,
}

impl Coalition {
    fn from_values(mask: &[bool], values: &[f64]) -> Self {
        let n = mask.len();
        debug_assert!(n == values.len());
        let mut total: f64 = 0.0;
        for i in 0..n {
            if !mask[i] {
                continue;
            }
            total += values[i];
        }

        let mut on_edge = true;
        for i in 0..n {
            if !mask[i] {
                continue;
            }
            if total - values[i] >= 0.5 {
                on_edge = false;
            }
        }

        if total < 0.5 {
            Self::Below
        } else if on_edge {
            Self::Edge
        } else {
            Self::Above
        }
    }
}

/// Method to decide where each pie chart should be placed.
#[derive(Debug, Clone, Copy, Default)]
pub enum CirclePlacement {
    /// Each circle is placed on it's grid position.
    Basic,

    /// Places circles in the middle of the intersections inner bounding square.
    #[default]
    SquareCenter,
}

impl CirclePlacement {
    pub(super) fn get_circle_pos(
        &self,
        x: usize,
        y: usize,
        internal_offset: InnerOffset,
    ) -> (f64, f64) {
        match self {
            Self::Basic => (x as f64 + 0.5, (y as f64) + 0.5),
            Self::SquareCenter => {
                let cx = x as f64;
                let cy = y as f64;

                let above_y = cy + internal_offset.above;
                let below_y = cy + 1.0 - internal_offset.below;
                let left_x = cx + internal_offset.left;
                let right_x = cx + 1.0 - internal_offset.right;

                let cy = f64::midpoint(above_y, below_y);
                let cx = f64::midpoint(left_x, right_x);
                (cx, cy)
            }
        }
    }
}
