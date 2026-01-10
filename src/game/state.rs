use crate::render::sprites::{ClaudeSprite, ObstacleType};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerState {
    Running,
    Jumping,
}

#[derive(Debug)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub velocity_y: f32,
    pub state: PlayerState,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            x: 8.0, // Fixed position from left edge
            y: 0.0, // Ground level
            velocity_y: 0.0,
            state: PlayerState::Running,
        }
    }
}

impl Player {
    /// Hitbox is slightly smaller than sprite for forgiving collisions
    pub fn hitbox_width(&self) -> f32 {
        ClaudeSprite::WIDTH as f32 - 2.0
    }

    pub fn hitbox_height(&self) -> f32 {
        ClaudeSprite::HEIGHT as f32 - 1.0
    }
}

#[derive(Debug)]
pub struct Obstacle {
    pub x: f32,
    pub obstacle_type: ObstacleType,
    pub passed: bool, // For scoring when player clears it
}

impl Obstacle {
    pub fn new(x: f32, obstacle_type: ObstacleType) -> Self {
        Self {
            x,
            obstacle_type,
            passed: false,
        }
    }
}

#[derive(Debug)]
pub struct GameState {
    pub player: Player,
    pub obstacles: Vec<Obstacle>,
    pub score: u32,
    pub frame_count: u64,
    pub scroll_offset: u32,
    pub speed: f32,
    pub should_quit: bool,
    pub collision_flash: u8, // Frames remaining for collision flash effect
    pub terminal_width: u16,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            player: Player::default(),
            obstacles: Vec::new(),
            score: 0,
            frame_count: 0,
            scroll_offset: 0,
            speed: 1.0,
            should_quit: false,
            collision_flash: 0,
            terminal_width: 80,
        }
    }
}

impl GameState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_terminal_width(mut self, width: u16) -> Self {
        self.terminal_width = width;
        self
    }
}
