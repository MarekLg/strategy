use bevy::prelude::*;

#[derive(Event)]
pub struct UnitMovedEvent {
    pub unit_entity: Entity,
    pub tile_entity: Entity,
}
