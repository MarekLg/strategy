use bevy::prelude::*;

#[derive(Component)]
pub struct Unit {
    pub order: Option<Order>,
}

impl Unit {
    pub fn new() -> Self {
        Self { order: None }
    }
}

#[derive(Debug)]
pub enum Order {
    Move { tile_entity: Entity },
}
