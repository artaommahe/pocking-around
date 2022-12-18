use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use super::collider::{check_collision, Collider, ColliderTarget};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_player)
            .add_system(move_player)
            .add_system(follow_camera.after(move_player));
    }
}

const PLAYER_SIZE: f32 = 10.0;
const PLAYER_SPEED: f32 = 500.0;

#[derive(Component)]
pub struct Player;

fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(PLAYER_SIZE).into()).into(),
            material: materials.add(ColorMaterial::from(
                Color::hex("55cbcd").expect("wrong player color"),
            )),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..default()
        },
        Player,
    ));
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player: Query<&mut Transform, With<Player>>,
    obstacles: Query<(&Transform, &Collider), Without<Player>>,
) {
    let mut player_transform = player.single_mut();
    let mut new_player_x = player_transform.translation.x;
    let mut new_player_y = player_transform.translation.y;

    if keyboard_input.pressed(KeyCode::Up) {
        new_player_y += PLAYER_SPEED * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::Down) {
        new_player_y -= PLAYER_SPEED * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::Left) {
        new_player_x -= PLAYER_SPEED * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::Right) {
        new_player_x += PLAYER_SPEED * time.delta_seconds();
    }

    if !check_collision(
        ColliderTarget {
            position: Vec3::new(new_player_x, 0., 0.),
            size: Vec2::splat(PLAYER_SIZE + PLAYER_SIZE),
        },
        &obstacles,
    ) {
        player_transform.translation.x = new_player_x;
    }

    if !check_collision(
        ColliderTarget {
            position: Vec3::new(0., new_player_y, 0.),
            size: Vec2::splat(PLAYER_SIZE + PLAYER_SIZE),
        },
        &obstacles,
    ) {
        player_transform.translation.y = new_player_y;
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
