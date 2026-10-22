use bevy::prelude::*;

/// High-level game states for screen and lifecycle transitions.
#[derive(States, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    Paused,
    GameOver,
    Victory,
}
