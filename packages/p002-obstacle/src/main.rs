use bevy::prelude::*;
mod components;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(components::player::setup_player)
        .add_startup_system(components::wall::setup_wall)
        .add_system(components::player::move_player)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
