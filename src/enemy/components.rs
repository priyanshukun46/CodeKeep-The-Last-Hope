use bevy::prelude::*;

/// Base component identifying hostile units marching toward the human Refuge.
#[derive(Component, Debug, Clone)]
pub struct Enemy {
    pub speed: f32,
    pub scrap_reward: u32,
    pub damage: f32,
}

/// Tracks the enemy's navigation progress along the predefined map waypoints.
#[derive(Component, Debug, Clone)]
pub struct PathFollower {
    pub current_waypoint_idx: usize,
}

/// Dynamic health points for combat entities (both enemies and structures).
#[derive(Component, Debug, Clone)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }

    /// Deducts damage and returns whether this damage was lethal.
    pub fn take_damage(&mut self, amount: f32) -> bool {
        self.current = (self.current - amount).max(0.0);
        self.is_dead()
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }

    pub fn fraction(&self) -> f32 {
        if self.max > 0.0 {
            (self.current / self.max).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enemy_health_rules() {
        let mut hp = Health::new(50.0);
        assert_eq!(hp.fraction(), 1.0);
        assert!(!hp.is_dead());

        let lethal = hp.take_damage(20.0);
        assert!(!lethal);
        assert_eq!(hp.current, 30.0);
        assert_eq!(hp.fraction(), 0.6);

        let lethal = hp.take_damage(40.0);
        assert!(lethal);
        assert_eq!(hp.current, 0.0);
        assert!(hp.is_dead());
    }
}
