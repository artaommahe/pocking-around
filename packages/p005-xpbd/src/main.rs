use bevy::prelude::*;
mod examples;
mod xpbd;

// https://johanhelsing.studio/posts/bevy_xpbd
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(xpbd::plugin::XpbdPlugin)
        .add_plugin(examples::example3::Example3Plugin)
        .add_startup_system(app_startup)
        .run();
}

fn app_startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
