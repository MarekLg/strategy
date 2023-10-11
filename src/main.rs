use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use selection::{SelectableBundle, Selection};
use tiles::{generate_circle, TilesPlugin};
use units::{Unit, UnitsPlugin};

mod selection;
mod tiles;
mod units;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            DefaultPickingPlugins
                .build()
                .disable::<DebugPickingPlugin>()
                .disable::<DefaultHighlightingPlugin>(),
        ))
        .init_resource::<Selection>()
        .add_plugins((TilesPlugin, UnitsPlugin))
        .add_systems(Startup, startup)
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let tiles = generate_circle(1);
    let unit_tile = tiles.first().unwrap();
    let unit_tile2 = tiles.last().unwrap();

    spawn_unit(
        &mut commands,
        &mut meshes,
        &mut materials,
        unit_tile.center(),
    );
    spawn_unit(
        &mut commands,
        &mut meshes,
        &mut materials,
        unit_tile2.center(),
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

fn spawn_unit(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    translation: Vec3,
) {
    commands.spawn((
        Unit::new(),
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
            transform: Transform::from_translation(translation),
            ..default()
        },
        SelectableBundle::default(),
    ));
}
