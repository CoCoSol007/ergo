use bevy::{prelude::*, window::PrimaryWindow};

pub struct CursorPlugin;
impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorPosition {
            in_world: Vec2::ZERO,
            in_screen: Vec2::ZERO,
        })
        .add_systems(Update, update_cursor_position);
    }
}

#[derive(Resource)]
pub struct CursorPosition {
    pub in_world: Vec2,
    pub in_screen: Vec2,
}

fn update_cursor_position(
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
    mut cursor_position: ResMut<CursorPosition>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_query.single().unwrap();

    if let Some(screen_pos) = window.unwrap().cursor_position() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, screen_pos) {
            cursor_position.in_world = world_pos;
            cursor_position.in_screen = screen_pos;
        }
    }
}
