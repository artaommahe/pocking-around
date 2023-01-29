use bevy::prelude::*;

use super::player::Player;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(WeaponPlugin::fire_bullet)
            .add_system(WeaponPlugin::projectile_movement)
            .add_system(WeaponPlugin::projectile_cleanup);
    }
}

const MAX_TRAVEL_DISTANCE: f32 = 350.;

impl WeaponPlugin {
    fn fire_bullet(
        mut commands: Commands,
        mouse_input: Res<Input<MouseButton>>,
        keyboard_input: Res<Input<KeyCode>>,
        player_query: Query<&Transform, With<Player>>,
    ) {
        if mouse_input.just_pressed(MouseButton::Left)
            || keyboard_input.just_pressed(KeyCode::Space)
        {
            let player_transform = player_query.single();

            commands.spawn(Bullet::from_player_position(player_transform));
        }
    }

    fn projectile_movement(
        mut projectiles: Query<(&mut Transform, &mut Projectile)>,
        time: Res<Time>,
    ) {
        for (mut transform, mut projectile) in projectiles.iter_mut() {
            let direction = transform.rotation.mul_vec3(Vec3::Y);
            let traveled_distance = projectile.speed * time.delta_seconds();

            transform.translation += direction * traveled_distance;
            projectile.traveled += traveled_distance;
        }
    }

    fn projectile_cleanup(mut commands: Commands, projectiles: Query<(Entity, &Projectile)>) {
        for (entity, projectile) in projectiles.iter() {
            if projectile.traveled > MAX_TRAVEL_DISTANCE {
                commands.entity(entity).despawn();
            }
        }
    }
}

#[derive(Component, Debug)]
struct Projectile {
    speed: f32,
    traveled: f32,
}

const BULLET_SIZE: Vec2 = Vec2::new(2., 10.);
const BULLET_COLOR: &str = "ffc48c";
const BULLET_SPEED: f32 = 750.;

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
            },
        }
    }
}
