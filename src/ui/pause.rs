use crate::audio::{PlaySoundEvent, SoundEffect};
use crate::core::state::GameState;
use bevy::prelude::*;

#[derive(Component)]
pub struct PauseScreen;

/// Listens for pause/resume input toggles during gameplay and pause states.
pub fn pause_toggle_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut sound_events: EventWriter<PlaySoundEvent>,
) {
    if keyboard.just_pressed(KeyCode::KeyP) || keyboard.just_pressed(KeyCode::Escape) {
        match *state.get() {
            GameState::Playing => {
                sound_events.send(PlaySoundEvent(SoundEffect::BuildTurret));
                next_state.set(GameState::Paused);
            }
            GameState::Paused => {
                sound_events.send(PlaySoundEvent(SoundEffect::BuildTurret));
                next_state.set(GameState::Playing);
            }
            _ => {}
        }
    }
}

/// Spawns the translucent tactical pause overlay.
pub fn setup_pause_screen(mut commands: Commands) {
    commands
        .spawn((
            PauseScreen,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(16.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.04, 0.06, 0.1, 0.85)),
        ))
        .with_children(|root| {
            root.spawn((
                Text::new("TACTICAL PAUSE"),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::srgb(0.3, 0.8, 1.0)),
            ));

            root.spawn((
                Text::new("Defense grid frozen. All automated systems on standby."),
                TextFont {
                    font_size: 15.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.75, 0.8)),
            ));

            // Controls Box
            root.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(8.0),
                    margin: UiRect::top(Val::Px(12.0)),
                    padding: UiRect::axes(Val::Px(24.0), Val::Px(12.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.1, 0.14, 0.2, 0.6)),
                BorderColor(Color::srgba(0.3, 0.5, 0.7, 0.4)),
            ))
            .with_children(|box_node| {
                box_node.spawn((
                    Text::new("[P / ESC] Resume Defense"),
                    TextFont {
                        font_size: 15.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.4, 0.9, 0.5)),
                ));
                box_node.spawn((
                    Text::new("[R] Restart Mission"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.85, 0.75, 0.3)),
                ));
            });
        });
}

/// Cleans up the pause overlay upon resuming gameplay.
pub fn cleanup_pause_screen(mut commands: Commands, query: Query<Entity, With<PauseScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
