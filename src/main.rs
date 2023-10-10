use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use tile::{events::TileSelectedEvent, map_generation::generate_circle};
use unit::{
    order::{add_tile_move_order, order_system},
    spawn_unit,
};

mod hex;
mod tile;
mod unit;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_systems(Startup, startup)
        .add_systems(Update, (order_system, add_tile_move_order))
        .add_event::<TileSelectedEvent>()
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let tiles = generate_circle(1);

    spawn_unit(
        &mut commands,
        &mut meshes,
        &mut materials,
        tiles.first().unwrap().clone(),
    );

    for tile in tiles {
        tile.spawn(&mut commands, &mut meshes, &mut materials);
    }

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 10.0, -1.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        RaycastPickCamera::default(),
    ));
}
