use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::time::Duration;
use std::io::Write;

/// Actions that can result from input
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameAction {
    Jump,
    Quit,
    None,
}

/// Poll for keyboard input with a timeout
/// Returns None if no event, Some(action) otherwise
pub fn poll_input(timeout: Duration) -> std::io::Result<Option<GameAction>> {
    if event::poll(timeout)? {
        if let Event::Key(key) = event::read()? {
            // Debug: log key events to file
            if let Ok(mut f) = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open("/tmp/clawd-keys.log")
            {
                let _ = writeln!(f, "Key: {:?}, kind: {:?}, modifiers: {:?}", key.code, key.kind, key.modifiers);
            }

            // Only respond to key press, not release or repeat
            if key.kind == KeyEventKind::Press {
                return Ok(Some(handle_key(key)));
            }
        }
    }
    Ok(None)
}

fn handle_key(key: KeyEvent) -> GameAction {
    match key.code {
        // Jump keys
        KeyCode::Char(' ') => GameAction::Jump,
        KeyCode::Up => GameAction::Jump,
        KeyCode::Char('w') => GameAction::Jump,

        // Quit keys
        KeyCode::Char('q') => GameAction::Quit,
        KeyCode::Esc => GameAction::Quit,
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => GameAction::Quit,

        _ => GameAction::None,
    }
}
