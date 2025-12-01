use crate::{logic::LogicButton, selection::CustomCollider};
use bevy::prelude::*;

impl CustomCollider for LogicButton {
    fn contains_point(&self, local_point: Vec2) -> bool {
        self.contains_point(local_point)
    }
}

impl LogicButton {
    pub fn contains_point(&self, local_point: Vec2) -> bool {
        let radius = 10.0;
        local_point.length_squared() <= radius * radius
    }
}
