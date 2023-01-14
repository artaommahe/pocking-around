use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
mod examples;
mod xpbd;

// https://johanhelsing.studio/posts/bevy_xpbd
fn main() {
    App::new()
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(xpbd::plugin::XpbdPlugin)
        // TOOD: listen for keys pressed to run different examples
        .add_plugin(examples::example4::Example4Plugin)
        .add_startup_system(app_startup)
        .run();
}

fn app_startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
