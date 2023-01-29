use std::time::Duration;

use bevy::prelude::*;

use super::{
    collider::{check_collision, Collider, ColliderTarget},
    player::Player,
};

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LastFiredProjectile {
            time: Duration::ZERO,
        })
        .add_system(WeaponPlugin::fire_bullet)
        .add_system(WeaponPlugin::projectile_movement)
        .add_system(WeaponPlugin::projectile_cleanup)
        .add_system(WeaponPlugin::projectile_collision);
    }
}

const MAX_TRAVEL_DISTANCE: f32 = 450.;
const PROJECTILES_THROTTLE: u128 = 200;

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

    fn projectile_collision(
        mut commands: Commands,
        projectiles: Query<(Entity, &Projectile, &Transform)>,
        collider_targets: Query<(&Transform, &Collider)>,
    ) {
        let collider_targets_vec = collider_targets.into_iter().collect();

        for (entity, projectile, transform) in projectiles.iter() {
            let projectile_front_position = transform.translation
                + transform.rotation.mul_vec3(projectile.size.extend(1.) / 2.);

            if let Some(_) = check_collision(
                ColliderTarget {
                    position: projectile_front_position,
                    size: Vec2::splat(1.),
                },
                &collider_targets_vec,
            ) {
                commands.entity(entity).despawn();
            }
        }
    }
}

#[derive(Component, Debug)]
struct Projectile {
    speed: f32,
    traveled: f32,
    size: Vec2,
}

#[derive(Resource, Debug)]
struct LastFiredProjectile {
    time: Duration,
}

const BULLET_SIZE: Vec2 = Vec2::new(2., 15.);
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
                size: BULLET_SIZE,
            },
        }
    }
}
