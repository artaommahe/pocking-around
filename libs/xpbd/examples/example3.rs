use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use xpbd::{colliders::*, components::*, XpbdPlugin};

fn main() {
    App::new()
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(XpbdPlugin)
        .add_plugin(Example3Plugin)
        .add_startup_system(app_startup)
        .run();
}

fn app_startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub struct Example3Plugin;

impl Plugin for Example3Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Example3Plugin::spawn_balls);
    }
}

impl Example3Plugin {
    fn spawn_balls(
        mut commands: Commands,
        mut materials: ResMut<Assets<ColorMaterial>>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) {
        let sphere = meshes.add(shape::Circle::new(1.).into());
        let blue = materials.add(ColorMaterial::from(Color::MIDNIGHT_BLUE));

        let static_size = Vec2::new(1000., 20.);

        commands
            .spawn(ColorMesh2dBundle {
                mesh: meshes.add(shape::Box::new(1., 1., 1.).into()).into(),
                material: blue.clone(),
                transform: Transform::from_scale(static_size.extend(1.)),
                ..default()
            })
            .insert(StaticBoxBundle {
                pos: Pos(Vec2::new(0., -50.)),
                collider: BoxCollider { size: static_size },
                ..default()
            });

        let radius = 10.;
        let rows = 15;
        let columns = 10;

        for row in 0..rows {
            for column in 0..columns {
                let pos = Vec2::new(
                    (column as f32 - columns as f32 / 2.) * 2.5 * radius,
                    2. * radius * row as f32 - 2.,
                );
                let vel = Vec2::ZERO;

                commands
                    .spawn(MaterialMesh2dBundle {
                        mesh: sphere.clone().into(),
                        material: blue.clone(),
                        transform: Transform {
                            scale: Vec3::splat(radius),
                            translation: pos.extend(0.),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(ParticleBundle {
                        collider: CircleCollider { radius },
                        ..ParticleBundle::new_with_pos_and_vel(pos, vel)
                    });
            }
        }
    }
}
