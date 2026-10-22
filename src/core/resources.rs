use bevy::prelude::*;

/// Player's scrap reserves and economy management.
#[derive(Resource, Debug, Clone)]
pub struct PlayerResources {
    pub scrap: u32,
}

impl Default for PlayerResources {
    fn default() -> Self {
        Self {
            scrap: 150, // Starting scrap to afford initial turrets
        }
    }
}

impl PlayerResources {
    #[allow(dead_code)]
    pub fn new(scrap: u32) -> Self {
        Self { scrap }
    }

    pub fn can_afford(&self, cost: u32) -> bool {
        self.scrap >= cost
    }

    pub fn spend(&mut self, cost: u32) -> bool {
        if self.can_afford(cost) {
            self.scrap -= cost;
            true
        } else {
            false
        }
    }

    pub fn add(&mut self, amount: u32) {
        self.scrap = self.scrap.saturating_add(amount);
    }
}

/// The human Refuge that the player is sworn to defend.
#[derive(Resource, Debug, Clone)]
pub struct Refuge {
    pub health: f32,
    pub max_health: f32,
    pub position: Vec2,
}

impl Default for Refuge {
    fn default() -> Self {
        Self {
            health: 100.0,
            max_health: 100.0,
            position: Vec2::new(480.0, 120.0),
        }
    }
}

impl Refuge {
    #[allow(dead_code)]
    pub fn new(max_health: f32, position: Vec2) -> Self {
        Self {
            health: max_health,
            max_health,
            position,
        }
    }

    pub fn take_damage(&mut self, amount: f32) -> bool {
        self.health = (self.health - amount).max(0.0);
        self.is_destroyed()
    }

    pub fn is_destroyed(&self) -> bool {
        self.health <= 0.0
    }

    pub fn health_fraction(&self) -> f32 {
        if self.max_health > 0.0 {
            (self.health / self.max_health).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }
}

/// Marker component for the physical Refuge Core entity in the world.
#[derive(Component)]
pub struct RefugeBase;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_resources_economy() {
        let mut resources = PlayerResources::new(100);
        assert!(resources.can_afford(50));
        assert!(!resources.can_afford(150));

        assert!(resources.spend(60));
        assert_eq!(resources.scrap, 40);

        assert!(!resources.spend(50));
        assert_eq!(resources.scrap, 40);

        resources.add(80);
        assert_eq!(resources.scrap, 120);
    }

    #[test]
    fn test_refuge_damage_and_destruction() {
        let mut refuge = Refuge::new(100.0, Vec2::ZERO);
        assert_eq!(refuge.health_fraction(), 1.0);
        assert!(!refuge.is_destroyed());

        let destroyed = refuge.take_damage(40.0);
        assert!(!destroyed);
        assert_eq!(refuge.health, 60.0);
        assert_eq!(refuge.health_fraction(), 0.6);

        let destroyed = refuge.take_damage(70.0);
        assert!(destroyed);
        assert_eq!(refuge.health, 0.0);
        assert!(refuge.is_destroyed());
    }
}
