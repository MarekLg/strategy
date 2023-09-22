use bevy::prelude::*;
use hex::{direction::Direction, position::Position};

mod hex;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_hex)
        .run();
}

fn spawn_hex(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let start = Position::zero();

    for position in [
        Direction::NE,
        Direction::E,
        Direction::SE,
        Direction::SW,
        Direction::W,
        Direction::NW,
    ]
    .map(|direction| start.neighbor(direction))
    {
        let offset = position.offset();

        commands.spawn(PbrBundle {
            mesh: meshes.add(
                (shape::Icosphere {
                    radius: 0.1,
                    ..default()
                })
                .try_into()
                .unwrap(),
            ),
            transform: Transform::from_xyz(offset.x, 0.0, offset.y),
            ..default()
        });
    }

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
