pub mod action;
pub mod camera;
pub mod cursor;
pub mod grid;
pub mod logic;
pub mod renderer;
pub mod selection;

use bevy::prelude::*;

use crate::{
    action::ActionPlugin, camera::CameraPlugin, cursor::CursorPlugin, grid::GridPlugin,
    logic::LogicPlugin, renderer::RendererPlugin, selection::SelectionPlugin,
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
        .run();
}
