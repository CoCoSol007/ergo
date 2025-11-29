use bevy::input::mouse::AccumulatedMouseScroll;
use bevy::prelude::*;
use bevy::window::{CursorIcon, PrimaryWindow, SystemCursorIcon};

use crate::cursor::CursorPosition;

const ZOOM_SCROLL_SPEED: f32 = 0.1;
const ZOOM_SCROLL_MAX: f32 = 1.;
const ZOOM_SCROLL_MIN: f32 = 0.01;
const DEFAULT_ZOOM: f32 = 0.1;
const BASE_SPACING: f32 = 40.0;

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
        })
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_informations)
        .add_systems(Update, manage_grid_pool_size)
        .add_systems(Update, movement_camera)
        .add_systems(Update, update_information)
        .add_systems(Update, update_grid_sprites)
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
}

#[derive(Component)]
struct CameraInformation;

#[derive(Component)]
struct CursorInformation;

/// Sets up the 2D camera with an orthographic projection.
fn setup_camera(
    mut commands: Commands,
    camera_settings: Res<CameraSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scale: camera_settings.current_zoom,
            ..OrthographicProjection::default_2d()
        }),
    ));

    commands.spawn((
        Mesh2d(meshes.add(Mesh::from(Circle {
            radius: 100.0,
            ..Default::default()
        }))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::WHITE))),
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
    camera_query: Query<(&Transform, &Projection), With<Camera>>,
    mut camera_info_query: Query<&mut Text, (With<CameraInformation>, Without<CursorInformation>)>,
    mut cursor_info_query: Query<&mut Text, (With<CursorInformation>, Without<CameraInformation>)>,
    cursor_position: Res<CursorPosition>,
) {
    let (camera_transform, camera_projection) = camera_query.single().unwrap();
    let mut info_text = camera_info_query.single_mut().unwrap();

    let Projection::Orthographic(orthographic_projection) = camera_projection else {
        return;
    };

    info_text.0 = format!(
        "Camera position : ({:.2}, {:.2}), scale: {:.2}",
        camera_transform.translation.x,
        camera_transform.translation.y,
        orthographic_projection.scale
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

    if mouse_wheel_input.delta.y == 0.0 {
        return;
    }

    let delta_zoom = -camera_settings.zoom_scroll_speed
        * (mouse_wheel_input.delta.y / mouse_wheel_input.delta.y.abs());

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

    camera.translation = camera.translation
        + Vec3 {
            x: delta.x,
            y: -delta.y,
            z: 0.0,
        } * camera_settings.current_zoom;
}

#[derive(Component)]
struct GridDot;

fn manage_grid_pool_size(
    mut commands: Commands,
    window_q: Single<&Window, With<PrimaryWindow>>,
    dots_q: Query<&GridDot>,
) {
    let window = window_q.into_inner();

    let cols_needed = (window.width() / BASE_SPACING) * 2.0;
    let rows_needed = (window.height() / BASE_SPACING) * 2.0;

    let safe_count = ((cols_needed * rows_needed) * 1.1) as usize;

    let current_count = dots_q.iter().len();

    if current_count < safe_count {
        let missing = safe_count - current_count;

        let dot_color = Color::srgba(0.5, 0.5, 0.5, 0.4);

        for _ in 0..missing {
            commands.spawn((
                Sprite {
                    color: dot_color,
                    custom_size: Some(Vec2::splat(4.0)),
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, -100.0),
                Visibility::Hidden,
                GridDot,
            ));
        }
    }
}

fn update_grid_sprites(
    camera_q: Single<(&Transform, &Projection), With<Camera>>,
    window_q: Single<&Window, With<PrimaryWindow>>,
    mut dots_q: Query<
        (&mut Transform, &mut Visibility, &mut Sprite),
        (With<GridDot>, Without<Camera>),
    >,
) {
    let (cam_transform, projection) = camera_q.into_inner();
    let window = window_q.into_inner();

    let zoom = if let Projection::Orthographic(ortho) = projection {
        ortho.scale
    } else {
        1.0
    };

    let step = 2.0_f32.powf(zoom.log2().floor());
    let spacing = BASE_SPACING * step.max(1.0);
    let dot_size = 3.0 * zoom;

    let margin = spacing * 2.0;
    let visible_width = window.width() * zoom + margin;
    let visible_height = window.height() * zoom + margin;

    let cam_x = cam_transform.translation.x;
    let cam_y = cam_transform.translation.y;

    let left = cam_x - visible_width / 2.0;
    let right = cam_x + visible_width / 2.0;
    let bottom = cam_y - visible_height / 2.0;
    let top = cam_y + visible_height / 2.0;

    let start_x = (left / spacing).floor() * spacing;
    let start_y = (bottom / spacing).floor() * spacing;

    let mut dot_iter = dots_q.iter_mut();

    let mut current_x = start_x;
    while current_x <= right {
        let mut current_y = start_y;
        while current_y <= top {
            if let Some((mut transform, mut visibility, mut sprite)) = dot_iter.next() {
                transform.translation.x = current_x;
                transform.translation.y = current_y;
                transform.translation.z = -100.0;
                sprite.custom_size = Some(Vec2::splat(dot_size));
                *visibility = Visibility::Visible;
            } else {
                break;
            }
            current_y += spacing;
        }
        current_x += spacing;
    }

    for (_, mut visibility, _) in dot_iter {
        *visibility = Visibility::Hidden;
    }
}
