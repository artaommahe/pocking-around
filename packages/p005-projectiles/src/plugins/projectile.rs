use bevy::prelude::*;

use super::player::Player;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(WeaponPlugin::fire_bullet);
    }
}

impl WeaponPlugin {
    fn fire_bullet(
        mut commands: Commands,
        mouse_input: Res<Input<MouseButton>>,
        keyboard_input: Res<Input<KeyCode>>,
        player_query: Query<(&Transform), With<Player>>,
    ) {
        if mouse_input.just_pressed(MouseButton::Left)
            || keyboard_input.just_pressed(KeyCode::Space)
        {
            let (player_transform) = player_query.single();

            println!("{:?}", player_transform);
            commands.spawn(Bullet::from_player_position(player_transform));
        }
    }
}

#[derive(Component)]
struct Projectile {
    speed: f32,
}

const BULLET_SIZE: Vec2 = Vec2::new(2., 10.);
const BULLET_COLOR: &str = "ffc48c";

#[derive(Bundle)]
struct Bullet {
    sprite: SpriteBundle,
    projectile: Projectile,
}

impl Bullet {
    fn from_player_position(player_transform: &Transform) -> Self {
        let position_correction = player_transform.rotation.mul_vec3(Vec3::Y * 20.);

        let transform = Transform {
            translation: player_transform.translation.clone() + position_correction,
            scale: BULLET_SIZE.extend(1.0),
            rotation: player_transform.rotation.clone(),
            ..default()
        };

        Bullet {
            sprite: SpriteBundle {
                transform,
                sprite: Sprite {
                    color: Color::hex(BULLET_COLOR).expect("wrong bullet color"),
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    ..default()
                },
                ..default()
            },
            projectile: Projectile { speed: 50. },
        }
    }
}
