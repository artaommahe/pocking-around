use bevy::prelude::*;
mod plugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(plugins::camera::CameraPlugin)
        .add_plugin(plugins::debug::DebugPlugin)
        .add_plugin(plugins::player::PlayerPlugin)
        .add_plugin(plugins::wall::WallPlugin)
        .add_plugin(plugins::random_obstacles::RandomObstaclesPlugin)
        .add_plugin(plugins::projectile::ProjectilePlugin)
        .add_plugin(plugins::weapon::weapon::WeaponPlugin)
        .run();
}
