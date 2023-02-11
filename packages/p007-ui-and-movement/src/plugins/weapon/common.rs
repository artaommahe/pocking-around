use bevy::prelude::*;

use crate::plugins::projectile::Projectile;

#[derive(Debug)]
pub struct WeaponBullet {
    pub size: Vec2,
    pub color: &'static str,
    pub speed: f32,
    pub damage: f32,
    pub throttle: f32,
    pub max_travel_distance: f32,
}

#[derive(Bundle)]
pub struct FiredBullet {
    pub sprite: SpriteBundle,
    pub projectile: Projectile,
}

pub struct FireEvent {
    pub weapon_id: &'static str,
    pub player_translation: Vec3,
    pub player_rotation: Quat,
}
