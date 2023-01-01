use bevy::prelude::*;

use super::{colliders::CircleCollider, consts::DELTA_TIME};

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

#[derive(Bundle, Default)]
pub struct ParticleBundle {
    pub pos: Pos,
    pub prev_pos: PrevPos,
    pub vel: Vel,
    pub pre_solve_vel: PreSolveVel,
    pub mass: Mass,
    pub restitution: Restitution,
    pub collider: CircleCollider,
}

impl ParticleBundle {
    pub fn new_with_pos_and_vel(pos: Vec2, vel: Vec2) -> Self {
        Self {
            pos: Pos(pos),
            prev_pos: PrevPos(pos - vel * DELTA_TIME),
            vel: Vel(vel),
            ..default()
        }
    }
}