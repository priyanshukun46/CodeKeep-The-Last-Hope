pub mod grid;
pub mod path;

use crate::core::resources::{Refuge, RefugeBase};
use bevy::prelude::*;
use grid::OccupiedCells;
use path::MapPath;

/// Controls whether the visual presentation focuses on the open-ended Ruined City Sandbox
/// or displays the legacy tower-defense road markers.
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MapDisplayMode {
    #[default]
    CitySandbox,
    LegacyDefense,
}

/// Marker component for legacy tower-defense visual terrain entities.
#[derive(Component)]
pub struct LegacyBattlefieldVisual;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapPath>()
            .init_resource::<OccupiedCells>()
            .init_resource::<MapDisplayMode>()
            .add_systems(Startup, setup_map)
            .add_systems(Update, (draw_map_gizmos, sync_legacy_visuals_visibility));
    }
}

/// Spawns the static map entities, tagging legacy defense visual primitives so they can be separated.
fn setup_map(
    mut commands: Commands,
    refuge: Res<Refuge>,
    path: Res<MapPath>,
    display_mode: Res<MapDisplayMode>,
) {
    let initial_vis = if *display_mode == MapDisplayMode::CitySandbox {
        Visibility::Hidden
    } else {
        Visibility::Visible
    };

    // 1. Spawn Portal (Alien / Hostile Incursion rift)
    let spawn_pos = path.spawn_point();
    commands.spawn((
        LegacyBattlefieldVisual,
        Sprite {
            color: Color::srgb(0.85, 0.2, 0.3), // Crimson
            custom_size: Some(Vec2::new(38.0, 38.0)),
            ..default()
        },
        Transform::from_xyz(spawn_pos.x, spawn_pos.y, 1.0),
        initial_vis,
    ));

    // 2. The Human Refuge Core (Beacon of Hope)
    commands.spawn((
        RefugeBase,
        LegacyBattlefieldVisual,
        Sprite {
            color: Color::srgb(0.15, 0.7, 0.95), // Hopeful Cyan
            custom_size: Some(Vec2::new(48.0, 48.0)),
            ..default()
        },
        Transform::from_xyz(refuge.position.x, refuge.position.y, 1.0),
        initial_vis,
    ));

    // 3. Decorative subtle path road tiles between waypoints
    for window in path.waypoints.windows(2) {
        let (start, end) = (window[0], window[1]);
        let center = (start + end) * 0.5;
        let delta = end - start;
        let length = delta.length();
        let angle = delta.y.atan2(delta.x);

        commands.spawn((
            LegacyBattlefieldVisual,
            Sprite {
                color: Color::srgb(0.12, 0.14, 0.18), // Dark lane pavement
                custom_size: Some(Vec2::new(length + 30.0, 32.0)),
                ..default()
            },
            Transform::from_xyz(center.x, center.y, 0.2)
                .with_rotation(Quat::from_rotation_z(angle)),
            initial_vis,
        ));
    }
}

/// Synchronizes the visibility of legacy battlefield visuals with the current display mode.
fn sync_legacy_visuals_visibility(
    display_mode: Res<MapDisplayMode>,
    mut query: Query<&mut Visibility, With<LegacyBattlefieldVisual>>,
) {
    if !display_mode.is_changed() {
        return;
    }
    let target_vis = if *display_mode == MapDisplayMode::CitySandbox {
        Visibility::Hidden
    } else {
        Visibility::Visible
    };
    for mut vis in query.iter_mut() {
        *vis = target_vis;
    }
}

/// Renders legacy glowing path boundary lines and portal indicators only when in LegacyDefense mode.
fn draw_map_gizmos(
    mut gizmos: Gizmos,
    path: Res<MapPath>,
    refuge: Res<Refuge>,
    display_mode: Res<MapDisplayMode>,
) {
    if *display_mode != MapDisplayMode::LegacyDefense {
        return;
    }

    // Draw lane center markings
    for window in path.waypoints.windows(2) {
        let (start, end) = (window[0], window[1]);
        gizmos.line_2d(start, end, Color::srgba(0.3, 0.4, 0.5, 0.6));
    }

    // Draw waypoints
    for &wp in &path.waypoints {
        gizmos.circle_2d(wp, 4.0, Color::srgba(0.4, 0.6, 0.8, 0.5));
    }

    // Spawn portal pulse ring
    gizmos.circle_2d(path.spawn_point(), 24.0, Color::srgba(0.9, 0.25, 0.35, 0.7));

    // Refuge defense perimeter
    gizmos.circle_2d(refuge.position, 34.0, Color::srgba(0.2, 0.8, 1.0, 0.8));
}
