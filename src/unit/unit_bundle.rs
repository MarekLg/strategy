use bevy::prelude::*;

use super::Unit;

#[derive(Bundle)]
pub struct UnitBundle {
    pub unit: Unit,
    pub pbr: PbrBundle,
}
