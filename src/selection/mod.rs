use crate::{
    cursor::CursorPosition,
    logic::{Gate, LogicButton},
};
use bevy::prelude::*;
mod button;
mod gate;

pub struct SelectionPlugin;
impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, generic_click_system::<Gate>)
            .add_systems(Update, generic_click_system::<LogicButton>);
    }
}

#[derive(Component)]
pub struct Selected;

pub trait CustomCollider: Component {
    fn contains_point(&self, local_point: Vec2) -> bool;
}

pub fn generic_click_system<T: CustomCollider>(
    query: Query<(Entity, &GlobalTransform, &T, Has<Selected>)>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    cursor_pos: Res<CursorPosition>,
    mut commands: Commands,
    selected_query: Query<Entity, With<Selected>>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    if !mouse_buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let is_shift = kb_input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    let world_point = cursor_pos.in_world.extend(0.0);

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
                commands.entity(entity).remove::<Selected>();
            } else {
                commands.entity(entity).insert(Selected);
            }
        } else {
            if !was_selected {
                for selected in selected_query.iter() {
                    commands.entity(selected).remove::<Selected>();
                }
            }
            commands.entity(entity).insert(Selected);
        }
    } else if !is_shift {
        for selected in selected_query.iter() {
            commands.entity(selected).remove::<Selected>();
        }
    }
}
