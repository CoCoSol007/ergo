use bevy::{input::keyboard::KeyboardInput, prelude::*};

use crate::{
    cursor::CursorPosition,
    link::Link,
    logic::{Gate, update_logic_system},
    selection::Selected,
};

pub struct ActionGatePlugin;
impl Plugin for ActionGatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, suppr_items.after(update_logic_system))
            .add_systems(Update, movement_item);
    }
}

fn suppr_items(
    query: Query<(Entity, Has<Link>), With<Selected>>,
    mut commands: Commands,
    mut inputs: MessageReader<KeyboardInput>,
    mut all_query: Query<(Entity, &mut Gate)>,
    link_query: Query<&Link>,
) {
    for input in inputs.read() {
        if input.key_code == KeyCode::Delete && input.state.is_pressed() {
            for (entity, has_link) in query.iter() {
                commands.entity(entity).despawn_children();
                commands.entity(entity).despawn();

                if !has_link {
                    continue;
                }

                let link = link_query.get(entity).unwrap();

                for (other_entity, mut gate) in all_query.iter_mut() {
                    if other_entity == link.from {
                        *gate = match &mut *gate {
                            Gate::And(a, b) => {
                                let mut new_a = None;
                                let mut new_b = None;

                                if *a != Some(link.to) {
                                    new_a = a.clone();
                                }
                                if *b != Some(link.to) {
                                    new_b = b.clone();
                                }

                                Gate::And(new_a, new_b)
                            }
                            Gate::Or(a, b) => {
                                let mut new_a = None;
                                let mut new_b = None;

                                if *a != Some(link.to) {
                                    new_a = a.clone();
                                }
                                if *b != Some(link.to) {
                                    new_b = b.clone();
                                }

                                Gate::Or(new_a, new_b)
                            }
                            Gate::Not(a) => {
                                let mut new_a = None;

                                if *a != Some(link.to) {
                                    new_a = a.clone();
                                }

                                Gate::Not(new_a)
                            }
                        }
                    }
                }
            }
        }
    }
}

fn movement_item(
    buttons: Res<ButtonInput<MouseButton>>,
    cursor: Res<CursorPosition>,
    mut pick_position: Local<Option<Vec2>>,
    mut query: Query<
        &mut Transform,
        (
            With<crate::selection::Selected>,
            Without<Camera>,
            With<crate::selection::Moveable>,
        ),
    >,
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
