use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct Gravity(pub Vec2);

impl Default for Gravity {
    fn default() -> Self {
        Self(Vec2::new(0., -9.81))
    }
}

#[derive(Default, Debug, Resource)]
pub struct Contacts(pub Vec<(Entity, Entity)>);