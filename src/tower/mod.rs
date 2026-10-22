pub mod components;
pub mod placement;

use crate::core::state::GameState;
use bevy::prelude::*;
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
                OnEnter(GameState::MainMenu),
                placement::hide_placement_ghost,
            )
            .add_systems(OnEnter(GameState::Paused), placement::hide_placement_ghost)
            .add_systems(
                OnEnter(GameState::GameOver),
                placement::hide_placement_ghost,
            )
            .add_systems(OnEnter(GameState::Victory), placement::hide_placement_ghost)
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
