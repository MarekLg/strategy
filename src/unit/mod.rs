pub mod order;
pub mod unit_bundle;

use bevy::prelude::*;

use crate::map::tile::Tile;

use self::{order::Order, unit_bundle::UnitBundle};

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

pub fn spawn_unit(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    tile: Tile,
) {
    let center = tile.center();

    commands.spawn(UnitBundle {
        unit: Unit::new(tile),
        pbr: PbrBundle {
            mesh: meshes.add(
                shape::Icosphere {
                    radius: 0.2,
                    ..default()
                }
                .try_into()
                .unwrap(),
            ),
            material: materials.add(StandardMaterial {
                base_color: Color::Rgba {
                    red: 0.0,
                    green: 1.0,
                    blue: 0.0,
                    alpha: 1.0,
                },
                ..default()
            }),
            transform: Transform::from_translation(center),
            ..default()
        },
    });
}
