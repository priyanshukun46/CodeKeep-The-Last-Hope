pub mod hud;
pub mod menu;
pub mod pause;

use crate::core::state::GameState;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (hud::setup_hud, menu::setup_main_menu))
            .add_systems(OnEnter(GameState::MainMenu), menu::setup_main_menu)
            .add_systems(
                Update,
                (
                    menu::main_menu_input_system.run_if(in_state(GameState::MainMenu)),
                    pause::pause_toggle_system,
                    hud::sync_hud_visibility_system,
                    hud::update_hud_system.run_if(in_state(GameState::Playing)),
                    hud::restart_game_system,
                ),
            )
            .add_systems(OnEnter(GameState::Paused), pause::setup_pause_screen)
            .add_systems(OnExit(GameState::Paused), pause::cleanup_pause_screen)
            .add_systems(OnEnter(GameState::GameOver), hud::setup_game_over_screen)
            .add_systems(OnEnter(GameState::Victory), hud::setup_victory_screen);
    }
}
