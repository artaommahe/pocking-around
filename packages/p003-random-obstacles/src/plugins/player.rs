use bevy::{
    prelude::*,
    sprite::{collide_aabb::Collision, MaterialMesh2dBundle},
};

use super::collider::{check_collision, Collider, ColliderTarget};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(PlayerPlugin::setup)
            .add_system(PlayerPlugin::move_player)
            .add_system(PlayerPlugin::follow_camera.after(PlayerPlugin::move_player));
    }
}

pub const PLAYER_SIZE: f32 = 10.0;
const PLAYER_COLOR: &str = "55cbcd";
const PLAYER_SPEED: f32 = 500.0;

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
            .insert(Name::new("Player"));
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
            &obstacles,
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
}
