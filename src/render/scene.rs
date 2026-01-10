use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::Widget,
};
use crate::game::state::{GameState, PlayerState};
use super::sprites::{ClaudeSprite, CLAUDE_COLOR, DARK_COLOR};
use super::ground::Ground;

/// The complete game scene widget
pub struct GameScene<'a> {
    game: &'a GameState,
}

impl<'a> GameScene<'a> {
    pub fn new(game: &'a GameState) -> Self {
        Self { game }
    }

    fn render_player(&self, area: Rect, buf: &mut Buffer) {
        let sprite = match self.game.player.state {
            PlayerState::Jumping => ClaudeSprite::BOOSTING,
            PlayerState::Running => {
                // Animate tentacles while floating
                if self.game.frame_count % 16 < 8 {
                    ClaudeSprite::FLOATING_1
                } else {
                    ClaudeSprite::FLOATING_2
                }
            }
        };

        let style = Style::default().fg(CLAUDE_COLOR);
        let player_x = self.game.player.x as u16;
        let ground_y = area.height.saturating_sub(2); // Ground is 1 row, status is 1 row
        let player_bottom = ground_y;
        let player_y = player_bottom.saturating_sub(ClaudeSprite::HEIGHT).saturating_sub(self.game.player.y as u16);

        for (row_idx, line) in sprite.iter().enumerate() {
            let y = player_y + row_idx as u16;
            if y >= area.height {
                continue;
            }
            for (col_idx, ch) in line.chars().enumerate() {
                let x = player_x + col_idx as u16;
                if x < area.width && ch != ' ' {
                    buf[(area.x + x, area.y + y)].set_char(ch).set_style(style);
                }
            }
        }
    }

    fn render_obstacles(&self, area: Rect, buf: &mut Buffer) {
        let style = Style::default().fg(DARK_COLOR);
        let ground_y = area.height.saturating_sub(2);

        for obstacle in &self.game.obstacles {
            let sprite = obstacle.obstacle_type.sprite();
            let height = obstacle.obstacle_type.height();
            let obs_bottom = ground_y;
            let obs_top = obs_bottom.saturating_sub(height);

            for (row_idx, line) in sprite.iter().enumerate() {
                let y = obs_top + row_idx as u16;
                if y >= area.height {
                    continue;
                }
                for (col_idx, ch) in line.chars().enumerate() {
                    let x = obstacle.x as i32 + col_idx as i32;
                    if x >= 0 && (x as u16) < area.width && ch != ' ' {
                        buf[(area.x + x as u16, area.y + y)].set_char(ch).set_style(style);
                    }
                }
            }
        }
    }

    fn render_status_bar(&self, area: Rect, buf: &mut Buffer) {
        if area.height == 0 {
            return;
        }

        let y = area.height - 1;

        // Left side: "compacting..." with animated dots
        let dots = match (self.game.frame_count / 15) % 4 {
            0 => ".",
            1 => "..",
            2 => "...",
            _ => "",
        };
        let left_text = format!("compacting{}", dots);
        for (i, ch) in left_text.chars().enumerate() {
            if (i as u16) < area.width {
                buf[(area.x + i as u16, area.y + y)].set_char(ch);
            }
        }

        // Right side: score
        let score_text = format!("score: {}", self.game.score);
        let score_start = area.width.saturating_sub(score_text.len() as u16);
        for (i, ch) in score_text.chars().enumerate() {
            let x = score_start + i as u16;
            if x < area.width {
                buf[(area.x + x, area.y + y)].set_char(ch);
            }
        }
    }
}

impl Widget for GameScene<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height < 6 || area.width < 40 {
            // Terminal too small
            let msg = "Terminal too small!";
            let x = area.width.saturating_sub(msg.len() as u16) / 2;
            for (i, ch) in msg.chars().enumerate() {
                let pos_x = area.x + x + i as u16;
                if pos_x < area.x + area.width {
                    buf[(pos_x, area.y)].set_char(ch);
                }
            }
            return;
        }

        // Ground is second-to-last row
        let ground_y = area.height.saturating_sub(2);
        let ground_area = Rect::new(area.x, area.y + ground_y, area.width, 1);
        Ground::new(self.game.scroll_offset as u16).render(ground_area, buf);

        // Render obstacles
        self.render_obstacles(area, buf);

        // Render player
        self.render_player(area, buf);

        // Status bar is last row
        self.render_status_bar(area, buf);
    }
}
