use bevy::prelude::*;

use super::{
    colliders::{BoxCollider, CircleCollider},
    consts::SUB_DT,
};

#[derive(Component, Debug, Default)]
pub struct Pos(pub Vec2);

#[derive(Component, Debug, Default)]
pub struct PrevPos(pub Vec2);

#[derive(Component, Debug, Default)]
pub struct Vel(pub Vec2);

#[derive(Component, Debug, Default)]
pub struct PreSolveVel(pub Vec2);

#[derive(Component, Debug)]
pub struct Mass(pub f32);

impl Default for Mass {
    fn default() -> Self {
        Self(1.)
    }
}

#[derive(Component, Debug)]
pub struct Restitution(pub f32);

impl Default for Restitution {
    fn default() -> Self {
        Self(0.3)
    }
}

#[derive(Component, Debug, Default)]
pub struct Aabb {
    // bottom-left corner
    pub min: Vec2,
    // top-right corner
    pub max: Vec2,
}

impl Aabb {
    pub fn intersects(&self, other: &Self) -> bool {
        self.max.x >= other.min.x
            && self.max.y >= other.min.y
            && self.min.x <= other.max.x
            && self.min.y <= other.max.y
    }
}

#[derive(Bundle, Default)]
pub struct ParticleBundle {
    pub pos: Pos,
    pub prev_pos: PrevPos,
    pub vel: Vel,
    pub pre_solve_vel: PreSolveVel,
    pub mass: Mass,
    pub restitution: Restitution,
    pub collider: CircleCollider,
    pub aabb: Aabb,
}

impl ParticleBundle {
    pub fn new_with_pos_and_vel(pos: Vec2, vel: Vec2) -> Self {
        Self {
            pos: Pos(pos),
            prev_pos: PrevPos(pos - vel * SUB_DT),
            vel: Vel(vel),
            ..default()
        }
    }
}

#[derive(Bundle, Default)]
pub struct DynamicBoxBundle {
    pub pos: Pos,
    pub prev_pos: PrevPos,
    pub vel: Vel,
    pub pre_solve_vel: PreSolveVel,
    pub mass: Mass,
    pub restitution: Restitution,
    pub collider: BoxCollider,
    pub aabb: Aabb,
}

impl DynamicBoxBundle {
    pub fn new_with_pos_and_vel(pos: Vec2, vel: Vec2) -> Self {
        Self {
            pos: Pos(pos),
            prev_pos: PrevPos(pos - vel * SUB_DT),
            vel: Vel(vel),
            ..default()
        }
    }
}

#[derive(Bundle, Default)]
pub struct StaticCircleBundle {
    pub pos: Pos,
    pub collider: CircleCollider,
    pub restitution: Restitution,
}

#[derive(Bundle, Default)]
pub struct StaticBoxBundle {
    pub pos: Pos,
    pub collider: BoxCollider,
    pub restitution: Restitution,
}
