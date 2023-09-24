use bevy::{
    prelude::{Mesh, Vec3},
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

use crate::hex::{
    corner::CORNERS,
    edge::{Edge, EDGES},
    position::Position,
};

pub struct Map {
    tiles: Vec<Position>,
}

impl Map {
    pub fn from_circle(radius: u16) -> Self {
        let mut tiles = Vec::new();

        let center = Position::ZERO;
        tiles.push(center);

        for i in 1..radius + 1 {
            let mut position = &center.neighbor(&Edge::W) * i;

            for edge in EDGES {
                for _ in 0..i {
                    tiles.push(position);

                    position = position.neighbor(&edge);
                }
            }
        }

        Self { tiles }
    }

    pub fn generate_mesh(&self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut indices = Vec::new();

        for (index, tile) in self.tiles.iter().enumerate() {
            positions.push(tile.center());
            positions.append(&mut CORNERS.map(|corner| tile.corner(corner)).to_vec());

            normals.append(&mut (0..7).map(|_| Vec3::Y).collect::<Vec<_>>());

            indices.append(
                &mut vec![
                    0, 1, 2, //
                    0, 2, 3, //
                    0, 3, 4, //
                    0, 4, 5, //
                    0, 5, 6, //
                    0, 6, 1, //
                ]
                .iter()
                .map(|i| (i + index * 7) as u32)
                .collect(),
            );
        }

        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            positions
                .iter()
                .map(|position| Vec3::new(position.x, 0.0, position.y))
                .collect::<Vec<_>>(),
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }
}
