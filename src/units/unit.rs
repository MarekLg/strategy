use bevy::prelude::*;

use crate::tile::Tile;

#[derive(Component)]
pub struct Unit {
    pub tile: Tile,
    pub order: Option<Order>,
}

impl Unit {
    pub fn new(tile: Tile) -> Self {
        Self { tile, order: None }
    }
}

#[derive(Debug)]
pub enum Order {
    Move(Tile),
}
