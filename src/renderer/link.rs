use bevy::prelude::*;

use crate::{link::Link, logic::Item};

pub struct RendererLinkPlugin;
impl Plugin for RendererLinkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_links, setup_links));
    }
}

fn setup_links(
    query: Query<(Entity, &Link), Added<Link>>,
    mut commands: Commands,
    item_query: Query<(&Transform, Entity)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, link) in query.iter() {
        let Ok((from_transform, _)) = item_query.get(link.from) else {
            continue;
        };
        let Ok((to_transform, _)) = item_query.get(link.to) else {
            continue;
        };

        commands.entity(entity).insert((
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::WHITE))),
            Mesh2d(meshes.add(Mesh::from(Segment2d::new(
                from_transform.translation.truncate(),
                to_transform.translation.truncate(),
            )))),
            Transform::from_xyz(0.0, 0.0, -2.),
            Link {
                from: link.from,
                to: link.to,
                from_position: from_transform.translation.truncate(),
                to_position: to_transform.translation.truncate(),
            },
        ));
    }
}

fn update_links(
    mut links: Query<(&mut Mesh2d, &mut Link)>,
    changed_item_query: Query<(&Transform, Entity), (With<Item>, Changed<Transform>)>,
    all_item_query: Query<(&Transform, Entity), With<Item>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (mut mesh, mut link) in links.iter_mut() {
        let from_transform = match changed_item_query.get(link.from) {
            Ok(v) => v.0,
            Err(_) => match all_item_query.get(link.from) {
                Ok(v) => v.0,
                Err(_) => continue,
            },
        };

        let to_transform = match changed_item_query.get(link.to) {
            Ok(v) => v.0,
            Err(_) => match all_item_query.get(link.to) {
                Ok(v) => v.0,
                Err(_) => continue,
            },
        };

        mesh.0 = meshes.add(Mesh::from(Segment2d::new(
            from_transform.translation.truncate(),
            to_transform.translation.truncate(),
        )));

        link.from_position = from_transform.translation.truncate();
        link.to_position = to_transform.translation.truncate();
    }
}
