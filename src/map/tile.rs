use bevy::prelude::*;

use crate::hex::{corner::Corner, position::Position};

#[derive(Debug, Clone, Copy)]
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

    pub fn center(&self) -> Vec3 {
        let offset = self.position.offset();

        Vec3 {
            x: offset.x,
            y: 0.0,
            z: offset.y,
        }
    }

    pub fn corner(&self, corner: &Corner) -> Vec3 {
        let offset = self.position.offset_corner(corner);

        Vec3 {
            x: offset.x,
            y: 0.0,
            z: offset.y,
        }
    }
}
