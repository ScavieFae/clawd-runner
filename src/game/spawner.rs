use super::state::{GameState, Obstacle};
use crate::render::sprites::ObstacleType;

/// Minimum frames between obstacles
const MIN_SPAWN_INTERVAL: u64 = 60;

/// Maximum frames between obstacles
const MAX_SPAWN_INTERVAL: u64 = 120;

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
            // Pick obstacle type pseudo-randomly
            let type_seed = spawn_seed.wrapping_mul(7919);
            let obstacle_type = match type_seed % 10 {
                0..=4 => ObstacleType::Small,  // 50% small
                5..=7 => ObstacleType::Tall,   // 30% tall
                _ => ObstacleType::Double,     // 20% double
            };

            self.obstacles.push(Obstacle::new(spawn_x, obstacle_type));
        }
    }
}
