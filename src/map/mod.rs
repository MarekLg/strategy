pub mod generation;
pub mod mesh;
pub mod tile;

use self::tile::Tile;

pub struct Map {
    tiles: Vec<Tile>,
}

impl Map {
    pub fn tiles(&self) -> &Vec<Tile> {
        &self.tiles
    }
}
