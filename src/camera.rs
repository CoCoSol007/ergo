use bevy::input::mouse::AccumulatedMouseScroll;
use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;
use bevy::sprite_render::{Material2d, Material2dPlugin};
use bevy::window::{CursorIcon, PrimaryWindow, SystemCursorIcon};

use crate::cursor::CursorPosition;

const ZOOM_SCROLL_SPEED: f32 = 0.1;
const ZOOM_SCROLL_MAX: f32 = 1.;
const ZOOM_SCROLL_MIN: f32 = 0.01;
const DEFAULT_ZOOM: f32 = 0.1;

/// Manages the camera behavior, including zooming and movement.
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraSettings {
            zoom_scroll_speed: ZOOM_SCROLL_SPEED,
            zoom_scroll_max: ZOOM_SCROLL_MAX,
            zoom_scroll_min: ZOOM_SCROLL_MIN,
            current_zoom: DEFAULT_ZOOM,
            information: true,
            grid: true,
        })
        .add_systems(Startup, setup_camera)
        .add_systems(Update, movement_camera)
        .add_systems(Update, update_information)
        .add_systems(Startup, setup_informations)
        .add_systems(Startup, setup_grid)
        .add_plugins(Material2dPlugin::<GridMaterial>::default())
        .add_systems(Update, zoom_camera);
    }
}

/// Represents the settings for the camera.
#[derive(Resource)]
struct CameraSettings {
    zoom_scroll_speed: f32,
    zoom_scroll_max: f32,
    zoom_scroll_min: f32,
    current_zoom: f32,
    information: bool,
    grid: bool,
}

#[derive(Component)]
struct CameraInformation;

#[derive(Component)]
struct CursorInformation;

/// Sets up the 2D camera with an orthographic projection.
fn setup_camera(mut commands: Commands, camera_settings: Res<CameraSettings>) {
    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scale: camera_settings.current_zoom,
            ..OrthographicProjection::default_2d()
        }),
    ));
}

fn setup_informations(mut commands: Commands, camera_settings: Res<CameraSettings>) {
    if camera_settings.information {
        commands.spawn((
            CameraInformation,
            Text::new("Camera position : (0, 0)"),
            Node {
                left: Val::Px(10.0),
                top: Val::Px(10.0),

                ..default()
            },
        ));
        commands.spawn((
            CursorInformation,
            Node {
                top: Val::Px(40.0),
                left: Val::Px(10.0),
                ..default()
            },
            Text::new("Cursor position : (0, 0)"),
        ));
    }
}

/// Updates the camera / cursor position information displayed on the screen.
fn update_information(
    camera_query: Query<&Transform, With<Camera>>,
    mut camera_info_query: Query<&mut Text, (With<CameraInformation>, Without<CursorInformation>)>,
    mut cursor_info_query: Query<&mut Text, (With<CursorInformation>, Without<CameraInformation>)>,
    cursor_position: Res<CursorPosition>,
) {
    let camera_transform = camera_query.single().unwrap();
    let mut info_text = camera_info_query.single_mut().unwrap();

    info_text.0 = format!(
        "Camera position : ({:.2}, {:.2})",
        camera_transform.translation.x, camera_transform.translation.y
    );

    let mut info_text = cursor_info_query.single_mut().unwrap();

    info_text.0 = format!(
        "Cursor position : ({:.2}, {:.2})",
        cursor_position.0.x, cursor_position.0.y
    );
}

/// Zooms the camera based on mouse wheel input.
fn zoom_camera(
    camera: Single<&mut Projection, With<Camera>>,
    mouse_wheel_input: Res<AccumulatedMouseScroll>,
    mut camera_settings: ResMut<CameraSettings>,
) {
    let mut projection = camera.into_inner();
    let Projection::Orthographic(ref mut orthographic) = *projection else {
        return;
    };

    let delta_zoom = -mouse_wheel_input.delta.y * camera_settings.zoom_scroll_speed;

    orthographic.scale = (orthographic.scale * (1.0 + delta_zoom)).clamp(
        camera_settings.zoom_scroll_min,
        camera_settings.zoom_scroll_max,
    );

    camera_settings.current_zoom = orthographic.scale;
}

/// Moves the camera based on mouse drag input.
fn movement_camera(
    mut camera: Single<&mut Transform, With<Camera>>,
    mut pick_position: Local<Option<Vec2>>,
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<(&Window, Entity), With<PrimaryWindow>>,
    mut commands: Commands,
    camera_settings: Res<CameraSettings>,
    mut grid_q: Query<&mut Transform, (With<Grid>, Without<Camera>)>,
) {
    if buttons.just_pressed(MouseButton::Right) {
        let Some(position) = window.0.cursor_position() else {
            return;
        };
        commands
            .entity(window.1)
            .insert(CursorIcon::System(SystemCursorIcon::Grabbing));
        *pick_position = Some(position)
    }

    if buttons.just_released(MouseButton::Right) {
        commands
            .entity(window.1)
            .insert(CursorIcon::System(SystemCursorIcon::Grab));

        *pick_position = None;
        return;
    }

    let (Some(start_position), Some(position)) = (*pick_position, window.0.cursor_position())
    else {
        return;
    };

    let delta = start_position - position;
    *pick_position = Some(position);

    let mut grid_transform = grid_q.single_mut().unwrap();

    camera.translation = camera.translation
        + Vec3 {
            x: delta.x,
            y: -delta.y,
            z: 0.0,
        } * camera_settings.current_zoom;

    grid_transform.translation.x = camera.translation.x;
    grid_transform.translation.y = camera.translation.y;
}

/// Sets up the infinite grid material and mesh.
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct GridMaterial {
    #[uniform(0)]
    color: LinearRgba,
}

impl Material2d for GridMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/grid.wgsl".into()
    }
}

/// Sets up the grid entity in the scene.
fn setup_grid(
    mut commands: Commands,
    camera_settings: Res<CameraSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<GridMaterial>>,
) {
    if camera_settings.grid {
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::default()).into()),
            Transform::from_xyz(0.0, 0.0, -1.0).with_scale(Vec3::splat(5000.0)),
            MeshMaterial2d(materials.add(GridMaterial {
                color: LinearRgba::new(0.2, 0.2, 0.2, 1.0),
            })),
            Grid,
        ));
    }
}

/// Component to identify the grid entity.
#[derive(Component)]
struct Grid;
