use bevy::prelude::*;

#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
}

pub struct ColliderTarget {
    pub position: Vec3,
    pub size: Vec2,
}
