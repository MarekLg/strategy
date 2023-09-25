use bevy::prelude::*;
use hex::position::Position;
use map::Map;
use unit::spawn_unit;

mod hex;
mod map;
mod unit;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_hex)
        .run();
}

fn spawn_hex(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let map = Map::from_circle(1);

    commands.spawn(PbrBundle {
        mesh: meshes.add(map.generate_mesh()),
        ..default()
    });

    spawn_unit(&mut commands, &mut meshes, &mut materials, Position::ZERO);

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 10.0, -1.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
