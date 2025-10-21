use svg::node::element::{Circle, Group, SVG};

use super::{InnerOffset, config::DiagramConfig};

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

pub(super) fn draw_circle(
    cx: f64,
    cy: f64,
    mask: &[bool],
    out: SVG,
    config: &DiagramConfig,
    values: &[f64],
    colors: &[String],
) -> SVG {
    let n = mask.len();
    debug_assert!(values.len() == n && colors.len() == n);
    let radius = config.radius;
    let c = std::f64::consts::TAU * radius;
    let mut group = Group::new().set("transform", format!("rotate(-90 {cx} {cy})"));

    let coalition: Coalition = Coalition::from_values(mask, values);

    let mut added = 0.0;
    for i in 0..n {
        if !mask[i] {
            continue;
        }
        let size = values[i];
        let color = &colors[i];

        let mut circle = Circle::new()
            .set("r", radius)
            .set("cx", cx)
            .set("cy", cy)
            .set("fill", "transparent")
            .set("stroke", color.as_str())
            .set("stroke-width", radius * 2.0)
            .set("stroke-dasharray", format!("{}, {}", c * size, c));
        if added != 0.0 {
            circle = circle.set("stroke-dashoffset", -added);
        }

        added += c * size;
        group = group.add(circle);
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

pub(super) enum Coalition {
    Below,
    Edge,
    Above,
}

impl Coalition {
    fn from_values(mask: &[bool], values: &[f64]) -> Coalition {
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
            Coalition::Below
        } else if on_edge {
            Coalition::Edge
        } else {
            Coalition::Above
        }
    }
}

pub enum CirclePlacement {
    Basic,
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
            CirclePlacement::Basic => (x as f64 + 0.5, (y as f64) + 0.5),
            CirclePlacement::SquareCenter => {
                let cx = x as f64;
                let cy = y as f64;

                let above_y = cy + internal_offset.above;
                let below_y = cy + 1.0 + internal_offset.below;
                let left_x = cx + internal_offset.left;
                let right_x = cx + 1.0 + internal_offset.right;

                let cy = f64::midpoint(above_y, below_y);
                let cx = f64::midpoint(left_x, right_x);
                (cx, cy)
            }
        }
    }
}
