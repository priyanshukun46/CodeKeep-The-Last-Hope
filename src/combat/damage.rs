use bevy::prelude::*;
use crate::core::resources::PlayerResources;
use crate::enemy::components::{Enemy, Health};

/// Event emitted when an attack strikes an entity.
#[derive(Event, Debug, Clone)]
pub struct DamageEvent {
    pub target: Entity,
    pub amount: f32,
}

/// Applies queued damage events to target entities with Health components.
pub fn damage_application_system(
    mut events: EventReader<DamageEvent>,
    mut health_query: Query<&mut Health>,
) {
    for event in events.read() {
        if let Ok(mut health) = health_query.get_mut(event.target) {
            health.take_damage(event.amount);
        }
    }
}

/// Detects entities whose health has reached zero, grants scrap bounties, and despawns them.
pub fn enemy_death_system(
    mut commands: Commands,
    mut resources: ResMut<PlayerResources>,
    query: Query<(Entity, &Health, Option<&Enemy>)>,
) {
    for (entity, health, enemy_opt) in query.iter() {
        if health.is_dead() {
            if let Some(enemy) = enemy_opt {
                resources.add(enemy.scrap_reward);
            }
            commands.entity(entity).despawn();
        }
    }
}
