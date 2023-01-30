use std::ops::Add;

use bevy::{prelude::*, sprite::collide_aabb::collide};
use rand::{thread_rng, Rng};

use super::{collider::Collider, player::PLAYER_SIZE, status::Health};

pub struct RandomObstaclesPlugin;

impl Plugin for RandomObstaclesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(RandomObstaclesPlugin::setup);
    }
}

impl RandomObstaclesPlugin {
    fn setup(mut commands: Commands) {
        generate_obstales()
            .iter()
            .enumerate()
            .for_each(|(index, obstacle_position)| {
                commands
                    .spawn(get_obstacle(obstacle_position))
                    .insert(Name::new("Obstacle".to_owned() + &index.to_string()));
            })
    }
}

const OBSTACLE_COLOR: &str = "b2cefe";
const OBSTACLE_SIZE: Vec2 = Vec2::splat(20.0);

#[derive(Bundle)]
struct Obstacle {
    sprite: SpriteBundle,
    collider: Collider,
    health: Health,
}

fn get_obstacle(position: &Vec2) -> Obstacle {
    let transform = Transform {
        translation: position.extend(0.0),
        scale: OBSTACLE_SIZE.extend(1.0),
        ..default()
    };

    Obstacle {
        sprite: SpriteBundle {
            transform,
            sprite: Sprite {
                color: Color::hex(OBSTACLE_COLOR).expect("wrong obstacle color"),
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..default()
            },
            ..default()
        },
        collider: Collider {
            size: transform.scale.truncate(),
        },
        health: Health::from_value(100.),
    }
}

const OBSTACLES_COUNT: u32 = 100;
const OBSTACLES_DISTANCE: f32 = 400.;
const OBSTACLES_PLACEMENT_GAP: f32 = 20.;

fn generate_obstales() -> Vec<Vec2> {
    let mut placed_obstacles = vec![];
    let mut rng = thread_rng();

    for _ in 0..OBSTACLES_COUNT {
        let mut obstacle_position: Vec2;

        loop {
            obstacle_position = Vec2::new(
                rng.gen_range(-OBSTACLES_DISTANCE..OBSTACLES_DISTANCE),
                rng.gen_range(-OBSTACLES_DISTANCE..OBSTACLES_DISTANCE),
            );

            if verify_position(&placed_obstacles, &obstacle_position) {
                break;
            }
        }

        placed_obstacles.push(obstacle_position);
    }

    placed_obstacles
}

fn verify_position(placed_obstacles: &[Vec2], new_obstacle: &Vec2) -> bool {
    if collide(
        Vec3::new(0.0, 0.0, 0.0),
        Vec2::splat(PLAYER_SIZE + OBSTACLES_PLACEMENT_GAP),
        new_obstacle.extend(0.),
        OBSTACLE_SIZE,
    )
    .is_some()
    {
        return false;
    }

    !placed_obstacles.iter().any(|obstacle| {
        collide(
            obstacle.extend(0.),
            OBSTACLE_SIZE.add(OBSTACLES_PLACEMENT_GAP),
            new_obstacle.extend(0.),
            OBSTACLE_SIZE,
        )
        .is_some()
    })
}
