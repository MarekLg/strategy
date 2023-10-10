use bevy::prelude::*;

use crate::{
    selection::Selection,
    tiles::{Tile, TileSelectedEvent},
};

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
    selection: Res<Selection>,
    mut units: Query<&mut Unit>,
    tiles: Query<&Tile>,
) {
    if let Some(selected_entity) = selection.entity {
        if let Ok(mut unit) = units.get_mut(selected_entity) {
            if unit.order.is_none() {
                for event in tile_selected.iter() {
                    unit.order = Some(Order::Move(*tiles.get(event.tile_entity).unwrap()));
                }
            }
        }
    }
}
