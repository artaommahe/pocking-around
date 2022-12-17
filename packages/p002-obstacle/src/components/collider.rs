use bevy::{prelude::*, sprite::collide_aabb::collide};

use super::player::Player;

#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
}

pub struct ColliderTarget {
    pub position: Vec3,
    pub size: Vec2,
}

pub fn check_collision(
    target: ColliderTarget,
    obstacles: &Query<(&Transform, &Collider), Without<Player>>,
) -> bool {
    for (obstacle_transform, obstacle_collider) in obstacles.iter() {
        let collision = collide(
            target.position,
            target.size,
            obstacle_transform.translation,
            obstacle_collider.size,
        );

        if collision.is_some() {
            return false;
        }
    }

    true
}
