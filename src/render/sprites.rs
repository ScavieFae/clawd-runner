use ratatui::style::Color;

/// Claude brand salmon/peach color
pub const CLAUDE_COLOR: Color = Color::Rgb(217, 119, 87); // #D97757

/// Dark gray for eyes and obstacles
pub const DARK_COLOR: Color = Color::Rgb(45, 45, 42); // #2D2D2A

/// Medium gray for ground
pub const GROUND_COLOR: Color = Color::Rgb(138, 138, 122); // #8A8A7A

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

    pub const WIDTH: u16 = 7;
    pub const HEIGHT: u16 = 3;
}

/// Obstacle types with their sprites
#[derive(Clone, Copy, Debug)]
pub enum ObstacleType {
    Small,
    Tall,
    Double,
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
        }
    }

    pub fn width(&self) -> u16 {
        match self {
            ObstacleType::Small => 3,
            ObstacleType::Tall => 3,
            ObstacleType::Double => 7,
        }
    }

    pub fn height(&self) -> u16 {
        match self {
            ObstacleType::Small => 3,
            ObstacleType::Tall => 4,
            ObstacleType::Double => 3,
        }
    }

    /// Hitbox is slightly smaller than visual for forgiving collision
    pub fn hitbox_width(&self) -> u16 {
        match self {
            ObstacleType::Small => 1,
            ObstacleType::Tall => 1,
            ObstacleType::Double => 5,
        }
    }

    pub fn hitbox_height(&self) -> u16 {
        match self {
            ObstacleType::Small => 2,
            ObstacleType::Tall => 3,
            ObstacleType::Double => 2,
        }
    }
}
