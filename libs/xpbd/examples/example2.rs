use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    sprite::MaterialMesh2dBundle,
    time::FixedTimestep,
};
use rand::random;
use xpbd::{colliders::*, components::*, XpbdPlugin};

fn main() {
    App::new()
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(XpbdPlugin)
        .add_plugin(Example2Plugin)
        .add_startup_system(app_startup)
        .run();
}

fn app_startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub struct Example2Plugin;

impl Plugin for Example2Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Example2Plugin::startup)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1. / 10.))
                    .with_system(Example2Plugin::spawn_marbles),
            )
            .add_system(Example2Plugin::despawn_marbles);
    }
}

#[derive(Resource)]
struct Materials {
    blue: Handle<ColorMaterial>,
}

#[derive(Resource)]
struct Meshes {
    sphere: Handle<Mesh>,
}

impl Example2Plugin {
    fn startup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
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
                pos: Pos(Vec2::new(0., -150.)),
                collider: BoxCollider { size: static_size },
                ..default()
            });

        commands.insert_resource(Meshes { sphere });
        commands.insert_resource(Materials { blue });
    }

    fn spawn_marbles(mut commands: Commands, materials: Res<Materials>, meshes: Res<Meshes>) {
        let radius = 10.;
        let pos = Vec2::new(
            (random::<f32>() - 0.5) * 300.,
            (random::<f32>() - 0.5) * 50.,
        ) + Vec2::Y * 150.;
        let vel = Vec2::new(
            (random::<f32>() - 0.5) * 10.,
            (random::<f32>() + 5.) * -100.,
        );

        commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes.sphere.clone().into(),
                material: materials.blue.clone(),
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

    fn despawn_marbles(mut commands: Commands, query: Query<(Entity, &Pos)>) {
        for (entity, pos) in query.iter() {
            if pos.0.y > 500. || pos.0.y < -800. || pos.0.x > 1000. || pos.0.x < -1000. {
                commands.entity(entity).despawn();
            }
        }
    }
}
