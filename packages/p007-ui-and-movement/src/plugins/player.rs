use std::f32::consts::PI;

use bevy::{
    prelude::*,
    render::camera::RenderTarget,
    sprite::{
        collide_aabb::{collide, Collision},
        MaterialMesh2dBundle,
    },
};

use super::{
    camera::MainCamera,
    collider::{Collider, ColliderTarget},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Cursor {
            angle: Quat::from_axis_angle(Vec3::Z, 0.),
        })
        .add_startup_system(PlayerPlugin::setup)
        .add_system(PlayerPlugin::track_cursor_position)
        .add_system(PlayerPlugin::rotate_player.after(PlayerPlugin::track_cursor_position))
        .add_system(PlayerPlugin::move_player.after(PlayerPlugin::rotate_player))
        .add_system(PlayerPlugin::follow_camera.after(PlayerPlugin::move_player));
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

        let x_direction: f32 = if keyboard_input.pressed(KeyCode::D) {
            1.
        } else if keyboard_input.pressed(KeyCode::A) {
            -1.
        } else {
            0.
        };

        let y_direction: f32 = if keyboard_input.pressed(KeyCode::W) {
            1.
        } else if keyboard_input.pressed(KeyCode::S) {
            -1.
        } else {
            0.
        };

        let slow_move_factor = if keyboard_input.any_pressed([KeyCode::LShift, KeyCode::RShift]) {
            0.5
        } else {
            1.
        };

        let movement_angle = y_direction.atan2(x_direction) - MOUSE_DIRECTION_NORMALIZATION;
        let movement_rotation = Quat::from_axis_angle(Vec3::Z, movement_angle);
        let angle_diff = player_transform.rotation.angle_between(movement_rotation);
        let rotation_slowdown_factor = 1. - (angle_diff / PI) * 0.8;

        let movement_speed = PLAYER_SPEED * rotation_slowdown_factor * slow_move_factor;

        let new_player_x =
            player_transform.translation.x + x_direction * movement_speed * time.delta_seconds();
        let new_player_y =
            player_transform.translation.y + y_direction * movement_speed * time.delta_seconds();

        let collision = check_collision(new_player_x, new_player_y, &obstacles);

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

    fn track_cursor_position(
        windows: Res<Windows>,
        camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
        player_query: Query<&Transform, With<Player>>,
        mut cursor: ResMut<Cursor>,
    ) {
        let cursor_position = get_cursor_word_position(&windows, &camera_query);

        if cursor_position.is_none() {
            return;
        }

        let player_transform = player_query.single();

        let player_position = player_transform.translation.truncate();
        let diff = cursor_position.unwrap() - player_position;
        let angle = diff.y.atan2(diff.x) - MOUSE_DIRECTION_NORMALIZATION;

        cursor.angle = Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle);
    }

    fn rotate_player(mut player_query: Query<&mut Transform, With<Player>>, cursor: Res<Cursor>) {
        let mut player_transform = player_query.single_mut();

        player_transform.rotation = cursor.angle;
    }
}

#[derive(Resource)]
struct Cursor {
    angle: Quat,
}

fn get_cursor_word_position(
    windows: &Res<Windows>,
    camera_query: &Query<(&Camera, &GlobalTransform), With<MainCamera>>,
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

fn check_collision(
    new_player_x: f32,
    new_player_y: f32,
    obstacles: &Query<(&Transform, &Collider), Without<Player>>,
) -> Option<Collision> {
    let target = ColliderTarget {
        position: Vec3::new(new_player_x, new_player_y, 0.),
        size: Vec2::splat(PLAYER_SIZE * 2.),
    };

    for (obstacle_transform, obstacle_collider) in obstacles.into_iter() {
        let obstacle_collision = collide(
            target.position,
            target.size,
            obstacle_transform.translation,
            obstacle_collider.size,
        );

        if obstacle_collision.is_some() {
            return obstacle_collision;
        }
    }

    None
}
