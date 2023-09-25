use bevy::prelude::*;

use crate::hex::position::Position;

pub struct Tile {
    position: Position,
    pub unit: Option<Entity>,
}

impl Tile {
    pub fn new(position: Position) -> Self {
        Self {
            position,
            unit: None,
        }
    }

    pub fn position(&self) -> &Position {
        &self.position
    }
}
