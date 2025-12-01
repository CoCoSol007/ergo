use bevy::{input::keyboard::KeyboardInput, prelude::*};

use crate::{
    cursor,
    logic::{Gate, Item},
    selection::Selected,
};

pub struct CreationPlugin;
impl Plugin for CreationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_creation)
            .add_systems(Update, connect_items);
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

fn connect_items(
    mut local_first: Local<Option<Entity>>,
    query_frist: Query<Entity, (With<Item>, With<Selected>)>,
    mut query_scd: Query<(&mut Gate, Entity), With<Selected>>,
    mut inputs: MessageReader<KeyboardInput>,
) {
    for input in inputs.read() {
        if input.state.is_pressed() && (input.key_code != KeyCode::KeyL) {
            local_first.take();
            continue;
        }
        if !input.state.is_pressed() {
            continue;
        }

        if let Some(first) = *local_first {
            for (mut gate, second) in query_scd.iter_mut() {
                if first == second {
                    continue;
                }
                match *gate {
                    Gate::And(ref mut a, ref mut b) | Gate::Or(ref mut a, ref mut b) => {
                        if a.is_none() {
                            *a = Some(first);
                        } else if b.is_none() {
                            *b = Some(first);
                        }
                    }
                    Gate::Not(ref mut a) => {
                        if a.is_none() {
                            *a = Some(first);
                        }
                    }
                }
            }
            *local_first = None;
        } else {
            for first in query_frist.iter() {
                *local_first = Some(first);
                break;
            }
        }
    }
}
