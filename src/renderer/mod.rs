use crate::renderer::{gate::GateRendererPlugin, shadow::ShadowRendererPlugin};
use bevy::prelude::*;

mod gate;
mod shadow;

pub struct RendererPlugin;
impl Plugin for RendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GateRendererPlugin)
            .add_plugins(ShadowRendererPlugin);
    }
}
