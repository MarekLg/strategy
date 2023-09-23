use bevy::prelude::*;
use hex::position::Position;
use map::generate_hex_mesh;

mod hex;
mod map;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_hex)
        .run();
}

fn spawn_hex(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(generate_hex_mesh(&Position::ZERO)),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, -1.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
