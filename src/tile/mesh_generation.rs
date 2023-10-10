use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

use crate::hex::corner::CORNERS;

use super::Tile;

impl Tile {
    pub fn generate_mesh(&self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        let mut positions = vec![self.center()];

        for corner in CORNERS {
            positions.push(self.corner(&corner));
        }

        let normals = vec![Vec3::Y; 7];

        let indices = vec![
            0, 1, 2, //
            0, 2, 3, //
            0, 3, 4, //
            0, 4, 5, //
            0, 5, 6, //
            0, 6, 1, //
        ];

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }
}
