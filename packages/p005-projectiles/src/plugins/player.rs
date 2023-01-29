use std::f32::consts::PI;

use bevy::{
    prelude::*,
    render::camera::RenderTarget,
    sprite::{collide_aabb::Collision, MaterialMesh2dBundle},
};

use super::{
    camera::MainCamera,
    collider::{check_collision, Collider, ColliderTarget},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(PlayerPlugin::setup)
            .add_system(PlayerPlugin::move_player)
            .add_system(PlayerPlugin::follow_camera.after(PlayerPlugin::move_player))
            .add_system(PlayerPlugin::track_mouse_position);
    }
}

pub const PLAYER_SIZE: f32 = 10.0;
const PLAYER_COLOR: &str = "55cbcd";
const PLAYER_SPEED: f32 = 300.0;
const MOUSE_DIRECTION_DOT_SIZE: f32 = 3.0;
const MOUSE_DIRECTION_DOT_COLOR: &str = "d4f0f0";
// initial player direction is to the north, so we have to adjust any rotation by it
const MOUSE_DIRECTION_NORMALIZATION: f32 = PI / 2.0;

#[derive(Component)]
pub struct Player;

impl PlayerPlugin {
    fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        commands
            .spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(PLAYER_SIZE).into()).into(),
                    material: materials.add(ColorMaterial::from(
                        Color::hex(PLAYER_COLOR).expect("wrong player color"),
                    )),
                    transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                    ..default()
                },
                Player,
            ))
            .insert(Name::new("Player"))
            .with_children(|player| {
                player
                    .spawn(MaterialMesh2dBundle {
                        mesh: meshes
                            .add(shape::Circle::new(MOUSE_DIRECTION_DOT_SIZE).into())
                            .into(),
                        material: materials.add(ColorMaterial::from(
                            Color::hex(MOUSE_DIRECTION_DOT_COLOR)
                                .expect("wrong mouse direction dot color"),
                        )),
                        transform: Transform::from_translation(Vec3::new(0., 30., 1.)),
                        ..default()
                    })
                    .insert(Name::new("Mouse direction dot"));
            });
    }

    fn move_player(
        keyboard_input: Res<Input<KeyCode>>,
        time: Res<Time>,
        mut player: Query<&mut Transform, With<Player>>,
        obstacles: Query<(&Transform, &Collider), Without<Player>>,
    ) {
        if !keyboard_input.any_pressed([KeyCode::W, KeyCode::S, KeyCode::A, KeyCode::D]) {
            return;
        }

        let mut player_transform = player.single_mut();
        let mut new_player_x = player_transform.translation.x;
        let mut new_player_y = player_transform.translation.y;

        if keyboard_input.pressed(KeyCode::W) {
            new_player_y += PLAYER_SPEED * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::S) {
            new_player_y -= PLAYER_SPEED * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::A) {
            new_player_x -= PLAYER_SPEED * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::D) {
            new_player_x += PLAYER_SPEED * time.delta_seconds();
        }

        let collision = check_collision(
            ColliderTarget {
                position: Vec3::new(new_player_x, new_player_y, 0.),
                size: Vec2::splat(PLAYER_SIZE + PLAYER_SIZE),
            },
            &obstacles.into_iter().collect(),
        );

        if ![Some(Collision::Left), Some(Collision::Right)].contains(&collision) {
            player_transform.translation.x = new_player_x;
        }
        if ![Some(Collision::Top), Some(Collision::Bottom)].contains(&collision) {
            player_transform.translation.y = new_player_y
        }
    }

    fn follow_camera(
        player: Query<&Transform, With<Player>>,
        mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    ) {
        let player_transform = player.single();
        let mut camera_transform = camera.single_mut();

        camera_transform.translation.x = player_transform.translation.x;
        camera_transform.translation.y = player_transform.translation.y;
    }

    fn track_mouse_position(
        windows: Res<Windows>,
        camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
        mut player_query: Query<&mut Transform, With<Player>>,
    ) {
        let cursor_position = get_cursor_word_position(windows, camera_query);

        if cursor_position.is_none() {
            return;
        }

        let mut player_transform = player_query.single_mut();

        let player_position = player_transform.translation.truncate();
        let diff = cursor_position.unwrap() - player_position;
        let angle = diff.y.atan2(diff.x) - MOUSE_DIRECTION_NORMALIZATION;

        player_transform.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle);
    }
}

fn get_cursor_word_position(
    windows: Res<Windows>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) -> Option<Vec2> {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = camera_query.single();

    // get the window that the camera is displaying to (or the primary window)
    let window = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = window.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(window.width(), window.height());
        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        return Some(world_pos);
    }

    None
}
