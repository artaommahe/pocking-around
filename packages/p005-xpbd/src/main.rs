use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
mod xpbd;

// https://johanhelsing.studio/posts/bevy_xpbd
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(xpbd::plugin::XpbdPlugin)
        .insert_resource(xpbd::resources::Gravity(Vec2::ZERO))
        .add_startup_system(app_setup)
        .run();
}

fn app_setup(
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

    commands.spawn(Camera2dBundle::default());
}
