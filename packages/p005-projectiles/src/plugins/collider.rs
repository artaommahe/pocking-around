use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

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
    obstacles: &Vec<(&Transform, &Collider)>,
) -> Option<Collision> {
    for (obstacle_transform, obstacle_collider) in obstacles.into_iter() {
        let collision = collide(
            target.position,
            target.size,
            obstacle_transform.translation,
            obstacle_collider.size,
        );

        if collision.is_some() {
            return collision;
        }
    }

    None
}
