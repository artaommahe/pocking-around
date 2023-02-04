use bevy::{prelude::*, sprite::collide_aabb::collide};

use super::{
    collider::{Collider, ColliderTarget},
    status::Health,
};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CollidedProjectiles>()
            .add_system(ProjectilePlugin::projectile_movement)
            .add_system(
                ProjectilePlugin::projectile_collision.after(ProjectilePlugin::projectile_movement),
            )
            .add_system(
                ProjectilePlugin::projectile_damage.after(ProjectilePlugin::projectile_collision),
            )
            .add_system(
                ProjectilePlugin::projectile_cleanup.after(ProjectilePlugin::projectile_damage),
            );
    }
}

impl ProjectilePlugin {
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
            if projectile.traveled > projectile.max_travel_distance {
                commands.entity(entity).despawn();
            }
        }

        for ProjectileCollision {
            projectile_entity, ..
        } in collided_projectiles.0.iter().cloned()
        {
            // NOTE: may be already despawned by max travel distance check
            commands.entity(projectile_entity).despawn();
        }

        collided_projectiles.0.clear();
    }
}

#[derive(Component, Debug)]
pub struct Projectile {
    pub speed: f32,
    pub traveled: f32,
    pub size: Vec2,
    pub damage: f32,
    pub max_travel_distance: f32,
}

#[derive(Debug, Clone)]
struct ProjectileCollision {
    collided_entity: Entity,
    projectile_entity: Entity,
    projectile_damage: f32,
}

#[derive(Resource, Default, Debug)]
struct CollidedProjectiles(pub Vec<ProjectileCollision>);
