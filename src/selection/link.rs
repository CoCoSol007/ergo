use crate::{link::Link, selection::CustomCollider};
use bevy::prelude::*;

impl CustomCollider for Link {
    fn contains_point(&self, local_point: Vec2) -> bool {
        self.contains_point(local_point)
    }
}

impl Link {
    pub fn contains_point(&self, local_point: Vec2) -> bool {
        let thickness = 6.0;

        let start = self.from_position;
        let end = self.to_position;

        let line_vec = end - start;
        let line_len_sq = line_vec.length_squared();

        if line_len_sq == 0.0 {
            return local_point.distance_squared(start) <= (thickness / 2.0) * (thickness / 2.0);
        }

        let t = ((local_point - start).dot(line_vec) / line_len_sq).clamp(0.0, 1.0);
        let projection = start + t * line_vec;

        local_point.distance_squared(projection) <= (thickness / 2.0) * (thickness / 2.0)
    }
}
