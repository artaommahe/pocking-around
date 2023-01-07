use bevy::prelude::*;
mod examples;
mod xpbd;

// https://johanhelsing.studio/posts/bevy_xpbd
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(xpbd::plugin::XpbdPlugin)
        .add_plugin(examples::example1::Example1Plugin)
        .insert_resource(xpbd::resources::Gravity(Vec2::ZERO))
        .run();
}
