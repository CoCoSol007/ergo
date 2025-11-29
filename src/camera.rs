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
        .add_systems(Startup, setup_grid_pool)
        .add_systems(Update, movement_camera)
        .add_systems(Update, update_information)
        .add_systems(Update, update_grid_dots)
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

// 1. Un composant pour identifier nos points (pour ne pas bouger le joueur ou les ennemis)
#[derive(Component)]
struct GridDot;

// 2. Configuration
const GRID_POOL_SIZE: usize = 1500; // Assez pour couvrir un écran 4K
const BASE_SPACING: f32 = 50.0; // Espace standard entre les points

// --- SYSTEM : SETUP (On crée les points une seule fois) ---
fn setup_grid_pool(mut commands: Commands) {
    // On spawn 1500 points cachés au début.
    // On utilise Sprite (plus léger que Mesh2d pour de simples carrés/points)
    for _ in 0..GRID_POOL_SIZE {
        commands.spawn((
            Sprite {
                color: Color::srgba(0.5, 0.5, 0.5, 0.5), // Gris semi-transparent
                custom_size: Some(Vec2::new(4.0, 4.0)),  // Taille du point
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, -10.0), // Z négatif pour être derrière
            Visibility::Hidden, // Caché par défaut tant que la caméra n'est pas là
            GridDot,
        ));
    }
}

// --- SYSTEM : UPDATE (La magie des Maths) ---

fn update_grid_dots(
    camera_q: Single<(&Transform, &Projection), With<Camera>>,
    window_q: Single<&Window, With<PrimaryWindow>>, // <--- On récupère la fenêtre
    mut dots_q: Query<
        (&mut Transform, &mut Visibility, &mut Sprite),
        (With<GridDot>, Without<Camera>),
    >,
) {
    let (cam_transform, projection) = camera_q.into_inner();
    let window = window_q.into_inner();

    // 1. Zoom
    let zoom = if let Projection::Orthographic(ortho) = projection {
        ortho.scale
    } else {
        1.0
    };

    // 2. Calculer la taille visible du monde (World Space)
    // On convertit la taille de la fenêtre (pixels) en taille monde (coordonnées)
    // On ajoute un petit buffer (+ 100.0) pour ne pas voir les points apparaître brutalement
    let visible_width = window.width() * zoom + 200.0;
    let visible_height = window.height() * zoom + 200.0;

    // 3. Logique d'espacement (LOD)
    // spacing = distance entre deux points dans le monde.
    // Plus on dézoome (zoom augmente), plus les points s'écartent.

    // Taille des points (ils grossissent un peu avec le zoom pour rester visibles)
    let step = 2.0_f32.powf(zoom.log2().floor());

    // On s'assure de ne jamais descendre en dessous de 1.0 pour éviter les bugs au zoom max
    let spacing_multiplier = step.max(1.0);

    let spacing = BASE_SPACING * spacing_multiplier;

    // Pour la taille des points : on veut qu'ils aient toujours la même taille EN PIXELS (écran).
    // Donc on multiplie leur taille Monde par le zoom.
    let dot_size = 4.0 * zoom;

    // 4. Calculer les bornes (Gauche, Droite, Haut, Bas)
    let cam_x = cam_transform.translation.x;
    let cam_y = cam_transform.translation.y;

    let left_bound = cam_x - (visible_width / 2.0);
    let bottom_bound = cam_y - (visible_height / 2.0);

    let right_bound = cam_x + (visible_width / 2.0);
    let top_bound = cam_y + (visible_height / 2.0);

    // 5. Trouver le premier point de la grille (Snapping Mathématique)
    // C'est ça qui empêche l'effet de "glissement"
    let start_x = (left_bound / spacing).floor() * spacing;
    let start_y = (bottom_bound / spacing).floor() * spacing;

    // 6. Boucle de placement
    let mut dot_iter = dots_q.iter_mut();

    let mut current_x = start_x;
    while current_x <= right_bound {
        let mut current_y = start_y;
        while current_y <= top_bound {
            if let Some((mut transform, mut visibility, mut sprite)) = dot_iter.next() {
                // Positionnement
                transform.translation.x = current_x;
                transform.translation.y = current_y;
                // On garde le Z bien au fond
                transform.translation.z = -10.0;

                // Mise à jour visuelle
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
