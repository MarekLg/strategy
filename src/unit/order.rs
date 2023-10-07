use std::ops::Mul;

use bevy::prelude::*;

use crate::map::tile::Tile;

use super::Unit;

#[derive(Debug)]
pub enum Order {
    Move(Tile),
}

pub fn order_system(mut query: Query<(&mut Unit, &mut Transform)>) {
    for (mut unit, mut transform) in query.iter_mut() {
        if let Some(order) = &unit.order {
            match order {
                Order::Move(tile) => {
                    let target = tile.center();
                    let direction = target - transform.translation;
                    let distance = direction.length();
                    let speed = 0.01;

                    if distance < f32::EPSILON {
                        transform.translation = target;
                        unit.order = None;
                    } else {
                        transform.translation +=
                            direction.normalize().mul(speed).clamp_length_max(distance);
                    }
                }
            }
        }
    }
}
