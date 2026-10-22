use bevy::prelude::*;

/// Defensive turret built by the player to eliminate hostile threats.
#[derive(Component, Debug, Clone)]
pub struct Tower {
    #[allow(dead_code)]
    pub cost: u32,
    pub damage: f32,
}

impl Default for Tower {
    fn default() -> Self {
        Self {
            cost: 50,
            damage: 25.0,
        }
    }
}

/// Attack acquisition and engagement radius.
#[derive(Component, Debug, Clone)]
pub struct TowerRange(pub f32);

impl Default for TowerRange {
    fn default() -> Self {
        Self(160.0)
    }
}

/// Attack cadence timer governing shots per second.
#[derive(Component, Debug, Clone)]
pub struct AttackCooldown(pub Timer);

impl Default for AttackCooldown {
    fn default() -> Self {
        let mut timer = Timer::from_seconds(0.75, TimerMode::Once);
        timer.set_elapsed(std::time::Duration::from_secs_f32(0.75));
        Self(timer)
    }
}

/// The currently locked enemy target.
#[derive(Component, Debug, Clone, Default)]
pub struct CurrentTarget(pub Option<Entity>);

/// Visual silhouette following the mouse cursor to preview placement validity.
#[derive(Component)]
pub struct PlacementGhost;
