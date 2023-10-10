use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use tile::{events::TileSelectedEvent, map_generation::generate_circle};
use units::{Unit, UnitsPlugin};

mod hex;
mod tile;
mod units;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DefaultPickingPlugins))
        .add_plugins(UnitsPlugin)
        .add_systems(Startup, startup)
        .add_event::<TileSelectedEvent>()
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let tiles = generate_circle(1);
    let unit_tile = tiles.first().unwrap();

    commands.spawn((
        Unit::new(unit_tile.clone()),
        PbrBundle {
            mesh: meshes.add(
                shape::Icosphere {
                    radius: 0.2,
                    ..default()
                }
                .try_into()
                .unwrap(),
            ),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                unlit: true,
                ..default()
            }),
            transform: Transform::from_translation(unit_tile.center()),
            ..default()
        },
    ));

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
