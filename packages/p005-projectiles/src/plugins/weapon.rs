use std::time::Duration;

use bevy::prelude::*;

use super::{player::Player, projectile::Projectile};

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LastFiredProjectile {
            time: Duration::ZERO,
        })
        .add_system(WeaponPlugin::fire_bullet);
    }
}

impl WeaponPlugin {
    fn fire_bullet(
        mut commands: Commands,
        mouse_input: Res<Input<MouseButton>>,
        keyboard_input: Res<Input<KeyCode>>,
        player_query: Query<&Transform, With<Player>>,
        time: Res<Time>,
        mut last_fired_projectile: ResMut<LastFiredProjectile>,
    ) {
        if (mouse_input.pressed(MouseButton::Left) || keyboard_input.pressed(KeyCode::Space))
            && (time.elapsed() - last_fired_projectile.time).as_millis() > PROJECTILES_THROTTLE
        {
            let player_transform = player_query.single();

            commands
                .spawn(Bullet::from_player_position(player_transform))
                .insert(Name::new("Bullet"));

            last_fired_projectile.time = time.elapsed();
        }
    }
}

const PROJECTILES_THROTTLE: u128 = 200;

#[derive(Resource, Debug)]
struct LastFiredProjectile {
    // TODO: rewrite with Some(Timer)
    time: Duration,
}

const BULLET_SIZE: Vec2 = Vec2::new(2., 15.);
const BULLET_COLOR: &str = "ffc48c";
const BULLET_SPEED: f32 = 750.;
const BULLET_DAMAGE: f32 = 20.;

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
            projectile: Projectile {
                speed: BULLET_SPEED,
                traveled: 0.,
                size: BULLET_SIZE,
                damage: BULLET_DAMAGE,
            },
        }
    }
}
