use super::state::{GameState, Obstacle};
use crate::render::sprites::ObstacleType;

/// Minimum frames between obstacles
const MIN_SPAWN_INTERVAL: u64 = 60;

/// Maximum frames between obstacles
const MAX_SPAWN_INTERVAL: u64 = 120;

/// Score thresholds for introducing new obstacle types
const TALL_THRESHOLD: u32 = 500;
const DOUBLE_THRESHOLD: u32 = 1000;
const FLYING_THRESHOLD: u32 = 1500;

impl GameState {
    /// Possibly spawn a new obstacle
    pub fn maybe_spawn_obstacle(&mut self) {
        // Don't spawn if there are obstacles too close to the right edge
        let spawn_x = self.terminal_width as f32 + 10.0;

        if let Some(last) = self.obstacles.last() {
            // Ensure minimum distance between obstacles
            let min_distance = 30.0 * self.speed;
            if spawn_x - last.x < min_distance {
                return;
            }
        }

        // Random spawn check based on frame count
        // Use a simple pseudo-random based on frame count and score
        let spawn_seed = self.frame_count.wrapping_mul(31337).wrapping_add(self.score as u64);
        let spawn_interval = MIN_SPAWN_INTERVAL + (spawn_seed % (MAX_SPAWN_INTERVAL - MIN_SPAWN_INTERVAL));

        if self.frame_count % spawn_interval == 0 {
            let obstacle_type = self.pick_obstacle_type(spawn_seed);
            self.obstacles.push(Obstacle::new(spawn_x, obstacle_type));
        }
    }

    /// Pick obstacle type based on score (gradual introduction)
    fn pick_obstacle_type(&self, seed: u64) -> ObstacleType {
        let type_seed = seed.wrapping_mul(7919);

        // Build available types based on score
        if self.score < TALL_THRESHOLD {
            // Early game: only small obstacles
            ObstacleType::Small
        } else if self.score < DOUBLE_THRESHOLD {
            // Mid game: small + tall
            match type_seed % 10 {
                0..=5 => ObstacleType::Small,  // 60% small
                _ => ObstacleType::Tall,       // 40% tall
            }
        } else if self.score < FLYING_THRESHOLD {
            // Later game: small + tall + double
            match type_seed % 10 {
                0..=4 => ObstacleType::Small,  // 50% small
                5..=7 => ObstacleType::Tall,   // 30% tall
                _ => ObstacleType::Double,     // 20% double
            }
        } else {
            // Full game: all types including flying
            match type_seed % 10 {
                0..=3 => ObstacleType::Small,  // 40% small
                4..=5 => ObstacleType::Tall,   // 20% tall
                6..=7 => ObstacleType::Double, // 20% double
                _ => ObstacleType::Flying,     // 20% flying
            }
        }
    }
}
