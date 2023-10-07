use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

use crate::hex::corner::CORNERS;

use super::Map;

impl Map {
    pub fn generate_mesh(&self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut indices = Vec::new();

        for (index, tile) in self.tiles.iter().enumerate() {
            positions.push(tile.center());
            for corner in CORNERS {
                positions.push(tile.corner(&corner));
            }

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

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }
}
