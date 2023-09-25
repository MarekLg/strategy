use crate::hex::{
    edge::{Edge, EDGES},
    position::Position,
};

use super::{tile::Tile, Map};

impl Map {
    pub fn from_circle(radius: u16) -> Self {
        let mut tiles = Vec::new();

        let center = Position::ZERO;
        tiles.push(Tile::new(center));

        for i in 1..radius + 1 {
            let mut position = &center.neighbor(&Edge::W) * i;

            for edge in EDGES {
                for _ in 0..i {
                    tiles.push(Tile::new(position));

                    position = position.neighbor(&edge);
                }
            }
        }

        Self { tiles }
    }
}
