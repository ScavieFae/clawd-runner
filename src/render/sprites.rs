use ratatui::style::Color;

/// Claude brand salmon/peach color
/// Using 256-color indexed palette for compatibility (209 ≈ salmon)
pub const CLAUDE_COLOR: Color = Color::Indexed(209);

/// Bright obstacle color (visible on dark backgrounds)
/// 256-color grayscale 250 ≈ light gray
pub const OBSTACLE_COLOR: Color = Color::Indexed(250);

/// Flash color for collision effect
pub const COLLISION_COLOR: Color = Color::Indexed(196); // Bright red

/// Medium gray for ground
/// 256-color grayscale 245 ≈ medium gray
pub const GROUND_COLOR: Color = Color::Indexed(245);

/// Clawd sprite frames - the Claude Code mascot
/// 7 chars wide, 3 lines tall
pub struct ClaudeSprite;

impl ClaudeSprite {
    /// Running frame 1 - feet together
    pub const FLOATING_1: &'static [&'static str] = &[
        "▗█▀█▀█▖",
        " █▅█▅█ ",
        "  ▀ ▀  ",
    ];

    /// Running frame 2 - feet apart
    pub const FLOATING_2: &'static [&'static str] = &[
        "▗█▀█▀█▖",
        " █▅█▅█ ",
        " ▀   ▀ ",
    ];

    /// Jumping - feet tucked/hidden
    pub const BOOSTING: &'static [&'static str] = &[
        "▗█▀█▀█▖",
        " █▅█▅█ ",
        "       ",
    ];

    /// Ducking frame 1 - feet out (positions 2 and 6)
    pub const DUCKING_1: &'static [&'static str] = &[
        "▗█▀█▀█▖",
        " ▀▔▔▔▀ ",
    ];

    /// Ducking frame 2 - feet in (positions 3 and 5)
    pub const DUCKING_2: &'static [&'static str] = &[
        "▗█▀█▀█▖",
        " ▔▀▔▀▔ ",
    ];

    /// Landing squash - feet tucked (2 lines)
    pub const LANDING: &'static [&'static str] = &[
        "▗█▀█▀█▖",
        " █▅█▅█ ",
    ];

    pub const WIDTH: u16 = 7;
    pub const HEIGHT: u16 = 3;
    pub const DUCK_HEIGHT: u16 = 2;
    pub const LANDING_HEIGHT: u16 = 2;
}

/// Obstacle types with their sprites
#[derive(Clone, Copy, Debug)]
pub enum ObstacleType {
    Small,
    Tall,
    Double,
    Flying,
}

impl ObstacleType {
    pub fn sprite(&self) -> &'static [&'static str] {
        match self {
            ObstacleType::Small => &[
                " █ ",
                "███",
                " █ ",
            ],
            ObstacleType::Tall => &[
                " █ ",
                "███",
                " █ ",
                " █ ",
            ],
            ObstacleType::Double => &[
                " █   █ ",
                "███ ███",
                " █   █ ",
            ],
            ObstacleType::Flying => &[
                "\\█/",
                " █ ",
            ],
        }
    }

    pub fn width(&self) -> u16 {
        match self {
            ObstacleType::Small => 3,
            ObstacleType::Tall => 3,
            ObstacleType::Double => 7,
            ObstacleType::Flying => 3,
        }
    }

    pub fn height(&self) -> u16 {
        match self {
            ObstacleType::Small => 3,
            ObstacleType::Tall => 4,
            ObstacleType::Double => 3,
            ObstacleType::Flying => 2,
        }
    }

    /// Hitbox is slightly smaller than visual for forgiving collision
    pub fn hitbox_width(&self) -> u16 {
        match self {
            ObstacleType::Small => 1,
            ObstacleType::Tall => 1,
            ObstacleType::Double => 5,
            ObstacleType::Flying => 1,
        }
    }

    pub fn hitbox_height(&self) -> u16 {
        match self {
            ObstacleType::Small => 2,
            ObstacleType::Tall => 3,
            ObstacleType::Double => 2,
            ObstacleType::Flying => 1,
        }
    }

    /// Returns true if this obstacle flies above ground
    pub fn is_flying(&self) -> bool {
        matches!(self, ObstacleType::Flying)
    }

    /// Height above ground for flying obstacles
    pub fn fly_height(&self) -> u16 {
        match self {
            ObstacleType::Flying => 2, // Positioned so ducking clears it
            _ => 0,
        }
    }
}
