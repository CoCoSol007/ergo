use crate::renderer::shadow::ShadowEffect;
use crate::{
    cursor::CursorPosition,
    link::Link,
    logic::{Gate, LogicButton},
};
use bevy::prelude::*;
mod button;
mod gate;
mod link;

#[derive(Component, Default)]
#[require(ShadowEffect)]
pub struct Moveable;

pub struct SelectionPlugin;
impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                generic_click_system::<LogicButton>,
                generic_click_system::<Gate>,
                generic_click_system::<Link>,
            )
                .chain(),
        );
    }
}

#[derive(Component)]
pub struct Selected;

pub trait CustomCollider: Component {
    fn contains_point(&self, local_point: Vec2) -> bool;
}

#[derive(Default)]
pub struct ClickInteractionState {
    start_pos: Vec2,
    pending_selected_entity: Option<Entity>,
}

pub fn generic_click_system<T: CustomCollider>(
    query: Query<(Entity, &GlobalTransform, &T, Has<Selected>)>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    cursor_pos: Res<CursorPosition>,
    mut commands: Commands,
    selected_query: Query<Entity, (With<T>, With<Selected>)>,
    kb_input: Res<ButtonInput<KeyCode>>,
    mut click_state: Local<ClickInteractionState>,
) {
    let is_shift = kb_input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    let world_point = cursor_pos.in_world.extend(0.0);

    let drag_threshold = 2.0;

    if mouse_buttons.just_pressed(MouseButton::Left) {
        click_state.start_pos = cursor_pos.in_world;
        click_state.pending_selected_entity = None;

        let clicked_target = query.iter().find(|(_, transform, collider, _)| {
            let local_point = transform
                .compute_transform()
                .to_matrix()
                .inverse()
                .transform_point3(world_point)
                .truncate();

            collider.contains_point(local_point)
        });

        if let Some((entity, _, _, was_selected)) = clicked_target {
            if is_shift {
                if was_selected {
                    click_state.pending_selected_entity = Some(entity);
                } else {
                    commands.entity(entity).insert(Selected);
                }
            } else {
                if !was_selected {
                    for selected in selected_query.iter() {
                        commands.entity(selected).remove::<Selected>();
                    }
                    commands.entity(entity).insert(Selected);
                } else {
                    click_state.pending_selected_entity = Some(entity);
                    for selected in selected_query.iter() {
                        if selected != entity {
                            commands.entity(selected).remove::<Selected>();
                        }
                    }
                }
            }
        } else if !is_shift {
            for selected in selected_query.iter() {
                commands.entity(selected).remove::<Selected>();
            }
        }
    }

    if mouse_buttons.just_released(MouseButton::Left) {
        if let Some(pending_entity) = click_state.pending_selected_entity {
            let current_pos = cursor_pos.in_world;
            let distance = click_state.start_pos.distance(current_pos);

            if distance < drag_threshold {
                for selected in selected_query.iter() {
                    commands.entity(selected).remove::<Selected>();
                }
                commands.entity(pending_entity).remove::<Selected>();
            }
        }

        click_state.pending_selected_entity = None;
    }
}
