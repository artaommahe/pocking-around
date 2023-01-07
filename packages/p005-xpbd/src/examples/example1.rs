use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::xpbd;

pub struct Example1Plugin;

impl Plugin for Example1Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Example1Plugin::setup);
    }
}

impl Example1Plugin {
    fn setup(
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
}
