use std::time::Duration;

use bevy::{prelude::*, sprite::collide_aabb::collide};

use super::{
    collider::{Collider, ColliderTarget},
    player::Player,
    status::Health,
};

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LastFiredProjectile {
            time: Duration::ZERO,
        })
        .init_resource::<CollidedProjectiles>()
        .add_system(WeaponPlugin::fire_bullet)
        .add_system(WeaponPlugin::projectile_movement)
        .add_system(WeaponPlugin::projectile_collision.after(WeaponPlugin::projectile_movement))
        .add_system(WeaponPlugin::projectile_damage.after(WeaponPlugin::projectile_collision))
        .add_system(WeaponPlugin::projectile_cleanup.after(WeaponPlugin::projectile_damage));
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

    fn projectile_collision(
        projectiles: Query<(Entity, &Projectile, &Transform)>,
        collider_targets: Query<(Entity, &Transform, &Collider)>,
        mut collided_projectiles: ResMut<CollidedProjectiles>,
    ) {
        for (entity, projectile, transform) in projectiles.iter() {
            let projectile_front_position = transform.translation
                + transform.rotation.mul_vec3(projectile.size.extend(1.) / 2.);

            let target = ColliderTarget {
                position: projectile_front_position,
                size: Vec2::splat(1.),
            };

            for (obstacle_entity, obstacle_transform, obstacle_collider) in
                collider_targets.into_iter()
            {
                let obstacle_collision = collide(
                    target.position,
                    target.size,
                    obstacle_transform.translation,
                    obstacle_collider.size,
                );

                if obstacle_collision.is_some() {
                    collided_projectiles.0.push(ProjectileCollision {
                        collided_entity: obstacle_entity,
                        projectile_entity: entity,
                        projectile_damage: projectile.damage,
                    });

                    break;
                }
            }
        }
    }

    fn projectile_damage(
        mut commands: Commands,
        collided_projectiles: Res<CollidedProjectiles>,
        mut collided_targets: Query<&mut Health>,
    ) {
        for ProjectileCollision {
            collided_entity,
            projectile_damage,
            ..
        } in collided_projectiles.0.iter().cloned()
        {
            if projectile_damage <= 0. {
                continue;
            }

            if let Ok(mut target_health) = collided_targets.get_mut(collided_entity) {
                target_health.current -= projectile_damage.max(0.);

                if target_health.current <= 0. {
                    commands.entity(collided_entity).despawn();
                }
            }
        }
    }

    fn projectile_cleanup(
        mut commands: Commands,
        projectiles: Query<(Entity, &Projectile)>,
        mut collided_projectiles: ResMut<CollidedProjectiles>,
    ) {
        for (entity, projectile) in projectiles.iter() {
            if projectile.traveled > MAX_TRAVEL_DISTANCE {
                commands.entity(entity).despawn();
            }
        }

        for ProjectileCollision {
            projectile_entity, ..
        } in collided_projectiles.0.iter().cloned()
        {
            commands.entity(projectile_entity).despawn();
        }

        collided_projectiles.0.clear();
    }
}

#[derive(Component, Debug)]
struct Projectile {
    speed: f32,
    traveled: f32,
    size: Vec2,
    damage: f32,
}

#[derive(Resource, Debug)]
struct LastFiredProjectile {
    // TODO: rewrite with Some(Timer)
    time: Duration,
}

#[derive(Debug, Clone)]
struct ProjectileCollision {
    collided_entity: Entity,
    projectile_entity: Entity,
    projectile_damage: f32,
}

#[derive(Resource, Default, Debug)]
struct CollidedProjectiles(pub Vec<ProjectileCollision>);

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
