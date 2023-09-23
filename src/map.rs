use bevy::{
    prelude::Mesh,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

use crate::hex::{corner::CORNERS, position::Position};

pub fn generate_hex_mesh(position: &Position) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let mut offsets = vec![position.offset()];
    for corner in CORNERS {
        offsets.push(position.corner_offset(corner))
    }

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        offsets
            .iter()
            .map(|offset| [offset.x, 0.0, offset.y])
            .collect::<Vec<_>>(),
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        (0..7).map(|_| [0.0, 1.0, 0.0]).collect::<Vec<[f32; 3]>>(),
    );

    mesh.set_indices(Some(Indices::U32(vec![
        0, 1, 2, //
        0, 2, 3, //
        0, 3, 4, //
        0, 4, 5, //
        0, 5, 6, //
        0, 6, 1, //
    ])));

    mesh
}
