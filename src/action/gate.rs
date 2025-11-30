use bevy::{input::keyboard::KeyboardInput, prelude::*};

use crate::{cursor::CursorPosition, selection::Selected};

pub struct ActionGatePlugin;
impl Plugin for ActionGatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, suppr_items)
            .add_systems(Update, movement_item);
    }
}

fn suppr_items(
    query: Query<Entity, With<Selected>>,
    mut commands: Commands,
    mut inputs: MessageReader<KeyboardInput>,
) {
    for input in inputs.read() {
        if input.key_code == KeyCode::Delete && input.state.is_pressed() {
            for entity in query.iter() {
                commands.entity(entity).despawn_children();
                commands.entity(entity).despawn();
            }
        }
    }
}

fn movement_item(
    buttons: Res<ButtonInput<MouseButton>>,
    cursor: Res<CursorPosition>,
    mut pick_position: Local<Option<Vec2>>,
    mut query: Query<&mut Transform, (With<crate::selection::Selected>, Without<Camera>)>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        *pick_position = Some(cursor.in_world);
    }

    if buttons.just_released(MouseButton::Left) {
        *pick_position = None;
    }

    for mut transform in query.iter_mut() {
        transform.translation += match *pick_position {
            Some(pick_pos) => {
                let delta = cursor.in_world - pick_pos;
                Vec3::new(delta.x, delta.y, 0.0)
            }
            None => Vec3::ZERO,
        };
    }
    if pick_position.is_some() {
        *pick_position = Some(cursor.in_world);
    }
}
