use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use xpbd::XpbdPlugin;

fn main() {
    App::new()
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(XpbdPlugin)
        .add_plugin(Example1Plugin)
        .add_startup_system(app_startup)
        .run();
}

fn app_startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub struct Example1Plugin;

impl Plugin for Example1Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Example1Plugin::startup)
            .insert_resource(xpbd::resources::Gravity(Vec2::ZERO));
    }
}

impl Example1Plugin {
    fn startup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(20.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                ..default()
            })
            .insert(xpbd::components::ParticleBundle::new_with_pos_and_vel(
                Vec2::new(-50., 0.),
                Vec2::new(60., 0.),
            ))
            .insert(xpbd::components::Mass(10.));

        commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(20.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                ..default()
            })
            .insert(xpbd::components::ParticleBundle::new_with_pos_and_vel(
                Vec2::new(50., 0.),
                Vec2::new(-60., 0.),
            ))
            .insert(xpbd::components::Mass(2.));
    }
}
