use crate::renderer::{
    gate::GateRendererPlugin, link::RendererLinkPlugin, shadow::ShadowRendererPlugin,
};
use bevy::prelude::*;

mod gate;
mod link;
pub mod shadow;

pub struct RendererPlugin;
impl Plugin for RendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GateRendererPlugin)
            .add_plugins(ShadowRendererPlugin)
            .add_plugins(RendererLinkPlugin);
    }
}
