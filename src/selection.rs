use bevy::prelude::*;
use bevy_mod_picking::{
    prelude::{Click, Listener, On, Pointer, RaycastPickTarget},
    PickableBundle,
};

#[derive(Resource)]
pub struct Selection {
    pub entity: Option<Entity>,
}

impl Default for Selection {
    fn default() -> Self {
        Self { entity: None }
    }
}

#[derive(Component)]
pub struct Selectable;

#[derive(Bundle)]
pub struct SelectableBundle {
    selectable: Selectable,
    pickable: PickableBundle,
    raycast_pick_target: RaycastPickTarget,
    select: On<Pointer<Click>>,
}

impl Default for SelectableBundle {
    fn default() -> Self {
        Self {
            selectable: Selectable,
            pickable: default(),
            raycast_pick_target: default(),
            select: On::<Pointer<Click>>::run(change_selection),
        }
    }
}

fn change_selection(event: Listener<Pointer<Click>>, mut selection: ResMut<Selection>) {
    selection.entity = Some(event.target);
}
