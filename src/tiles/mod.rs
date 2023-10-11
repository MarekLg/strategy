use bevy::prelude::*;

use self::tile::clear_occupant_on_moved;
pub use self::{
    events::TileSelectedEvent,
    hex::{
        corner::{Corner, CORNERS},
        edge::{Edge, EDGES},
        position::Position,
    },
    map_generation::generate_circle,
    tile::Tile,
};

mod events;
mod hex;
mod map_generation;
mod mesh_generation;
mod tile;

pub struct TilesPlugin;

impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TileSelectedEvent>()
            .add_systems(Update, clear_occupant_on_moved);
    }
}
