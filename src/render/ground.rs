use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::Widget,
};
use super::sprites::GROUND_COLOR;

/// Scrolling ground line
pub struct Ground {
    offset: u16,
    pattern_interval: u16,
}

impl Ground {
    pub fn new(offset: u16) -> Self {
        Self {
            offset,
            pattern_interval: 15, // Add texture every N characters
        }
    }

    pub fn with_offset(mut self, offset: u16) -> Self {
        self.offset = offset;
        self
    }
}

impl Widget for Ground {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height == 0 || area.width == 0 {
            return;
        }

        let style = Style::default().fg(GROUND_COLOR);

        for x in 0..area.width {
            // Add texture variation at intervals
            let world_x = x.wrapping_add(self.offset);
            let ch = if world_x % self.pattern_interval == 0 {
                '╦'
            } else {
                '═'
            };
            buf[(area.x + x, area.y)].set_char(ch).set_style(style);
        }
    }
}
