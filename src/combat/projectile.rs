use super::damage::DamageEvent;
use super::effects::HitSpark;
use crate::audio::{PlaySoundEvent, SoundEffect};
use crate::enemy::components::Health;
use bevy::prelude::*;

/// Kinetic/plasma projectile flying toward an enemy.
#[derive(Component, Debug, Clone)]
pub struct Projectile {
    pub damage: f32,
    pub speed: f32,
    pub target: Entity,
}

/// Moves projectiles toward their targets and impacts on arrival.
pub fn projectile_movement_system(
    mut commands: Commands,
    time: Res<Time>,
    mut damage_events: EventWriter<DamageEvent>,
    mut sound_events: EventWriter<PlaySoundEvent>,
    target_query: Query<&Transform, (With<Health>, Without<Projectile>)>,
    mut projectile_query: Query<(Entity, &Projectile, &mut Transform)>,
) {
    let delta_secs = time.delta_secs();

    for (proj_entity, projectile, mut proj_transform) in projectile_query.iter_mut() {
        if let Ok(target_transform) = target_query.get(projectile.target) {
            let current_pos = proj_transform.translation.truncate();
            let target_pos = target_transform.translation.truncate();
            let to_target = target_pos - current_pos;
            let distance = to_target.length();

            if distance < 10.0 {
                // Impact!
                damage_events.send(DamageEvent {
                    target: projectile.target,
                    amount: projectile.damage,
                });
                sound_events.send(PlaySoundEvent(SoundEffect::Impact));

                // Visual plasma impact spark
                commands.spawn(HitSpark {
                    position: current_pos,
                    timer: Timer::from_seconds(0.12, TimerMode::Once),
                });

                commands.entity(proj_entity).despawn();
            } else {
                let direction = to_target.normalize();
                proj_transform.translation +=
                    (direction * projectile.speed * delta_secs).extend(0.0);

                // Rotate projectile towards target
                let angle = direction.y.atan2(direction.x);
                proj_transform.rotation = Quat::from_rotation_z(angle);
            }
        } else {
            // Target was already destroyed or despawned
            commands.entity(proj_entity).despawn();
        }
    }
}
