use bevy::prelude::*;

#[derive(Event)]
pub struct TileSelectedEvent {
    pub tile_entity: Entity,
}
