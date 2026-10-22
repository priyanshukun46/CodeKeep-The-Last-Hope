pub mod components;
pub mod wave;

use bevy::prelude::*;
use crate::core::resources::Refuge;
use crate::core::state::GameState;
use crate::map::path::MapPath;
pub use components::{Enemy, Health, PathFollower};
#[allow(unused_imports)]
pub use wave::{WaveDef, WaveManager};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WaveManager>()
            .add_systems(
                Update,
                (
                    wave_spawner_system,
                    enemy_movement_system,
                    draw_enemy_health_gizmos,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

/// Spawns waves of hostile enemies at the map incursion portal.
fn wave_spawner_system(
    mut commands: Commands,
    time: Res<Time>,
    mut wave_manager: ResMut<WaveManager>,
    path: Res<MapPath>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    if wave_manager.all_waves_completed {
        return;
    }

    // 1. Intermission between waves
    if wave_manager.is_intermission {
        wave_manager.intermission_timer.tick(time.delta());
        if wave_manager.intermission_timer.just_finished() {
            wave_manager.is_intermission = false;
        }
        return;
    }

    // 2. Check wave completion: all spawned and all defeated
    let living_enemy_count = enemy_query.iter().count();
    if wave_manager.is_spawning_complete() && living_enemy_count == 0 {
        wave_manager.advance_wave();
        return;
    }

    // 3. Spawn enemies according to the wave cadence
    wave_manager.spawn_timer.tick(time.delta());
    if wave_manager.spawn_timer.just_finished() && !wave_manager.is_spawning_complete() {
        if let Some(wave) = wave_manager.current_wave().cloned() {
            let spawn_pos = path.spawn_point();

            commands.spawn((
                Enemy {
                    speed: wave.enemy_speed,
                    scrap_reward: wave.enemy_reward,
                    damage: wave.enemy_damage,
                },
                PathFollower {
                    current_waypoint_idx: 1, // Head toward the second waypoint
                },
                Health::new(wave.enemy_health),
                Sprite {
                    color: Color::srgb(0.9, 0.25, 0.2), // Hostile crimson
                    custom_size: Some(Vec2::new(20.0, 20.0)),
                    ..default()
                },
                Transform::from_xyz(spawn_pos.x, spawn_pos.y, 2.0),
            ));

            wave_manager.enemies_spawned_this_wave += 1;
        }
    }
}

/// Moves hostile enemies along the waypoint path toward the refuge.
fn enemy_movement_system(
    mut commands: Commands,
    time: Res<Time>,
    path: Res<MapPath>,
    mut refuge: ResMut<Refuge>,
    mut enemy_query: Query<(Entity, &Enemy, &mut PathFollower, &mut Transform)>,
) {
    let delta_secs = time.delta_secs();

    for (entity, enemy, mut follower, mut transform) in enemy_query.iter_mut() {
        if let Some(&target_wp) = path.waypoints.get(follower.current_waypoint_idx) {
            let current_pos = transform.translation.truncate();
            let to_target = target_wp - current_pos;
            let distance = to_target.length();

            if distance < 5.0 {
                // Reached current waypoint, advance to next
                follower.current_waypoint_idx += 1;
                if follower.current_waypoint_idx >= path.waypoints.len() {
                    // Creep breached the human Refuge!
                    refuge.take_damage(enemy.damage);
                    commands.entity(entity).despawn();
                }
            } else {
                let direction = to_target.normalize();
                transform.translation += (direction * enemy.speed * delta_secs).extend(0.0);

                // Rotate facing direction of travel
                let angle = direction.y.atan2(direction.x);
                transform.rotation = Quat::from_rotation_z(angle);
            }
        } else {
            // End of path fallback
            commands.entity(entity).despawn();
        }
    }
}

/// Draws floating health bars above active enemy entities.
fn draw_enemy_health_gizmos(mut gizmos: Gizmos, query: Query<(&Transform, &Health), With<Enemy>>) {
    for (transform, health) in query.iter() {
        let pos = transform.translation.truncate() + Vec2::new(0.0, 16.0);
        let bar_width = 24.0;
        let bar_height = 4.0;

        // Background (empty health)
        gizmos.rect_2d(
            pos,
            Vec2::new(bar_width, bar_height),
            Color::srgba(0.2, 0.0, 0.0, 0.8),
        );

        // Foreground (remaining health)
        let fill_fraction = health.fraction();
        let fill_width = bar_width * fill_fraction;
        let fill_pos = pos + Vec2::new(-bar_width * 0.5 + fill_width * 0.5, 0.0);

        let color = if fill_fraction > 0.5 {
            Color::srgb(0.2, 0.9, 0.3) // Healthy green
        } else {
            Color::srgb(0.9, 0.7, 0.1) // Damaged amber
        };

        gizmos.rect_2d(fill_pos, Vec2::new(fill_width, bar_height), color);
    }
}
