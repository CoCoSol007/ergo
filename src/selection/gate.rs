use crate::{logic::Gate, selection::CustomCollider};
use bevy::prelude::*;

impl CustomCollider for Gate {
    fn contains_point(&self, local_point: Vec2) -> bool {
        self.contains_point(local_point)
    }
}

impl Gate {
    pub fn contains_point(&self, local_point: Vec2) -> bool {
        match self {
            Gate::And(_, _) => self.check_and(local_point),
            Gate::Or(_, _) => self.check_or(local_point),
            Gate::Not(_) => self.check_not(local_point),
        }
    }

    fn check_and(&self, p: Vec2) -> bool {
        let width = 50.0;
        let height = 40.0;
        let radius = height / 2.0;

        let in_rect = p.x >= -width / 2.0 && p.x <= 0.0 && p.y.abs() <= radius;

        let in_circle = p.x > 0.0 && p.length_squared() <= (radius * radius);

        in_rect || in_circle
    }

    fn check_not(&self, p: Vec2) -> bool {
        let width = 40.0;
        let height = 30.0;
        let bubble_radius = 6.0;
        let triangle_w = width - (bubble_radius * 2.0);
        let left_x = -width / 2.0;
        let tip_x = left_x + triangle_w;

        let circle_center = Vec2::new(tip_x + bubble_radius - 2.0, 0.0);
        if p.distance_squared(circle_center) <= bubble_radius * bubble_radius {
            return true;
        }

        if p.x > tip_x || p.x < left_x {
            return false;
        }

        let t = (p.x - left_x) / triangle_w;
        let max_y_at_x = (height / 2.0) * (1.0 - t);

        p.y.abs() <= max_y_at_x
    }

    fn check_or(&self, p: Vec2) -> bool {
        let width = 50.0;
        let height = 40.0;
        let half_h = height / 2.0;
        let left_x = -width / 2.0;
        let tip_x = width / 2.0;

        if p.x < left_x || p.x > tip_x || p.y.abs() > half_h {
            return false;
        }

        let t = (p.x - left_x) / width;

        let curve_indent = width * 0.15;
        if p.x < (left_x + curve_indent) {
            let normalized_y = p.y.abs() / half_h;
            let min_x = left_x + (1.0 - normalized_y) * curve_indent;
            if p.x < min_x {
                return false;
            }
        }

        let max_y = half_h * ((1.0 - t).sqrt());

        p.y.abs() <= (max_y * 1.1)
    }
}
