use bevy::prelude::*;
use crate::core::resources::{PlayerResources, Refuge};
use crate::enemy::wave::WaveManager;

#[derive(Component)]
pub struct RefugeHealthText;

#[derive(Component)]
pub struct RefugeHealthBarFill;

#[derive(Component)]
pub struct ScrapText;

#[derive(Component)]
pub struct WaveInfoText;

/// Spawns the HUD overlay including health bar, scrap counter, and wave status.
pub fn setup_hud(mut commands: Commands) {
    // Root container covering the viewport
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(16.0)),
                ..default()
            },
            // Transparent container so world clicks pass through
        ))
        .with_children(|root| {
            // Top HUD Bar
            root.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(56.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    padding: UiRect::axes(Val::Px(20.0), Val::Px(8.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.06, 0.08, 0.12, 0.9)),
                BorderColor(Color::srgba(0.2, 0.35, 0.5, 0.5)),
            ))
            .with_children(|top_bar| {
                // 1. Refuge Health Section
                top_bar
                    .spawn(Node {
                        flex_direction: FlexDirection::Column,
                        width: Val::Px(240.0),
                        row_gap: Val::Px(4.0),
                        ..default()
                    })
                    .with_children(|hp_box| {
                        hp_box.spawn((
                            RefugeHealthText,
                            Text::new("REFUGE INTEGRITY: 100%"),
                            TextFont {
                                font_size: 13.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.3, 0.85, 1.0)),
                        ));

                        // Health bar outline
                        hp_box
                            .spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Px(12.0),
                                    border: UiRect::all(Val::Px(1.0)),
                                    ..default()
                                },
                                BackgroundColor(Color::srgba(0.15, 0.05, 0.05, 0.8)),
                                BorderColor(Color::srgba(0.4, 0.5, 0.6, 0.6)),
                            ))
                            .with_children(|bar_bg| {
                                bar_bg.spawn((
                                    RefugeHealthBarFill,
                                    Node {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(100.0),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.1, 0.8, 0.9)),
                                ));
                            });
                    });

                // 2. Wave Status (Center)
                top_bar.spawn((
                    WaveInfoText,
                    Text::new("WAVE 1 / 5: Incursion Scouts"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.95, 0.8, 0.3)),
                ));

                // 3. Scrap Reserves (Right)
                top_bar.spawn((
                    ScrapText,
                    Text::new("SCRAP RESERVES: 150"),
                    TextFont {
                        font_size: 15.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.4, 0.95, 0.5)),
                ));
            });

            // Bottom Controls Hint Bar
            root.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(36.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.04, 0.06, 0.08, 0.8)),
            ))
            .with_children(|bottom_bar| {
                bottom_bar.spawn((
                    Text::new(
                        "[LMB] Build Pulse Turret (Cost: 50 Scrap | Range: 160px | Dmg: 25)",
                    ),
                    TextFont {
                        font_size: 13.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.7, 0.75, 0.8)),
                ));
            });
        });
}

/// Updates HUD text elements and health bar widths each frame.
pub fn update_hud_system(
    refuge: Res<Refuge>,
    resources: Res<PlayerResources>,
    wave_manager: Res<WaveManager>,
    mut hp_text_query: Query<&mut Text, (With<RefugeHealthText>, Without<ScrapText>, Without<WaveInfoText>)>,
    mut hp_bar_query: Query<&mut Node, With<RefugeHealthBarFill>>,
    mut scrap_text_query: Query<&mut Text, (With<ScrapText>, Without<RefugeHealthText>, Without<WaveInfoText>)>,
    mut wave_text_query: Query<&mut Text, (With<WaveInfoText>, Without<RefugeHealthText>, Without<ScrapText>)>,
) {
    // 1. Update Refuge Health
    let fraction = refuge.health_fraction();
    if let Ok(mut text) = hp_text_query.get_single_mut() {
        **text = format!(
            "REFUGE INTEGRITY: {:.0} / {:.0} ({:.0}%)",
            refuge.health,
            refuge.max_health,
            fraction * 100.0
        );
    }
    if let Ok(mut node) = hp_bar_query.get_single_mut() {
        node.width = Val::Percent(fraction * 100.0);
    }

    // 2. Update Scrap Reserves
    if let Ok(mut text) = scrap_text_query.get_single_mut() {
        **text = format!("SCRAP RESERVES: {}", resources.scrap);
    }

    // 3. Update Wave Info
    if let Ok(mut text) = wave_text_query.get_single_mut() {
        if wave_manager.all_waves_completed {
            **text = "ALL HOSTILE WAVES ELIMINATED".to_string();
        } else if wave_manager.is_intermission {
            let remaining = wave_manager.intermission_timer.remaining_secs();
            let wave_num = wave_manager.current_wave_idx + 1;
            **text = format!("PREPARE DEFENSES: Wave {} in {:.1}s", wave_num, remaining);
        } else if let Some(wave) = wave_manager.current_wave() {
            let wave_num = wave_manager.current_wave_idx + 1;
            let total = wave_manager.total_waves();
            **text = format!(
                "{} ({}/{}) — Spawned: {}/{}",
                wave.name, wave_num, total, wave_manager.enemies_spawned_this_wave, wave.enemy_count
            );
        }
    }
}

/// Spawns the Game Over screen overlay when the refuge falls.
pub fn setup_game_over_screen(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(16.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.12, 0.02, 0.03, 0.92)),
        ))
        .with_children(|panel| {
            panel.spawn((
                Text::new("REFUGE BREACHED — HUMANITY HAS FALLEN"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::srgb(0.95, 0.15, 0.15)),
            ));
            panel.spawn((
                Text::new("The last beacon of hope was overrun by hostile forces."),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.7, 0.7)),
            ));
        });
}

/// Spawns the Victory screen overlay when all waves are repelled.
pub fn setup_victory_screen(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(16.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.02, 0.1, 0.08, 0.92)),
        ))
        .with_children(|panel| {
            panel.spawn((
                Text::new("REFUGE SECURED — THE FLAME ENDURES"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::srgb(0.2, 0.95, 0.5)),
            ));
            panel.spawn((
                Text::new("All hostile waves neutralized. The seeds of humanity's rebirth take root."),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.85, 0.8)),
            ));
        });
}
