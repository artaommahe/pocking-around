use std::time::Duration;

use bevy::prelude::*;

use crate::plugins::projectile::Projectile;

use super::common::{CurrentWeaponThrottle, FireEvent, FiredBullet, WeaponBullet, WeaponUi};

pub struct PistolPlugin;

impl Plugin for PistolPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(PistolPlugin::fire);
    }
}

impl PistolPlugin {
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
            if *weapon_id != PISTOL_WEAPON.id {
                continue;
            }

            current_weapon_throttle
                .fire
                .set_duration(Duration::from_secs_f32(PISTOL_WEAPON.bullet.throttle));
            current_weapon_throttle.fire.reset();

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
    label: &'static str,
    bullet: WeaponBullet,
    pub ui: WeaponUi,
}

pub const PISTOL_WEAPON: PistolWeapon = PistolWeapon {
    id: "pistol",
    label: "Pistol",
    bullet: WeaponBullet {
        size: Vec2::new(3., 15.),
        color: "ffc48c",
        speed: 750.,
        damage: 20.,
        throttle: 0.5,
        max_travel_distance: 250.,
    },
    ui: WeaponUi {
        label: "Ps",
        color: "ffc48c",
    },
};
