use bevy::{prelude::*, window::PrimaryWindow};

const BASE_SPACING: f32 = 40.0;

pub struct GridPlugin;
impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, manage_grid_pool_size)
            .add_systems(Update, update_grid_sprites);
    }
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
