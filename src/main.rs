pub mod camera;
pub mod cursor;
pub mod grid;
pub mod logic;

use bevy::prelude::*;

use crate::{camera::CameraPlugin, cursor::CursorPlugin, grid::GridPlugin, logic::LogicPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(CursorPlugin)
        .add_plugins(GridPlugin)
        .add_plugins(LogicPlugin)
        .run();
}
