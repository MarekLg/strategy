mod systems;
mod unit;

pub use unit::{Order, Unit};

use bevy::prelude::*;

use self::systems::{add_move_order_on_tile_selected, order_system};

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (order_system, add_move_order_on_tile_selected));
    }
}
