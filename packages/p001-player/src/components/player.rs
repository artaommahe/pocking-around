use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const PLAYER_SIZE: f32 = 50.0;
const PLAYER_SPEED: f32 = 500.0;

#[derive(Component)]
pub struct Player;

pub fn setup_player(
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
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        Player,
    ));
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut player_transform = query.single_mut();
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

    player_transform.translation.x = new_player_x;
    player_transform.translation.y = new_player_y;
}
