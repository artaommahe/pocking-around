use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct CircleCollider {
    pub radius: f32,
}

impl Default for CircleCollider {
    fn default() -> Self {
        Self { radius: 20. }
    }
}

#[derive(Component, Debug)]
pub struct BoxCollider {
    pub size: Vec2,
}

impl Default for BoxCollider {
    fn default() -> Self {
        Self { size: Vec2::ONE }
    }
}
