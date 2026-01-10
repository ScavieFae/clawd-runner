use super::state::{GameState, PlayerState};

/// Gravity constant (pulls velocity down each tick)
const GRAVITY: f32 = 0.6;

/// Initial jump velocity (positive = upward)
const JUMP_VELOCITY: f32 = 5.0;

/// Maximum speed multiplier
const MAX_SPEED: f32 = 2.5;

/// Speed increase per frame
const SPEED_INCREMENT: f32 = 0.0005;

/// Base obstacle scroll speed
const BASE_SCROLL_SPEED: f32 = 3.0;

impl GameState {
    /// Called each frame to update physics
    pub fn tick(&mut self) {
        self.frame_count += 1;

        // Update speed (gradually increases, capped)
        if self.speed < MAX_SPEED {
            self.speed += SPEED_INCREMENT;
        }

        // Update scroll offset for ground animation
        self.scroll_offset = self.scroll_offset.wrapping_add((self.speed * 2.0) as u32);

        // Update player physics
        self.update_player();

        // Move obstacles
        self.update_obstacles();

        // Check collisions
        self.check_collisions();

        // Update score (+1 per frame survived)
        self.score += 1;

        // Decrease collision flash
        if self.collision_flash > 0 {
            self.collision_flash -= 1;
        }
    }

    fn update_player(&mut self) {
        if self.player.state == PlayerState::Jumping {
            // Apply gravity (subtract to pull velocity down)
            self.player.velocity_y -= GRAVITY;
            self.player.y += self.player.velocity_y;

            // Check if landed (y <= 0 means back on ground)
            if self.player.y <= 0.0 {
                self.player.y = 0.0;
                self.player.velocity_y = 0.0;
                self.player.state = PlayerState::Running;
            }
        }
    }

    fn update_obstacles(&mut self) {
        let scroll_speed = BASE_SCROLL_SPEED * self.speed;

        // Move obstacles left
        for obstacle in &mut self.obstacles {
            obstacle.x -= scroll_speed;

            // Check if player passed this obstacle (for bonus scoring)
            if !obstacle.passed && (obstacle.x + obstacle.obstacle_type.width() as f32) < self.player.x {
                obstacle.passed = true;
                self.score += 10; // Bonus for clearing obstacle
            }
        }

        // Remove obstacles that are off-screen
        self.obstacles.retain(|o| o.x > -20.0);
    }

    fn check_collisions(&mut self) {
        let player_left = self.player.x + 1.0; // Slightly inset hitbox
        let player_right = player_left + self.player.hitbox_width();
        let player_bottom = self.player.y;
        let player_top = player_bottom + self.player.hitbox_height();

        for obstacle in &self.obstacles {
            let obs_left = obstacle.x + 1.0; // Slightly inset hitbox
            let obs_right = obs_left + obstacle.obstacle_type.hitbox_width() as f32;
            let obs_bottom = 0.0;
            let obs_top = obs_bottom + obstacle.obstacle_type.hitbox_height() as f32;

            // AABB collision check
            let x_overlap = player_left < obs_right && player_right > obs_left;
            let y_overlap = player_bottom < obs_top && player_top > obs_bottom;

            if x_overlap && y_overlap {
                // Collision! Flash but don't die (spec says no death state)
                self.collision_flash = 10;
                // Could add a small score penalty here if desired
            }
        }
    }

    /// Initiate a jump if on the ground
    pub fn jump(&mut self) {
        if self.player.state == PlayerState::Running && self.player.y <= 0.0 {
            self.player.velocity_y = JUMP_VELOCITY;
            self.player.state = PlayerState::Jumping;
        }
    }
}
