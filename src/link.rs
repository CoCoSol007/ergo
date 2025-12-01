use bevy::{platform::collections::HashSet, prelude::*};

use crate::logic::{Gate, Item};

pub struct LinkPlugin;
impl Plugin for LinkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (link_system, suppr_links));
    }
}

#[derive(Component)]
pub struct Link {
    pub from: Entity,
    pub to: Entity,
    pub from_position: Vec2,
    pub to_position: Vec2,
}

fn link_system(
    mut query: Query<(&Gate, Entity), Changed<Gate>>,
    mut commands: Commands,
    mut local_links: Local<HashSet<(Entity, Entity)>>,
) {
    for (gate, entity) in query.iter_mut() {
        let links = match gate {
            Gate::And(a, b) => vec![a, b],
            Gate::Or(a, b) => vec![a, b],
            Gate::Not(a) => vec![a],
        };

        for link in links {
            if let Some(target_entity) = link {
                let link_key = (entity, *target_entity);
                if !local_links.contains(&link_key) {
                    local_links.insert(link_key);

                    commands.spawn(Link {
                        from: entity,
                        to: *target_entity,
                        from_position: Vec2::ZERO,
                        to_position: Vec2::ZERO,
                    });
                }
            }
        }
    }
}

fn suppr_links(
    mut commands: Commands,
    mut removed_items: RemovedComponents<Item>,
    link_query: Query<(Entity, &Link)>,
) {
    let removed_set: std::collections::HashSet<Entity> = removed_items.read().collect();

    for (link_entity, link) in link_query.iter() {
        if removed_set.contains(&link.from) || removed_set.contains(&link.to) {
            commands.entity(link_entity).despawn();
        }
    }
}
