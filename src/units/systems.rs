use bevy::prelude::*;

use crate::tiles::{Tile, TileSelectedEvent};

use super::{Order, Unit};

pub fn order_system(mut query: Query<(&mut Unit, &mut Transform)>) {
    for (mut unit, mut transform) in query.iter_mut() {
        if let Some(order) = &unit.order {
            match order {
                Order::Move(tile) => {
                    let target = tile.center();
                    let direction = target - transform.translation;
                    let distance = direction.length();
                    let speed = 0.02;

                    if distance < f32::EPSILON {
                        transform.translation = target;
                        unit.order = None;
                    } else {
                        transform.translation +=
                            (direction.normalize() * speed).clamp_length_max(distance);
                    }
                }
            }
        }
    }
}

pub fn add_move_order_on_tile_selected(
    mut tile_selected: EventReader<TileSelectedEvent>,
    mut units: Query<&mut Unit>,
    tiles: Query<&Tile>,
) {
    for event in tile_selected.iter() {
        for mut unit in units.iter_mut() {
            if unit.order.is_none() {
                unit.order = Some(Order::Move(*tiles.get(event.tile_entity).unwrap()));
            }
        }
    }
}
