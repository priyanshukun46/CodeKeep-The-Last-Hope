use bevy::prelude::*;

/// Specification for a single wave of hostile incursion.
#[derive(Debug, Clone)]
pub struct WaveDef {
    pub name: &'static str,
    pub enemy_count: usize,
    pub spawn_interval: f32,
    pub enemy_health: f32,
    pub enemy_speed: f32,
    pub enemy_reward: u32,
    pub enemy_damage: f32,
}

/// Global manager orchestrating wave lifecycles, timers, and difficulty scaling.
#[derive(Resource, Debug)]
pub struct WaveManager {
    pub current_wave_idx: usize,
    pub waves: Vec<WaveDef>,
    pub spawn_timer: Timer,
    pub enemies_spawned_this_wave: usize,
    pub intermission_timer: Timer,
    pub is_intermission: bool,
    pub all_waves_completed: bool,
}

impl Default for WaveManager {
    fn default() -> Self {
        let waves = vec![
            WaveDef {
                name: "Wave 1: Incursion Scouts",
                enemy_count: 6,
                spawn_interval: 1.5,
                enemy_health: 45.0,
                enemy_speed: 75.0,
                enemy_reward: 15,
                enemy_damage: 10.0,
            },
            WaveDef {
                name: "Wave 2: Automata Swarm",
                enemy_count: 10,
                spawn_interval: 1.2,
                enemy_health: 70.0,
                enemy_speed: 85.0,
                enemy_reward: 20,
                enemy_damage: 10.0,
            },
            WaveDef {
                name: "Wave 3: Corrupted Skimmers",
                enemy_count: 14,
                spawn_interval: 1.0,
                enemy_health: 100.0,
                enemy_speed: 100.0,
                enemy_reward: 22,
                enemy_damage: 12.0,
            },
            WaveDef {
                name: "Wave 4: Cyber Behemoths",
                enemy_count: 16,
                spawn_interval: 0.9,
                enemy_health: 150.0,
                enemy_speed: 80.0,
                enemy_reward: 28,
                enemy_damage: 15.0,
            },
            WaveDef {
                name: "Wave 5: AI Core Colossus",
                enemy_count: 20,
                spawn_interval: 0.8,
                enemy_health: 220.0,
                enemy_speed: 90.0,
                enemy_reward: 35,
                enemy_damage: 20.0,
            },
        ];

        let first_interval = waves[0].spawn_interval;
        Self {
            current_wave_idx: 0,
            waves,
            spawn_timer: Timer::from_seconds(first_interval, TimerMode::Repeating),
            enemies_spawned_this_wave: 0,
            intermission_timer: Timer::from_seconds(4.0, TimerMode::Once),
            is_intermission: true, // Grace period before first wave begins
            all_waves_completed: false,
        }
    }
}

impl WaveManager {
    pub fn current_wave(&self) -> Option<&WaveDef> {
        self.waves.get(self.current_wave_idx)
    }

    pub fn total_waves(&self) -> usize {
        self.waves.len()
    }

    pub fn advance_wave(&mut self) -> bool {
        if self.current_wave_idx + 1 < self.waves.len() {
            self.current_wave_idx += 1;
            let next_interval = self.waves[self.current_wave_idx].spawn_interval;
            self.spawn_timer = Timer::from_seconds(next_interval, TimerMode::Repeating);
            self.enemies_spawned_this_wave = 0;
            self.intermission_timer = Timer::from_seconds(6.0, TimerMode::Once);
            self.is_intermission = true;
            true
        } else {
            self.all_waves_completed = true;
            false
        }
    }

    pub fn is_spawning_complete(&self) -> bool {
        if let Some(wave) = self.current_wave() {
            self.enemies_spawned_this_wave >= wave.enemy_count
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wave_progression() {
        let mut manager = WaveManager::default();
        assert_eq!(manager.current_wave_idx, 0);
        assert!(manager.is_intermission);

        assert!(!manager.is_spawning_complete());
        manager.enemies_spawned_this_wave = 6;
        assert!(manager.is_spawning_complete());

        assert!(manager.advance_wave());
        assert_eq!(manager.current_wave_idx, 1);
        assert_eq!(manager.enemies_spawned_this_wave, 0);
        assert!(manager.is_intermission);
    }
}
