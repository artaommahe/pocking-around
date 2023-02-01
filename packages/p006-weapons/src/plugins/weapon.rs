use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;

use super::{player::Player, projectile::Projectile};

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentWeapon {
            weapon: Weapon::Pistol(PistolWeapon::new()),
            fire_throttle: Timer::new(Duration::from_millis(0), TimerMode::Once),
        })
        .add_system(WeaponPlugin::fire)
        .add_system(WeaponPlugin::change_weapon);
    }
}

impl WeaponPlugin {
    fn fire(
        commands: Commands,
        mouse_input: Res<Input<MouseButton>>,
        keyboard_input: Res<Input<KeyCode>>,
        player_query: Query<&Transform, With<Player>>,
        time: Res<Time>,
        mut current_weapon: ResMut<CurrentWeapon>,
    ) {
        current_weapon.fire_throttle.tick(time.delta());

        if (mouse_input.pressed(MouseButton::Left) || keyboard_input.pressed(KeyCode::Space))
            && (current_weapon.fire_throttle.duration().is_zero()
                || current_weapon.fire_throttle.finished())
        {
            let player_transform = player_query.single();

            match &current_weapon.weapon {
                Weapon::Pistol(weapon) => {
                    weapon.fire(commands, player_transform);

                    let throttle_duration = weapon.bullet.throttle;
                    current_weapon
                        .fire_throttle
                        .set_duration(Duration::from_millis(throttle_duration));
                }
                Weapon::Shotgun(weapon) => {
                    weapon.fire(commands, player_transform);

                    let throttle_duration = weapon.bullet.throttle;
                    current_weapon
                        .fire_throttle
                        .set_duration(Duration::from_millis(throttle_duration));
                }
                Weapon::Rifle(weapon) => {
                    weapon.fire(commands, player_transform);

                    let throttle_duration = weapon.bullet.throttle;
                    current_weapon
                        .fire_throttle
                        .set_duration(Duration::from_millis(throttle_duration));
                }
            }

            current_weapon.fire_throttle.reset();
        }
    }

    fn change_weapon(
        keyboard_input: Res<Input<KeyCode>>,
        mut current_weapon: ResMut<CurrentWeapon>,
    ) {
        let new_weapon: Option<Weapon> = match keyboard_input.get_just_pressed().next() {
            Some(KeyCode::Key1) => Some(Weapon::Pistol(PistolWeapon::new())),
            Some(KeyCode::Key2) => Some(Weapon::Shotgun(ShotgunWeapon::new())),
            Some(KeyCode::Key3) => Some(Weapon::Rifle(RifleWeapon::new())),
            _ => None,
        };

        match new_weapon {
            Some(weapon)
                if std::mem::discriminant(&current_weapon.weapon)
                    != std::mem::discriminant(&weapon) =>
            {
                current_weapon.weapon = weapon;

                current_weapon.fire_throttle =
                    Timer::new(Duration::from_millis(SWITCH_WEAPON_DELAY), TimerMode::Once);
            }
            Some(_) | None => {}
        }
    }
}

const SWITCH_WEAPON_DELAY: u64 = 500;

#[derive(Resource)]
struct CurrentWeapon {
    weapon: Weapon,
    fire_throttle: Timer,
}

#[derive(Debug)]
enum Weapon {
    Pistol(PistolWeapon),
    Shotgun(ShotgunWeapon),
    Rifle(RifleWeapon),
}

#[derive(Bundle)]
struct FiredBullet {
    sprite: SpriteBundle,
    projectile: Projectile,
}

#[derive(Debug)]
struct WeaponBullet {
    size: Vec2,
    color: &'static str,
    speed: f32,
    damage: f32,
    throttle: u64,
}

#[derive(Debug)]
struct PistolWeapon {
    name: &'static str,
    bullet: WeaponBullet,
}

impl PistolWeapon {
    fn new() -> Self {
        PistolWeapon {
            name: "Pistol",
            bullet: WeaponBullet {
                size: Vec2::new(3., 15.),
                color: "ffc48c",
                speed: 750.,
                damage: 20.,
                throttle: 500,
            },
        }
    }

    fn fire(&self, mut commands: Commands, player_transform: &Transform) {
        let position_correction = player_transform.rotation.mul_vec3(Vec3::Y * 20.);
        let transform = Transform {
            translation: player_transform.translation.clone() + position_correction,
            scale: self.bullet.size.extend(1.0),
            rotation: player_transform.rotation.clone(),
            ..default()
        };

        commands
            .spawn(FiredBullet {
                sprite: SpriteBundle {
                    transform,
                    sprite: Sprite {
                        color: Color::hex(self.bullet.color).expect("wrong bullet color"),
                        custom_size: Some(Vec2::new(1.0, 1.0)),
                        ..default()
                    },
                    ..default()
                },
                projectile: Projectile {
                    speed: self.bullet.speed,
                    traveled: 0.,
                    size: self.bullet.size,
                    damage: self.bullet.damage,
                },
            })
            .insert(Name::new(self.name.to_owned() + &" bullet"));
    }
}

#[derive(Debug)]
struct ShotgunWeapon {
    name: &'static str,
    bullet: WeaponBullet,
}

const SHOTGUN_DISPERSION_ANGLE: f32 = PI / 24.;
const SHOTGUN_SHOTS_COUNT: u32 = 8;

impl ShotgunWeapon {
    fn new() -> Self {
        ShotgunWeapon {
            name: "Shotgun",
            bullet: WeaponBullet {
                size: Vec2::new(1., 10.),
                color: "ffffd1",
                speed: 700.,
                damage: 10.,
                throttle: 1000,
            },
        }
    }

    fn fire(&self, mut commands: Commands, player_transform: &Transform) {
        let mut rotation = player_transform
            .rotation
            .mul_quat(Quat::from_axis_angle(Vec3::Z, -SHOTGUN_DISPERSION_ANGLE));
        let angle_correction = SHOTGUN_DISPERSION_ANGLE * 2. / SHOTGUN_SHOTS_COUNT as f32;

        for _ in 0..SHOTGUN_SHOTS_COUNT {
            let position_correction = rotation.mul_vec3(Vec3::Y * 20.);
            let transform = Transform {
                translation: player_transform.translation.clone() + position_correction,
                scale: self.bullet.size.extend(1.0),
                rotation,
                ..default()
            };

            commands
                .spawn(FiredBullet {
                    sprite: SpriteBundle {
                        transform,
                        sprite: Sprite {
                            color: Color::hex(self.bullet.color).expect("wrong bullet color"),
                            custom_size: Some(Vec2::new(1.0, 1.0)),
                            ..default()
                        },
                        ..default()
                    },
                    projectile: Projectile {
                        speed: self.bullet.speed,
                        traveled: 0.,
                        size: self.bullet.size,
                        damage: self.bullet.damage,
                    },
                })
                .insert(Name::new(self.name));

            rotation = rotation.mul_quat(Quat::from_axis_angle(Vec3::Z, angle_correction));
        }
    }
}

#[derive(Debug)]
struct RifleWeapon {
    name: &'static str,
    bullet: WeaponBullet,
}

impl RifleWeapon {
    fn new() -> Self {
        RifleWeapon {
            name: "Rifle",
            bullet: WeaponBullet {
                size: Vec2::new(2., 20.),
                color: "aff8db",
                speed: 1000.,
                damage: 30.,
                throttle: 100,
            },
        }
    }

    fn fire(&self, mut commands: Commands, player_transform: &Transform) {
        let position_correction = player_transform.rotation.mul_vec3(Vec3::Y * 20.);
        let transform = Transform {
            translation: player_transform.translation.clone() + position_correction,
            scale: self.bullet.size.extend(1.0),
            rotation: player_transform.rotation.clone(),
            ..default()
        };

        commands
            .spawn(FiredBullet {
                sprite: SpriteBundle {
                    transform,
                    sprite: Sprite {
                        color: Color::hex(self.bullet.color).expect("wrong bullet color"),
                        custom_size: Some(Vec2::new(1.0, 1.0)),
                        ..default()
                    },
                    ..default()
                },
                projectile: Projectile {
                    speed: self.bullet.speed,
                    traveled: 0.,
                    size: self.bullet.size,
                    damage: self.bullet.damage,
                },
            })
            .insert(Name::new(self.name));
    }
}
