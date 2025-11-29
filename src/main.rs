pub mod camera;
pub mod cursor;

use bevy::prelude::*;

use crate::{camera::CameraPlugin, cursor::CursorPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(CursorPlugin)
        .run();
}
