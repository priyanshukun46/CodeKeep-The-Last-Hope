mod combat;
mod core;
mod enemy;
mod map;
mod tower;
mod ui;

use bevy::prelude::*;
use combat::CombatPlugin;
use core::resources::{PlayerResources, Refuge};
use core::state::GameState;
use enemy::EnemyPlugin;
use map::MapPlugin;
use tower::TowerPlugin;
use ui::UiPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.05, 0.06, 0.08)))
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "CodeKeep — The Last Hope [Prototype]".to_string(),
                    resolution: (1280.0_f32, 720.0_f32).into(),
                    resizable: true,
                    ..default()
                }),
                ..default()
            }),
        )
        .init_state::<GameState>()
        .init_resource::<PlayerResources>()
        .init_resource::<Refuge>()
        .add_plugins((
            MapPlugin,
            EnemyPlugin,
            TowerPlugin,
            CombatPlugin,
            UiPlugin,
        ))
        .add_systems(Startup, setup_camera)
        .run();
}

/// Spawns the central 2D orthographic camera.
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
