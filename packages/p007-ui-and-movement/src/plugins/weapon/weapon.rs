use std::time::Duration;

use bevy::prelude::*;

use crate::plugins::player::Player;

use super::{
    common::{CurrentWeaponThrottle, FireEvent, WeaponUi},
    pistol::{PistolPlugin, PISTOL_WEAPON},
    rifle::{RiflePlugin, RIFLE_WEAPON},
    shotgun::{ShotgunPlugin, SHOTGUN_WEAPON},
};

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        let mut switch_throttle = Timer::from_seconds(SWITCH_WEAPON_DELAY, TimerMode::Once);
        switch_throttle.set_elapsed(switch_throttle.duration());

        app.insert_resource(CurrentWeapon {
            weapon_id: PISTOL_WEAPON.id,
        })
        .insert_resource(CurrentWeaponThrottle {
            switch: switch_throttle,
            fire: Timer::from_seconds(0., TimerMode::Once),
        })
        .add_event::<FireEvent>()
        .add_event::<ChangeWeaponEvent>()
        .add_startup_system(WeaponPlugin::setup)
        .add_system(WeaponPlugin::hotkeys)
        .add_system(WeaponPlugin::fire)
        .add_system(WeaponPlugin::change_weapon)
        .add_system(WeaponPlugin::weapon_button)
        .add_system(WeaponPlugin::set_active_weapon_button)
        .add_system(WeaponPlugin::highlight_current_weapon_buttons)
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
                spawn_weapon_button(parent, &asset_server, PISTOL_WEAPON.id, &PISTOL_WEAPON.ui);
                spawn_weapon_button(parent, &asset_server, SHOTGUN_WEAPON.id, &SHOTGUN_WEAPON.ui);
                spawn_weapon_button(parent, &asset_server, RIFLE_WEAPON.id, &RIFLE_WEAPON.ui);
            });
    }

    fn fire(
        mouse_input: Res<Input<MouseButton>>,
        keyboard_input: Res<Input<KeyCode>>,
        player_query: Query<&Transform, With<Player>>,
        time: Res<Time>,
        current_weapon: Res<CurrentWeapon>,
        mut current_weapon_throttle: ResMut<CurrentWeaponThrottle>,
        mut fire_event: EventWriter<FireEvent>,
    ) {
        current_weapon_throttle.fire.tick(time.delta());
        current_weapon_throttle.switch.tick(time.delta());

        if !(mouse_input.pressed(MouseButton::Left) || keyboard_input.pressed(KeyCode::Space))
            || !current_weapon_throttle.fire.finished()
            || !current_weapon_throttle.switch.finished()
        {
            return;
        }

        let player_transform = player_query.single();

        fire_event.send(FireEvent {
            weapon_id: current_weapon.weapon_id,
            player_translation: player_transform.translation,
            player_rotation: player_transform.rotation,
        });
    }

    fn change_weapon(
        mut current_weapon: ResMut<CurrentWeapon>,
        mut current_weapon_throttle: ResMut<CurrentWeaponThrottle>,
        mut change_weapon_event: EventReader<ChangeWeaponEvent>,
    ) {
        for ChangeWeaponEvent { weapon_id } in change_weapon_event.iter() {
            if *weapon_id == current_weapon.weapon_id {
                continue;
            }

            current_weapon.weapon_id = weapon_id;
            current_weapon_throttle.fire.set_duration(Duration::ZERO);
            current_weapon_throttle.switch.reset();
        }
    }

    fn hotkeys(
        keyboard_input: Res<Input<KeyCode>>,
        mut change_weapon_event: EventWriter<ChangeWeaponEvent>,
    ) {
        let new_weapon: Option<&str> = match keyboard_input.get_just_pressed().next() {
            Some(KeyCode::Key1) => Some(PISTOL_WEAPON.id),
            Some(KeyCode::Key2) => Some(SHOTGUN_WEAPON.id),
            Some(KeyCode::Key3) => Some(RIFLE_WEAPON.id),
            _ => None,
        };

        match new_weapon {
            Some(weapon_id) => change_weapon_event.send(ChangeWeaponEvent { weapon_id }),
            None => {}
        }
    }

    fn weapon_button(
        mut interaction_query: Query<
            (&Interaction, &mut BackgroundColor, &WeaponButton),
            (Changed<Interaction>, With<Button>),
        >,
        mut change_weapon_event: EventWriter<ChangeWeaponEvent>,
    ) {
        for (interaction, mut background_color, weapon_button) in &mut interaction_query {
            if weapon_button.active {
                continue;
            }

            match *interaction {
                Interaction::Hovered => {
                    background_color.0.set_a(1.);
                }
                Interaction::None => {
                    background_color.0.set_a(UNHOVERED_WEAPON_BUTTON_ALPHA);
                }
                // for now button click propagates to the fire system
                // https://github.com/bevyengine/bevy/issues/3570
                Interaction::Clicked => change_weapon_event.send(ChangeWeaponEvent {
                    weapon_id: weapon_button.weapon_id,
                }),
            }
        }
    }

    fn set_active_weapon_button(
        current_weapon: Res<CurrentWeapon>,
        mut buttons_query: Query<&mut WeaponButton, With<Button>>,
    ) {
        if !current_weapon.is_changed() {
            return;
        }

        for mut weapon_button in buttons_query.iter_mut() {
            weapon_button.active = weapon_button.weapon_id == current_weapon.weapon_id;
        }
    }

    fn highlight_current_weapon_buttons(
        mut buttons_query: Query<
            (&mut BackgroundColor, &WeaponButton),
            (With<Button>, Changed<WeaponButton>),
        >,
    ) {
        for (mut background_color, weapon_button) in buttons_query.iter_mut() {
            if weapon_button.active {
                background_color.0.set_a(1.);
            } else {
                background_color.0.set_a(UNHOVERED_WEAPON_BUTTON_ALPHA);
            }
        }
    }
}

const SWITCH_WEAPON_DELAY: f32 = 0.5;
const UNHOVERED_WEAPON_BUTTON_ALPHA: f32 = 0.5;

#[derive(Resource)]
struct CurrentWeapon {
    weapon_id: &'static str,
}

struct ChangeWeaponEvent {
    weapon_id: &'static str,
}

#[derive(Component)]
struct WeaponButton {
    weapon_id: &'static str,
    active: bool,
}

fn spawn_weapon_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    weapon_id: &'static str,
    weapon_ui: &WeaponUi,
) {
    let mut background_color = Color::hex(weapon_ui.color).unwrap();
    background_color.set_a(UNHOVERED_WEAPON_BUTTON_ALPHA);

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
            background_color: background_color.into(),
            ..default()
        })
        .insert(WeaponButton {
            weapon_id,
            active: false,
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                weapon_ui.label,
                TextStyle {
                    font: asset_server.load("fonts/OpenSans-Regular.ttf"),
                    font_size: 32.,
                    color: Color::hex("888888").unwrap(),
                },
            ));
        });
}
