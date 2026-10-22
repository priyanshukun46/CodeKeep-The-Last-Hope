use super::components::{AttackCooldown, CurrentTarget, PlacementGhost, Tower, TowerRange};
use crate::audio::{PlaySoundEvent, SoundEffect};
use crate::core::resources::{PlayerResources, Refuge};
use crate::map::grid::{OccupiedCells, can_build, grid_to_world, world_to_grid};
use crate::map::path::MapPath;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub const TURRET_BUILD_COST: u32 = 50;

/// Stores the current cursor position projected onto the 2D game world.
#[derive(Resource, Default, Debug)]
pub struct CursorWorldCoords(pub Option<Vec2>);

/// Spawns the translucent holographic placement preview sprite.
pub fn setup_placement_ghost(mut commands: Commands) {
    commands.spawn((
        PlacementGhost,
        Sprite {
            color: Color::srgba(0.2, 0.8, 0.4, 0.5),
            custom_size: Some(Vec2::new(28.0, 28.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 5.0),
        Visibility::Hidden,
    ));
}

/// Hides the placement ghost when leaving the active playing state.
pub fn hide_placement_ghost(mut ghost_query: Query<&mut Visibility, With<PlacementGhost>>) {
    for mut vis in ghost_query.iter_mut() {
        *vis = Visibility::Hidden;
    }
}

/// Converts screen cursor coordinates into 2D world space coordinates.
pub fn update_cursor_world_coords_system(
    mut cursor_coords: ResMut<CursorWorldCoords>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    let Ok(window) = window_query.get_single() else {
        cursor_coords.0 = None;
        return;
    };

    let Ok((camera, camera_transform)) = camera_query.get_single() else {
        cursor_coords.0 = None;
        return;
    };

    if let Some(cursor_pos) = window.cursor_position() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            cursor_coords.0 = Some(world_pos);
            return;
        }
    }

    cursor_coords.0 = None;
}

/// Updates the placement ghost position, validity colors, and range preview ring.
pub fn tower_placement_preview_system(
    cursor_coords: Res<CursorWorldCoords>,
    occupied: Res<OccupiedCells>,
    path: Res<MapPath>,
    refuge: Res<Refuge>,
    resources: Res<PlayerResources>,
    mut gizmos: Gizmos,
    mut ghost_query: Query<(&mut Transform, &mut Sprite, &mut Visibility), With<PlacementGhost>>,
) {
    let Ok((mut ghost_transform, mut ghost_sprite, mut ghost_vis)) = ghost_query.get_single_mut()
    else {
        return;
    };

    let Some(raw_world_pos) = cursor_coords.0 else {
        *ghost_vis = Visibility::Hidden;
        return;
    };

    *ghost_vis = Visibility::Visible;
    let grid_coord = world_to_grid(raw_world_pos);
    let snapped_pos = grid_to_world(grid_coord);

    let can_place = can_build(grid_coord, snapped_pos, &occupied, &path, refuge.position);
    let can_afford = resources.can_afford(TURRET_BUILD_COST);
    let is_valid = can_place && can_afford;

    ghost_transform.translation = snapped_pos.extend(5.0);

    let (color, ring_color) = if is_valid {
        (
            Color::srgba(0.2, 0.8, 0.4, 0.55),
            Color::srgba(0.2, 0.9, 0.4, 0.6),
        )
    } else {
        (
            Color::srgba(0.9, 0.2, 0.2, 0.55),
            Color::srgba(0.9, 0.2, 0.2, 0.6),
        )
    };

    ghost_sprite.color = color;

    // Draw range preview ring
    gizmos.circle_2d(snapped_pos, 160.0, ring_color);
}

/// Handles player left-click input to construct defensive turrets on valid ground.
pub fn tower_placement_input_system(
    mut commands: Commands,
    mouse_button: Res<ButtonInput<MouseButton>>,
    cursor_coords: Res<CursorWorldCoords>,
    mut occupied: ResMut<OccupiedCells>,
    path: Res<MapPath>,
    refuge: Res<Refuge>,
    mut resources: ResMut<PlayerResources>,
    mut sound_events: EventWriter<PlaySoundEvent>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    let Some(raw_world_pos) = cursor_coords.0 else {
        return;
    };

    let grid_coord = world_to_grid(raw_world_pos);
    let snapped_pos = grid_to_world(grid_coord);

    if !can_build(grid_coord, snapped_pos, &occupied, &path, refuge.position) {
        return;
    }

    if !resources.spend(TURRET_BUILD_COST) {
        return;
    }

    occupied.occupy(grid_coord);
    sound_events.send(PlaySoundEvent(SoundEffect::BuildTurret));

    // Spawn Tower base and barrel
    commands.spawn((
        Tower {
            cost: TURRET_BUILD_COST,
            damage: 25.0,
        },
        TowerRange(160.0),
        AttackCooldown::default(),
        CurrentTarget::default(),
        Sprite {
            color: Color::srgb(0.2, 0.6, 0.95), // High-tech cobalt blue
            custom_size: Some(Vec2::new(28.0, 28.0)),
            ..default()
        },
        Transform::from_xyz(snapped_pos.x, snapped_pos.y, 2.0),
    ));
}
