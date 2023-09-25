pub mod unit_bundle;

use bevy::prelude::*;

use crate::hex::position::Position;

use self::unit_bundle::UnitBundle;

#[derive(Component)]
pub struct Unit {
    pub position: Position,
}

pub fn spawn_unit(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Position,
) {
    let center = position.center();

    commands.spawn(UnitBundle {
        unit: Unit { position },
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
            transform: Transform::from_xyz(center.x, 0.0, center.y),
            ..default()
        },
    });
}
