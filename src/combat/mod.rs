pub mod damage;
pub mod projectile;

use bevy::prelude::*;
use crate::core::resources::Refuge;
use crate::core::state::GameState;
use crate::enemy::components::{Enemy, Health, PathFollower};
use crate::enemy::wave::WaveManager;
use crate::tower::components::{AttackCooldown, CurrentTarget, Tower, TowerRange};
pub use damage::DamageEvent;
pub use projectile::Projectile;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEvent>()
            .add_systems(
                Update,
                (
                    tower_targeting_system,
                    tower_attack_system,
                    projectile::projectile_movement_system,
                    damage::damage_application_system,
                    damage::enemy_death_system,
                    combat_state_check_system,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

/// Identifies the highest priority hostile target within each tower's engagement radius.
/// Priority is given to enemies furthest along the path (closest to the Refuge).
fn tower_targeting_system(
    mut tower_query: Query<(&Transform, &TowerRange, &mut CurrentTarget), With<Tower>>,
    enemy_query: Query<(Entity, &Transform, &PathFollower), (With<Enemy>, With<Health>)>,
) {
    for (tower_transform, range, mut current_target) in tower_query.iter_mut() {
        let tower_pos = tower_transform.translation.truncate();

        let mut best_target: Option<(Entity, usize, f32)> = None;

        for (enemy_entity, enemy_transform, follower) in enemy_query.iter() {
            let enemy_pos = enemy_transform.translation.truncate();
            let dist = tower_pos.distance(enemy_pos);

            if dist <= range.0 {
                // Prioritize higher waypoint index, then shorter distance to tower
                let is_better = match best_target {
                    None => true,
                    Some((_, best_idx, best_dist)) => {
                        follower.current_waypoint_idx > best_idx
                            || (follower.current_waypoint_idx == best_idx && dist < best_dist)
                    }
                };

                if is_better {
                    best_target = Some((enemy_entity, follower.current_waypoint_idx, dist));
                }
            }
        }

        current_target.0 = best_target.map(|(e, _, _)| e);
    }
}

/// Fires projectiles at active targets when attack cooldown timer expires.
fn tower_attack_system(
    mut commands: Commands,
    time: Res<Time>,
    mut tower_query: Query<(
        &mut Transform,
        &Tower,
        &TowerRange,
        &mut AttackCooldown,
        &CurrentTarget,
    )>,
    target_query: Query<&Transform, (With<Enemy>, With<Health>, Without<Tower>)>,
) {
    for (mut tower_transform, tower, range, mut cooldown, target) in tower_query.iter_mut() {
        cooldown.0.tick(time.delta());

        let Some(target_entity) = target.0 else {
            continue;
        };

        let Ok(target_transform) = target_query.get(target_entity) else {
            continue;
        };

        let tower_pos = tower_transform.translation.truncate();
        let target_pos = target_transform.translation.truncate();
        let to_target = target_pos - tower_pos;

        // Orient turret barrel toward target
        let angle = to_target.y.atan2(to_target.x);
        tower_transform.rotation = Quat::from_rotation_z(angle);

        // Verify still in range
        if to_target.length() > range.0 {
            continue;
        }

        // Fire projectile
        if cooldown.0.just_finished() {
            commands.spawn((
                Projectile {
                    damage: tower.damage,
                    speed: 380.0,
                    target: target_entity,
                },
                Sprite {
                    color: Color::srgb(1.0, 0.9, 0.2), // High-energy plasma yellow
                    custom_size: Some(Vec2::new(8.0, 8.0)),
                    ..default()
                },
                Transform::from_xyz(tower_pos.x, tower_pos.y, 3.0),
            ));
        }
    }
}

/// Checks winning/losing conditions and transitions GameState.
fn combat_state_check_system(
    refuge: Res<Refuge>,
    wave_manager: Res<WaveManager>,
    enemy_query: Query<(), With<Enemy>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // 1. Defeat condition: Refuge breached and destroyed
    if refuge.is_destroyed() {
        next_state.set(GameState::GameOver);
        return;
    }

    // 2. Victory condition: All waves completed and all surviving enemies eliminated
    if wave_manager.all_waves_completed && enemy_query.is_empty() {
        next_state.set(GameState::Victory);
    }
}
