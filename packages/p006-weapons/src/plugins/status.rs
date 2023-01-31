use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Health {
    pub max: f32,
    pub current: f32,
}

impl Health {
    pub fn from_value(value: f32) -> Self {
        Health {
            max: value,
            current: value,
        }
    }
}
