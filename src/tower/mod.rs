pub mod components;
pub mod placement;

use bevy::prelude::*;
use crate::core::state::GameState;
#[allow(unused_imports)]
pub use components::{AttackCooldown, CurrentTarget, PlacementGhost, Tower, TowerRange};
pub use placement::CursorWorldCoords;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorWorldCoords>()
            .add_systems(Startup, placement::setup_placement_ghost)
            .add_systems(PreUpdate, placement::update_cursor_world_coords_system)
            .add_systems(
                Update,
                (
                    placement::tower_placement_preview_system,
                    placement::tower_placement_input_system,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
