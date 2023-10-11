use bevy::prelude::*;

use crate::selection::SelectableBundle;

use super::Unit;

#[derive(Bundle)]
pub struct UnitBundle {
    unit: Unit,
    pbr: PbrBundle,
    selectable: SelectableBundle,
}

impl Default for UnitBundle {
    fn default() -> Self {
        Self {
            unit: Unit::new(),
            ..default()
        }
    }
}
