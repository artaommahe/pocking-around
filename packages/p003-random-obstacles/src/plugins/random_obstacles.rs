use bevy::prelude::*;
use rand::{thread_rng, Rng};

use super::collider::Collider;

pub struct RandomObstaclesPlugin;

impl Plugin for RandomObstaclesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(RandomObstaclesPlugin::setup);
    }
}

struct ObstacleBounds {
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
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
    }
}

const OBSTACLES_COUNT: u32 = 100;
const OBSTACLES_DISTANCE: i32 = 300;
const OBSTACLES_PLACEMENT_GAP: f32 = 10.;

fn generate_obstales() -> Vec<Vec2> {
    let mut placed_obstacles = vec![];
    let mut placed_obstacles_bounds = vec![];
    let mut rng = thread_rng();

    for _ in 0..OBSTACLES_COUNT {
        let mut obstacle_position: Vec2;

        loop {
            obstacle_position = Vec2::new(
                rng.gen_range(-OBSTACLES_DISTANCE..OBSTACLES_DISTANCE) as f32,
                rng.gen_range(-OBSTACLES_DISTANCE..OBSTACLES_DISTANCE) as f32,
            );

            if verify_obstacle_position(&placed_obstacles_bounds, obstacle_position) {
                break;
            }
        }

        placed_obstacles.push(obstacle_position);
        placed_obstacles_bounds.push(get_obstacle_bounds(obstacle_position, 0.));
    }

    placed_obstacles
}

fn verify_obstacle_position(placed_obstacles: &[ObstacleBounds], new_obstacle: Vec2) -> bool {
    let new_obstacle_bounds = get_obstacle_bounds(new_obstacle, OBSTACLES_PLACEMENT_GAP);

    !placed_obstacles.iter().any(|obstacle_bounds| {
        obstacle_bounds.x1 < new_obstacle_bounds.x2
            && obstacle_bounds.x2 > new_obstacle_bounds.x1
            && obstacle_bounds.y1 < new_obstacle_bounds.y2
            && obstacle_bounds.y2 > new_obstacle_bounds.y1
    })
}

fn get_obstacle_bounds(obstacle_position: Vec2, gap: f32) -> ObstacleBounds {
    ObstacleBounds {
        x1: obstacle_position.x - gap,
        x2: obstacle_position.x + OBSTACLE_SIZE.x + gap,
        y1: obstacle_position.y - gap,
        y2: obstacle_position.y + OBSTACLE_SIZE.y + gap,
    }
}
