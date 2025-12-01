use bevy::{input::keyboard::KeyboardInput, prelude::*};

use crate::{cursor, logic::Gate};

pub struct CreationPlugin;
impl Plugin for CreationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_creation);
    }
}

fn handle_creation(
    mut inputs: MessageReader<KeyboardInput>,
    mut commands: Commands,
    cursor: Res<cursor::CursorPosition>,
) {
    for input in inputs.read() {
        if !input.state.is_pressed() {
            continue;
        }
        let mut entity = match input.key_code {
            KeyCode::KeyZ => commands.spawn(Gate::And(None, None)),
            KeyCode::KeyX => commands.spawn(Gate::Or(None, None)),
            KeyCode::KeyC => commands.spawn(Gate::Not(None)),
            KeyCode::KeyV => commands.spawn(crate::logic::LogicButton),
            _ => continue,
        };

        entity.insert(Transform::from_translation(Vec3::new(
            cursor.in_world.x,
            cursor.in_world.y,
            0.0,
        )));
    }
}
