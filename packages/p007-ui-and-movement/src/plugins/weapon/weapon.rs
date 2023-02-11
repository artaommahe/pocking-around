use std::time::Duration;

use bevy::prelude::*;

use crate::plugins::player::Player;

use super::{
    common::FireEvent,
    pistol::{PistolPlugin, PISTOL_WEAPON},
    rifle::{RiflePlugin, RIFLE_WEAPON},
    shotgun::{ShotgunPlugin, SHOTGUN_WEAPON},
};

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        let mut fire_throttle = Timer::from_seconds(PISTOL_WEAPON.bullet.throttle, TimerMode::Once);
        fire_throttle.set_elapsed(fire_throttle.duration());
        let mut switch_throttle = Timer::from_seconds(SWITCH_WEAPON_DELAY, TimerMode::Once);
        switch_throttle.set_elapsed(switch_throttle.duration());

        app.insert_resource(CurrentWeapon {
            weapon_id: PISTOL_WEAPON.id,
            switch_throttle,
            fire_throttle,
        })
        .add_event::<FireEvent>()
        .add_startup_system(WeaponPlugin::setup)
        .add_system(WeaponPlugin::change_weapon)
        .add_system(WeaponPlugin::fire)
        .add_plugin(PistolPlugin)
        .add_plugin(ShotgunPlugin)
        .add_plugin(RiflePlugin);
    }
}

impl WeaponPlugin {
    fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands
            .spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexEnd,
                    padding: UiRect {
                        bottom: Val::Px(10.),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                spawn_weapon_button(parent, &asset_server, "ffc48c", PISTOL_WEAPON.short_label);
                spawn_weapon_button(parent, &asset_server, "ffffd1", SHOTGUN_WEAPON.short_label);
                spawn_weapon_button(parent, &asset_server, "aff8db", RIFLE_WEAPON.short_label);
            });
    }

    fn fire(
        mouse_input: Res<Input<MouseButton>>,
        keyboard_input: Res<Input<KeyCode>>,
        player_query: Query<&Transform, With<Player>>,
        time: Res<Time>,
        mut current_weapon: ResMut<CurrentWeapon>,
        mut fire_event: EventWriter<FireEvent>,
    ) {
        current_weapon.fire_throttle.tick(time.delta());
        current_weapon.switch_throttle.tick(time.delta());

        if !(mouse_input.pressed(MouseButton::Left) || keyboard_input.pressed(KeyCode::Space))
            || !current_weapon.fire_throttle.finished()
            || !current_weapon.switch_throttle.finished()
        {
            return;
        }

        let player_transform = player_query.single();

        fire_event.send(FireEvent {
            weapon_id: current_weapon.weapon_id,
            player_translation: player_transform.translation,
            player_rotation: player_transform.rotation,
        });

        current_weapon.fire_throttle.reset();
    }

    fn change_weapon(
        keyboard_input: Res<Input<KeyCode>>,
        mut current_weapon: ResMut<CurrentWeapon>,
    ) {
        let new_weapon: Option<(&str, f32)> = match keyboard_input.get_just_pressed().next() {
            Some(KeyCode::Key1) => Some((PISTOL_WEAPON.id, PISTOL_WEAPON.bullet.throttle)),
            Some(KeyCode::Key2) => Some((SHOTGUN_WEAPON.id, SHOTGUN_WEAPON.bullet.throttle)),
            Some(KeyCode::Key3) => Some((RIFLE_WEAPON.id, RIFLE_WEAPON.bullet.throttle)),
            _ => None,
        };

        match new_weapon {
            Some((weapon_id, fire_throttle)) if current_weapon.weapon_id != weapon_id => {
                let throttle = Duration::from_secs_f32(fire_throttle);

                current_weapon.weapon_id = weapon_id;
                current_weapon.fire_throttle.set_duration(throttle);
                current_weapon.fire_throttle.set_elapsed(throttle);
                current_weapon.switch_throttle.reset();
            }
            Some(_) | None => {}
        }
    }
}

const SWITCH_WEAPON_DELAY: f32 = 0.5;

#[derive(Resource)]
struct CurrentWeapon {
    weapon_id: &'static str,
    switch_throttle: Timer,
    fire_throttle: Timer,
}

#[derive(Component)]
struct WeaponButton {}

fn spawn_weapon_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    color_hex: &str,
    text: &str,
) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(50.), Val::Px(50.)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect {
                    right: Val::Px(10.),
                    ..default()
                },
                ..default()
            },
            background_color: Color::hex(color_hex).unwrap().into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font: asset_server.load("fonts/OpenSans-Regular.ttf"),
                    font_size: 32.,
                    color: Color::hex("888888").unwrap(),
                },
            ));
        });
}
