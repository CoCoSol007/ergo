mod action;
mod camera;
mod creation;
mod cursor;
mod grid;
mod link;
mod logic;
mod renderer;
pub mod selection;

use bevy::prelude::*;

use crate::{
    action::ActionPlugin, camera::CameraPlugin, creation::CreationPlugin, cursor::CursorPlugin,
    grid::GridPlugin, link::LinkPlugin, logic::LogicPlugin, renderer::RendererPlugin,
    selection::SelectionPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(CursorPlugin)
        .add_plugins(GridPlugin)
        .add_plugins(LogicPlugin)
        .add_plugins(RendererPlugin)
        .add_plugins(SelectionPlugin)
        .add_plugins(ActionPlugin)
        .add_plugins(CreationPlugin)
        .add_plugins(LinkPlugin)
        .run();
}
