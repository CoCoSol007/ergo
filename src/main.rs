pub mod camera;
pub mod cursor;
pub mod grid;

use bevy::prelude::*;

use crate::{camera::CameraPlugin, cursor::CursorPlugin, grid::GridPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(CursorPlugin)
        .add_plugins(GridPlugin)
        .run();
}
