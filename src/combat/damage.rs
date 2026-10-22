use crate::core::resources::PlayerResources;
use crate::enemy::components::{Enemy, Health};
use bevy::prelude::*;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_damage_event_and_death_reward() {
        let mut app = App::new();
        app.add_event::<DamageEvent>();
        app.insert_resource(PlayerResources::new(50));
        app.add_systems(
            Update,
            (damage_application_system, enemy_death_system).chain(),
        );

        let enemy = app
            .world_mut()
            .spawn((
                Enemy {
                    speed: 80.0,
                    scrap_reward: 25,
                    damage: 10.0,
                },
                Health::new(40.0),
            ))
            .id();

        // 1. Apply non-lethal damage
        app.world_mut().send_event(DamageEvent {
            target: enemy,
            amount: 20.0,
        });
        app.update();

        assert_eq!(app.world().get::<Health>(enemy).unwrap().current, 20.0);
        assert_eq!(app.world().resource::<PlayerResources>().scrap, 50);

        // 2. Apply lethal damage
        app.world_mut().send_event(DamageEvent {
            target: enemy,
            amount: 25.0,
        });
        app.update();

        // Enemy should be despawned and scrap rewarded (50 + 25 = 75)
        assert!(app.world().get::<Health>(enemy).is_none());
        assert_eq!(app.world().resource::<PlayerResources>().scrap, 75);
    }
}
