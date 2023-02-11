use bevy::prelude::*;

use crate::plugins::projectile::Projectile;

use super::common::{FireEvent, FiredBullet, WeaponBullet};

pub struct PistolPlugin;

impl Plugin for PistolPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(PistolPlugin::fire);
    }
}

impl PistolPlugin {
    fn fire(mut commands: Commands, mut fire_event: EventReader<FireEvent>) {
        for FireEvent {
            weapon_id,
            player_translation,
            player_rotation,
        } in fire_event.iter()
        {
            if *weapon_id != PISTOL_WEAPON.id {
                continue;
            }

            let position_correction = player_rotation.mul_vec3(Vec3::Y * 20.);
            let transform = Transform {
                translation: player_translation.clone() + position_correction,
                scale: PISTOL_WEAPON.bullet.size.extend(1.0),
                rotation: player_rotation.clone(),
                ..default()
            };

            commands
                .spawn(FiredBullet {
                    sprite: SpriteBundle {
                        transform,
                        sprite: Sprite {
                            color: Color::hex(PISTOL_WEAPON.bullet.color)
                                .expect("wrong bullet color"),
                            custom_size: Some(Vec2::new(1.0, 1.0)),
                            ..default()
                        },
                        ..default()
                    },
                    projectile: Projectile {
                        speed: PISTOL_WEAPON.bullet.speed,
                        traveled: 0.,
                        size: PISTOL_WEAPON.bullet.size,
                        damage: PISTOL_WEAPON.bullet.damage,
                        max_travel_distance: PISTOL_WEAPON.bullet.max_travel_distance,
                    },
                })
                .insert(Name::new(PISTOL_WEAPON.label.to_owned() + &" bullet"));
        }
    }
}

#[derive(Debug)]
pub struct PistolWeapon {
    pub id: &'static str,
    pub label: &'static str,
    pub short_label: &'static str,
    pub bullet: WeaponBullet,
}

pub const PISTOL_WEAPON: PistolWeapon = PistolWeapon {
    id: "pistol",
    label: "Pistol",
    short_label: "Ps",
    bullet: WeaponBullet {
        size: Vec2::new(3., 15.),
        color: "ffc48c",
        speed: 750.,
        damage: 20.,
        throttle: 0.5,
        max_travel_distance: 250.,
    },
};
