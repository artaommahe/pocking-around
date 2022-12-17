use bevy::{prelude::*, sprite::SpriteBundle};

use super::collider::Collider;

const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
const WALL_THICKNESS: f32 = 10.0;
// x coordinates
const LEFT_WALL: f32 = -600.;
const RIGHT_WALL: f32 = 600.;
// y coordinates
const BOTTOM_WALL: f32 = -350.;
const TOP_WALL: f32 = 350.;

enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

#[derive(Bundle)]
pub struct Wall {
    sprite: SpriteBundle,
    collider: Collider,
}

impl Wall {
    fn new(location: WallLocation) -> Wall {
        let transform = Transform {
            translation: location.position().extend(0.0),
            scale: location.size().extend(1.0),
            ..default()
        };

        Wall {
            sprite: SpriteBundle {
                transform,
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider {
                size: transform.scale.truncate(),
            },
        }
    }
}

pub fn setup_wall(mut commands: Commands) {
    commands.spawn(Wall::new(WallLocation::Top));
    commands.spawn(Wall::new(WallLocation::Right));
    commands.spawn(Wall::new(WallLocation::Bottom));
    commands.spawn(Wall::new(WallLocation::Left));
}
