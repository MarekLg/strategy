mod bundle;
mod events;
mod systems;
mod unit;

use bevy::prelude::*;

pub use self::{
    bundle::UnitBundle,
    events::UnitMovedEvent,
    unit::{Order, Unit},
};

use self::systems::{add_move_order_on_tile_selected, order_system};

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UnitMovedEvent>()
            .add_systems(Update, (order_system, add_move_order_on_tile_selected));
    }
}
