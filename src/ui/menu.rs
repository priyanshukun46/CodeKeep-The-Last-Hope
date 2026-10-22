use crate::audio::{PlaySoundEvent, SoundEffect};
use crate::core::state::GameState;
use bevy::prelude::*;

#[derive(Component)]
pub struct MainMenuScreen;

/// Spawns the main title screen and briefing overlay.
pub fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn((
            MainMenuScreen,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.0),
                padding: UiRect::all(Val::Px(32.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.04, 0.05, 0.08, 0.95)),
        ))
        .with_children(|root| {
            // Title Header
            root.spawn((
                Text::new("CODEKEEP"),
                TextFont {
                    font_size: 54.0,
                    ..default()
                },
                TextColor(Color::srgb(0.2, 0.85, 1.0)),
            ));

            root.spawn((
                Text::new("— AN ISOMETRIC CITY-RESTORATION SANDBOX —"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.95, 0.75, 0.2)),
            ));

            // Narrative Briefing Box
            root.spawn((
                Node {
                    max_width: Val::Px(640.0),
                    padding: UiRect::all(Val::Px(16.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    margin: UiRect::axes(Val::Px(0.0), Val::Px(12.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.08, 0.1, 0.15, 0.7)),
                BorderColor(Color::srgba(0.2, 0.4, 0.6, 0.4)),
            ))
            .with_children(|briefing| {
                briefing.spawn((
                    Text::new(
                        "You awaken amidst the silent ruins of a long-abandoned metropolis.\n\
                         No directives or prescribed paths remain.\n\
                         Survey decaying city sectors, inspect foundations, and engineer automated systems to rebuild.",
                    ),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.75, 0.8, 0.85)),
                ));
            });

            // Start Prompt Button Box
            root.spawn((
                Node {
                    padding: UiRect::axes(Val::Px(28.0), Val::Px(14.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    margin: UiRect::axes(Val::Px(0.0), Val::Px(8.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.1, 0.25, 0.4, 0.8)),
                BorderColor(Color::srgb(0.3, 0.85, 1.0)),
            ))
            .with_children(|btn| {
                btn.spawn((
                    Text::new("[ PRESS SPACE OR ENTER TO ENTER THE RUINS ]"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::srgb(1.0, 1.0, 1.0)),
                ));
            });

            // Controls Cheat-sheet
            root.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(4.0),
                    margin: UiRect::top(Val::Px(10.0)),
                    ..default()
                },
            ))
            .with_children(|hints| {
                hints.spawn((
                    Text::new("CONTROLS"),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.5, 0.6, 0.7)),
                ));
                hints.spawn((
                    Text::new("[LMB] Inspect / Select Isometric Tile  |  [P / ESC] Pause  |  [R] Reset"),
                    TextFont {
                        font_size: 13.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.65, 0.7, 0.75)),
                ));
            });
        });
}

/// Cleans up all main menu UI elements when leaving the MainMenu state.
pub fn cleanup_main_menu(mut commands: Commands, menu_query: Query<Entity, With<MainMenuScreen>>) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Listens for start keys to transition from MainMenu to Playing.
pub fn main_menu_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut sound_events: EventWriter<PlaySoundEvent>,
) {
    if keyboard.just_pressed(KeyCode::Space)
        || keyboard.just_pressed(KeyCode::Enter)
        || mouse_button.just_pressed(MouseButton::Left)
    {
        sound_events.send(PlaySoundEvent(SoundEffect::BuildTurret));
        next_state.set(GameState::Playing);
    }
}
