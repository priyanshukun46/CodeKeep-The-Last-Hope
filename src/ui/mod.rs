pub mod hud;

use bevy::prelude::*;
use crate::core::state::GameState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, hud::setup_hud)
            .add_systems(Update, hud::update_hud_system.run_if(in_state(GameState::Playing)))
            .add_systems(OnEnter(GameState::GameOver), hud::setup_game_over_screen)
            .add_systems(OnEnter(GameState::Victory), hud::setup_victory_screen);
    }
}
