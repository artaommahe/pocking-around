use std::f32::consts::PI;

use bevy::prelude::*;

use crate::plugins::projectile::Projectile;

use super::common::{FireEvent, FiredBullet, WeaponBullet};

pub struct ShotgunPlugin;

impl Plugin for ShotgunPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(ShotgunPlugin::fire);
    }
}

impl ShotgunPlugin {
    fn fire(mut commands: Commands, mut fire_event: EventReader<FireEvent>) {
        for FireEvent {
            weapon_id,
            player_translation,
            player_rotation,
        } in fire_event.iter()
        {
            if *weapon_id != SHOTGUN_WEAPON.id {
                continue;
            }

            let mut rotation =
                player_rotation.mul_quat(Quat::from_axis_angle(Vec3::Z, -SHOTGUN_DISPERSION_ANGLE));
            let angle_correction = SHOTGUN_DISPERSION_ANGLE * 2. / SHOTGUN_SHOTS_COUNT as f32;

            for _ in 0..SHOTGUN_SHOTS_COUNT {
                let position_correction = rotation.mul_vec3(Vec3::Y * 20.);
                let transform = Transform {
                    translation: player_translation.clone() + position_correction,
                    scale: SHOTGUN_WEAPON.bullet.size.extend(1.0),
                    rotation,
                    ..default()
                };

                commands
                    .spawn(FiredBullet {
                        sprite: SpriteBundle {
                            transform,
                            sprite: Sprite {
                                color: Color::hex(SHOTGUN_WEAPON.bullet.color)
                                    .expect("wrong bullet color"),
                                custom_size: Some(Vec2::new(1.0, 1.0)),
                                ..default()
                            },
                            ..default()
                        },
                        projectile: Projectile {
                            speed: SHOTGUN_WEAPON.bullet.speed,
                            traveled: 0.,
                            size: SHOTGUN_WEAPON.bullet.size,
                            damage: SHOTGUN_WEAPON.bullet.damage,
                            max_travel_distance: SHOTGUN_WEAPON.bullet.max_travel_distance,
                        },
                    })
                    .insert(Name::new(SHOTGUN_WEAPON.label.to_owned() + &" bullet"));

                rotation = rotation.mul_quat(Quat::from_axis_angle(Vec3::Z, angle_correction));
            }
        }
    }
}

const SHOTGUN_DISPERSION_ANGLE: f32 = PI / 24.;
const SHOTGUN_SHOTS_COUNT: u32 = 8;

#[derive(Debug)]
pub struct ShotgunWeapon {
    pub id: &'static str,
    pub label: &'static str,
    pub short_label: &'static str,
    pub bullet: WeaponBullet,
}

pub const SHOTGUN_WEAPON: ShotgunWeapon = ShotgunWeapon {
    id: "shotgun",
    label: "Shotgun",
    short_label: "Sh",
    bullet: WeaponBullet {
        size: Vec2::new(1., 10.),
        color: "ffffd1",
        speed: 700.,
        damage: 10.,
        throttle: 1.,
        max_travel_distance: 150.,
    },
};
