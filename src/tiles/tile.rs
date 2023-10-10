use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use rand::Rng;

use super::{Corner, Position, TileSelectedEvent};

#[derive(Debug, Clone, Copy, Component)]
pub struct Tile {
    position: Position,
}

impl Tile {
    pub fn new(position: Position) -> Self {
        Self { position }
    }

    pub fn spawn(
        self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        let mut rng = rand::thread_rng();

        commands.spawn((
            self,
            PbrBundle {
                mesh: meshes.add(self.generate_mesh()),
                material: materials.add(StandardMaterial {
                    base_color: Color::Hsla {
                        hue: rng.gen_range(0.0..360.0),
                        saturation: 0.6,
                        lightness: 0.5,
                        alpha: 1.0,
                    },
                    unlit: true,
                    ..default()
                }),
                ..default()
            },
            PickableBundle::default(),
            RaycastPickTarget::default(),
            On::<Pointer<Click>>::run(send_tile_selected_event),
        ));
    }

    pub fn center(&self) -> Vec3 {
        let offset = self.position.offset();

        Vec3 {
            x: offset.x,
            y: 0.0,
            z: offset.y,
        }
    }

    pub fn corner(&self, corner: &Corner) -> Vec3 {
        let offset = self.position.offset_corner(corner);

        Vec3 {
            x: offset.x,
            y: 0.0,
            z: offset.y,
        }
    }
}

fn send_tile_selected_event(
    event: Listener<Pointer<Click>>,
    mut tile_selected: EventWriter<TileSelectedEvent>,
) {
    tile_selected.send(TileSelectedEvent {
        tile_entity: event.target,
    });
}
