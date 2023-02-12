use std::time::Duration;

use bevy::prelude::*;

use crate::plugins::projectile::Projectile;

use super::common::{CurrentWeaponThrottle, FireEvent, FiredBullet, WeaponBullet, WeaponUi};

pub struct RiflePlugin;

impl Plugin for RiflePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(RiflePlugin::fire);
    }
}

impl RiflePlugin {
    fn fire(
        mut commands: Commands,
        mut fire_event: EventReader<FireEvent>,
        mut current_weapon_throttle: ResMut<CurrentWeaponThrottle>,
    ) {
        for FireEvent {
            weapon_id,
            player_translation,
            player_rotation,
        } in fire_event.iter()
        {
            if *weapon_id != RIFLE_WEAPON.id {
                continue;
            }

            current_weapon_throttle
                .fire
                .set_duration(Duration::from_secs_f32(RIFLE_WEAPON.bullet.throttle));
            current_weapon_throttle.fire.reset();

            let position_correction = player_rotation.mul_vec3(Vec3::Y * 20.);
            let transform = Transform {
                translation: player_translation.clone() + position_correction,
                scale: RIFLE_WEAPON.bullet.size.extend(1.0),
                rotation: player_rotation.clone(),
                ..default()
            };

            commands
                .spawn(FiredBullet {
                    sprite: SpriteBundle {
                        transform,
                        sprite: Sprite {
                            color: Color::hex(RIFLE_WEAPON.bullet.color)
                                .expect("wrong bullet color"),
                            custom_size: Some(Vec2::new(1.0, 1.0)),
                            ..default()
                        },
                        ..default()
                    },
                    projectile: Projectile {
                        speed: RIFLE_WEAPON.bullet.speed,
                        traveled: 0.,
                        size: RIFLE_WEAPON.bullet.size,
                        damage: RIFLE_WEAPON.bullet.damage,
                        max_travel_distance: RIFLE_WEAPON.bullet.max_travel_distance,
                    },
                })
                .insert(Name::new(RIFLE_WEAPON.label.to_owned() + &" bullet"));
        }
    }
}

#[derive(Debug)]
pub struct RifleWeapon {
    pub id: &'static str,
    label: &'static str,
    bullet: WeaponBullet,
    pub ui: WeaponUi,
}

pub const RIFLE_WEAPON: RifleWeapon = RifleWeapon {
    id: "rifle",
    label: "Rifle",
    bullet: WeaponBullet {
        size: Vec2::new(2., 20.),
        color: "aff8db",
        speed: 1000.,
        damage: 30.,
        throttle: 0.1,
        max_travel_distance: 400.,
    },
    ui: WeaponUi {
        label: "Rf",
        color: "aff8db",
    },
};
