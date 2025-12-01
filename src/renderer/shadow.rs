use bevy::prelude::*;

use crate::selection::Selected;

const SHADOW_OFFSET_Y: f32 = -5.0;

#[derive(Component, Default)]
pub struct ShadowEffect;

pub struct ShadowRendererPlugin;
impl Plugin for ShadowRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (display_shadows, display_selection_without_shadow));
    }
}

#[derive(Component)]
pub struct ShadowEntity;

pub fn display_shadows(
    mut commands: Commands,
    mut added_selection: Query<
        (Entity, &Mesh2d, &mut Transform),
        (With<ShadowEffect>, Added<Selected>),
    >,
    mut unselected_transforms: Query<&mut Transform, (Without<Selected>, With<ShadowEffect>)>,
    mut removed_selection: RemovedComponents<Selected>,
    children_query: Query<&Children>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    shadow_query: Query<Entity, With<ShadowEntity>>,
) {
    let lift_vector = Vec3::new(0.0, -SHADOW_OFFSET_Y, 1.0);

    for (entity, mesh, mut transform) in added_selection.iter_mut() {
        transform.translation += lift_vector;

        let shadow_material = materials.add(ColorMaterial {
            color: Color::linear_rgba(0., 0., 0., 0.75),
            ..default()
        });

        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                mesh.clone(),
                MeshMaterial2d(shadow_material),
                Transform::from_translation(Vec3::new(0.0, SHADOW_OFFSET_Y, -0.1)),
                ShadowEntity,
            ));
        });
    }

    for entity in removed_selection.read() {
        if let Ok(mut transform) = unselected_transforms.get_mut(entity) {
            transform.translation -= lift_vector;
        }

        if let Ok(children) = children_query.get(entity) {
            for &child in children.into_iter().filter(|c| shadow_query.contains(**c)) {
                commands.entity(child).despawn();
            }
        }
    }
}

pub fn display_selection_without_shadow(
    mut added_selection: Query<
        &mut MeshMaterial2d<ColorMaterial>,
        (Without<ShadowEffect>, Added<Selected>),
    >,
    mut unselected_transforms: Query<
        &mut MeshMaterial2d<ColorMaterial>,
        (Without<Selected>, Without<ShadowEffect>),
    >,
    mut removed_selection: RemovedComponents<Selected>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for mut material in added_selection.iter_mut() {
        *material = MeshMaterial2d(materials.add(ColorMaterial {
            color: Color::linear_rgba(0.3, 0.3, 0.3, 1.),
            ..default()
        }));
    }

    for entity in removed_selection.read() {
        if let Ok(mut material) = unselected_transforms.get_mut(entity) {
            *material = MeshMaterial2d(materials.add(ColorMaterial {
                color: Color::linear_rgba(1., 1.0, 1.0, 1.),
                ..default()
            }));
        }
    }
}
