use bevy::input::mouse::AccumulatedMouseScroll;
use bevy::prelude::*;
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
        })
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_informations)
        .add_systems(Update, movement_camera)
        .add_systems(Update, update_information)
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
        "Cursor screen position : ({:.2}, {:.2}), world position : ({:.2}, {:.2})",
        cursor_position.in_screen.x,
        cursor_position.in_screen.y,
        cursor_position.in_world.x,
        cursor_position.in_world.y
    );
}

fn zoom_camera(
    q_camera: Single<(&mut Projection, &mut Transform), With<Camera>>,
    mouse_wheel_input: Res<AccumulatedMouseScroll>,
    mut camera_settings: ResMut<CameraSettings>,
    cursor: Res<CursorPosition>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let (mut projection, mut camera_transform) = q_camera.into_inner();

    let Projection::Orthographic(ref mut orthographic) = *projection else {
        return;
    };

    if mouse_wheel_input.delta.y == 0.0 {
        return;
    }

    let cursor_screen_pos = cursor.in_screen;

    let center = camera_transform.translation.truncate();
    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;
    let offset_x = cursor_screen_pos.x - half_width;
    let offset_y = half_height - cursor_screen_pos.y;

    let mouse_local_vector = Vec2::new(offset_x, offset_y) * orthographic.scale;
    let mouse_world_pos = center + mouse_local_vector;

    let old_scale = orthographic.scale;
    let delta_zoom = -camera_settings.zoom_scroll_speed
        * (mouse_wheel_input.delta.y / mouse_wheel_input.delta.y.abs());

    let new_scale = (old_scale * (1.0 + delta_zoom)).clamp(
        camera_settings.zoom_scroll_min,
        camera_settings.zoom_scroll_max,
    );

    orthographic.scale = new_scale;
    camera_settings.current_zoom = new_scale;

    let new_mouse_local_vector = mouse_local_vector * (new_scale / old_scale);
    let new_center = mouse_world_pos - new_mouse_local_vector;

    camera_transform.translation.x = new_center.x;
    camera_transform.translation.y = new_center.y;
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
