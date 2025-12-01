use crate::logic::*;

use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

pub struct GateRendererPlugin;
impl Plugin for GateRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (display_gates, update_gate_colors, display_buttons));
    }
}

impl Gate {
    pub fn mesh2d(&self) -> Mesh {
        let resolution = 32;

        match self {
            Gate::And(_, _) => create_and_mesh(50.0, 40.0, resolution),
            Gate::Or(_, _) => create_or_mesh(50.0, 40.0, resolution),
            Gate::Not(_) => create_not_mesh(40.0, 30.0, resolution),
        }
    }
}

fn create_and_mesh(width: f32, height: f32, resolution: u32) -> Mesh {
    let mut positions = Vec::new();
    let mut indices = Vec::new();

    positions.push([-width / 2.0, height / 2.0, 0.0]);
    positions.push([-width / 2.0, -height / 2.0, 0.0]);
    positions.push([0.0, -height / 2.0, 0.0]);
    positions.push([0.0, height / 2.0, 0.0]);

    indices.extend_from_slice(&[0, 1, 2, 0, 2, 3]);

    let center_idx = positions.len() as u32;
    positions.push([0.0, 0.0, 0.0]);

    let radius = height / 2.0;
    let start_angle = -std::f32::consts::FRAC_PI_2;
    let end_angle = std::f32::consts::FRAC_PI_2;

    let first_arc_idx = positions.len() as u32;
    for i in 0..=resolution {
        let t = i as f32 / resolution as f32;
        let angle = start_angle + t * (end_angle - start_angle);

        positions.push([radius * angle.cos(), radius * angle.sin(), 0.0]);

        if i > 0 {
            let current = first_arc_idx + i;
            indices.extend_from_slice(&[center_idx, current - 1, current]);
        }
    }

    generate_mesh(positions, indices)
}

fn create_or_mesh(width: f32, height: f32, resolution: u32) -> Mesh {
    let mut positions = Vec::new();
    let mut indices = Vec::new();

    let half_h = height / 2.0;
    let left_x = -width / 2.0;
    let tip_x = width / 2.0;

    let curve_indent = width * 0.15;

    positions.push([0.0, 0.0, 0.0]);
    let center_idx = 0;

    let p0_back = Vec2::new(left_x, half_h);
    let p2_back = Vec2::new(left_x, -half_h);
    let ctrl_back = Vec2::new(left_x + curve_indent, 0.0);

    for i in 0..=resolution {
        let t = i as f32 / resolution as f32;
        let p = quadratic_bezier(p0_back, ctrl_back, p2_back, t);
        positions.push([p.x, p.y, 0.0]);
    }

    let p0_bot = p2_back;
    let p2_bot = Vec2::new(tip_x, 0.0);
    let ctrl_bot = Vec2::new(left_x + (width * 0.75), -half_h);

    for i in 1..=resolution {
        let t = i as f32 / resolution as f32;
        let p = quadratic_bezier(p0_bot, ctrl_bot, p2_bot, t);
        positions.push([p.x, p.y, 0.0]);
    }

    let p0_top = p2_bot;
    let p2_top = p0_back;
    let ctrl_top = Vec2::new(left_x + (width * 0.75), half_h);

    for i in 1..resolution {
        let t = i as f32 / resolution as f32;
        let p = quadratic_bezier(p0_top, ctrl_top, p2_top, t);
        positions.push([p.x, p.y, 0.0]);
    }

    let total_points = positions.len() as u32;
    for i in 1..total_points - 1 {
        indices.extend_from_slice(&[center_idx, i, i + 1]);
    }
    indices.extend_from_slice(&[center_idx, total_points - 1, 1]);

    generate_mesh(positions, indices)
}

fn quadratic_bezier(p0: Vec2, p1: Vec2, p2: Vec2, t: f32) -> Vec2 {
    let one_minus_t = 1.0 - t;
    p0 * one_minus_t.powi(2) + p1 * 2.0 * one_minus_t * t + p2 * t.powi(2)
}

fn create_not_mesh(width: f32, height: f32, resolution: u32) -> Mesh {
    let mut positions = Vec::new();
    let mut indices = Vec::new();

    let bubble_radius = 6.0;
    let triangle_w = width - (bubble_radius * 2.0);
    let left_x = -width / 2.0;
    let tip_x = left_x + triangle_w;

    positions.push([left_x, height / 2.0, 0.0]);
    positions.push([left_x, -height / 2.0, 0.0]);
    positions.push([tip_x, 0.0, 0.0]);
    indices.extend_from_slice(&[0, 1, 2]);

    let center_x = tip_x + bubble_radius - 2.0;
    let center_idx = positions.len() as u32;
    positions.push([center_x, 0.0, 0.0]);

    let first_circle_idx = positions.len() as u32;
    for i in 0..=resolution {
        let t = i as f32 / resolution as f32;
        let angle = t * std::f32::consts::TAU;

        positions.push([
            center_x + bubble_radius * angle.cos(),
            bubble_radius * angle.sin(),
            0.0,
        ]);

        if i > 0 {
            let current = first_circle_idx + i;
            indices.extend_from_slice(&[center_idx, current - 1, current]);
        }
    }

    generate_mesh(positions, indices)
}

fn generate_mesh(positions: Vec<[f32; 3]>, indices: Vec<u32>) -> Mesh {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

pub fn display_gates(
    new_gates: Query<(Entity, &Gate), Added<Gate>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, gate) in new_gates.iter() {
        let mesh = gate.mesh2d();

        commands.entity(entity).insert((
            Mesh2d(meshes.add(mesh)),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.5, 0.5, 0.5)))),
        ));
    }
}

pub fn update_gate_colors(
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(&Value, &MeshMaterial2d<ColorMaterial>), Changed<Value>>,
    mut removed_values: RemovedComponents<Value>,
) {
    for (value, mat_handle) in query.iter() {
        if let Some(material) = materials.get_mut(mat_handle) {
            material.color = if value.state {
                Color::srgb(0.0, 1.0, 0.0)
            } else {
                Color::srgb(1.0, 0.0, 0.0)
            };
        }
    }

    for entity in removed_values.read() {
        if let Ok((_, mat_handle)) = query.get(entity) {
            if let Some(material) = materials.get_mut(mat_handle) {
                material.color = Color::srgb(0.5, 0.5, 0.5);
            }
        }
    }
}

pub fn display_buttons(
    new_buttons: Query<(Entity, &LogicButton), Added<LogicButton>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, _) in new_buttons.iter() {
        commands.entity(entity).insert((
            Mesh2d(meshes.add(Mesh::from(Circle {
                radius: 10.0,
                ..default()
            }))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(1.0, 0., 0.)))),
        ));
    }
}
