use bevy::prelude::*;
use hex::{edge::Edge, position::Position};
use map::{tile::Tile, Map};
use unit::{
    order::{order_system, Order},
    spawn_unit, Unit,
};

mod hex;
mod map;
mod unit;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(PostStartup, add_order)
        .add_systems(Update, order_system)
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let map = Map::from_circle(1);

    commands.spawn(PbrBundle {
        mesh: meshes.add(map.generate_mesh()),
        ..default()
    });

    spawn_unit(
        &mut commands,
        &mut meshes,
        &mut materials,
        map.tiles().first().unwrap().clone(),
    );

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 10.0, -1.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn add_order(mut query: Query<&mut Unit>) {
    for mut unit in query.iter_mut() {
        unit.order = Some(Order::Move(Tile::new(Position::ZERO.neighbor(&Edge::NE))));
    }
}
